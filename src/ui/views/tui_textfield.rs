use rustyline::completion::{Candidate, Completer, FilenameCompleter, Pair};
use rustyline::line_buffer;

use line_buffer::LineBuffer;
use termion::event::{Event, Key};
use tui::layout::Rect;
use tui::widgets::Clear;
use unicode_width::UnicodeWidthStr;

use crate::context::AppContext;
use crate::event::AppEvent;
use crate::ui::views::TuiView;
use crate::ui::widgets::{TuiMenu, TuiMultilineText};
use crate::ui::{JoshutoTerminal, TuiBackend};
use crate::util::input;

struct CompletionTracker {
    pub index: usize,
    pub pos: usize,
    pub _original: String,
    pub candidates: Vec<Pair>,
}

impl CompletionTracker {
    pub fn new(pos: usize, candidates: Vec<Pair>, _original: String) -> Self {
        CompletionTracker {
            index: 0,
            pos,
            _original,
            candidates,
        }
    }
}

pub struct CursorInfo {
    pub x: usize,
    pub y: usize,
}

pub struct TuiTextField<'a> {
    _prompt: &'a str,
    _prefix: &'a str,
    _suffix: &'a str,
    _menu_items: Vec<&'a str>,
}

impl<'a> TuiTextField<'a> {
    pub fn menu_items<I>(&mut self, items: I) -> &mut Self
    where
        I: Iterator<Item = &'a str>,
    {
        self._menu_items = items.collect();
        self
    }

    pub fn prompt(&mut self, prompt: &'a str) -> &mut Self {
        self._prompt = prompt;
        self
    }

    pub fn prefix(&mut self, prefix: &'a str) -> &mut Self {
        self._prefix = prefix;
        self
    }

    pub fn suffix(&mut self, suffix: &'a str) -> &mut Self {
        self._suffix = suffix;
        self
    }

    pub fn get_input(
        &'a mut self,
        backend: &'a mut TuiBackend,
        context: &'a mut AppContext,
    ) -> GetInput<'a> {
        let mut line_buffer = line_buffer::LineBuffer::with_capacity(255);
        let completer = FilenameCompleter::new();

        let completion_tracker: Option<CompletionTracker> = None;

        let char_idx = self._prefix.chars().map(|c| c.len_utf8()).sum();

        line_buffer.insert_str(0, self._suffix);
        line_buffer.insert_str(0, self._prefix);
        line_buffer.set_pos(char_idx);

        let terminal = backend.terminal_mut();
        let _ = terminal.show_cursor();

        let curr_history_index = context.commandline_context_ref().history_ref().len();

        GetInput {
            tui_text_field: self,
            context,
            completion_tracker,
            line_buffer,
            completer,
            terminal,
            curr_history_index,
        }
    }
}

impl<'a> std::default::Default for TuiTextField<'a> {
    fn default() -> Self {
        Self {
            _prompt: "",
            _prefix: "",
            _suffix: "",
            _menu_items: vec![],
        }
    }
}

pub struct GetInput<'a> {
    tui_text_field: &'a mut TuiTextField<'a>,
    terminal: &'a mut JoshutoTerminal,
    context: &'a mut AppContext,

    line_buffer: LineBuffer,
    completion_tracker: Option<CompletionTracker>,
    completer: FilenameCompleter,
    curr_history_index: usize,
}

pub enum GetInputState {
    Running(String),
    Finished(String),
    Stopped,
}

impl<'a> GetInput<'a> {
    pub fn next(&mut self) -> GetInputState {
        let GetInput {
            tui_text_field,
            completer,
            terminal,
            ..
        } = self;

        let line_buffer_str = self.line_buffer.as_str();
        let line_buffer_pos = self.line_buffer.pos();
        let mut view = TuiView::new(self.context);
        terminal
            .draw(move |frame| {
                let area: Rect = frame.size();
                if area.height == 0 {
                    return;
                }
                {
                    view.show_bottom_status = false;
                    frame.render_widget(view, area);
                }

                let area_width = area.width as usize;
                let buffer_str = line_buffer_str;
                let cursor_xpos = line_buffer_pos;

                let line_str = format!("{}{}", tui_text_field._prompt, buffer_str);
                let multiline = TuiMultilineText::new(line_str.as_str(), area_width);
                let multiline_height = multiline.height();

                // render menu
                {
                    let menu_widget = TuiMenu::new(tui_text_field._menu_items.as_slice());
                    let menu_len = menu_widget.len();
                    let menu_y = if menu_len + 1 > area.height as usize {
                        0
                    } else {
                        (area.height as usize - menu_len - 1) as u16
                    };

                    let menu_rect = Rect {
                        x: 0,
                        y: menu_y - multiline_height as u16,
                        width: area.width,
                        height: menu_len as u16 + 1,
                    };
                    frame.render_widget(Clear, menu_rect);
                    frame.render_widget(menu_widget, menu_rect);
                }

                let multiline_rect = Rect {
                    x: 0,
                    y: area.height - multiline_height as u16,
                    width: area.width,
                    height: multiline_height as u16,
                };
                let mut cursor_info = CursorInfo {
                    x: 0,
                    y: area.height as usize,
                };

                // get cursor render position
                let cursor_prefix_width =
                    buffer_str[0..cursor_xpos].width() + tui_text_field._prompt.len();
                let y_offset = cursor_prefix_width / area_width;
                cursor_info.y = area.height as usize - multiline_height + y_offset;
                cursor_info.x = cursor_prefix_width % area_width + y_offset;

                // render multiline textfield
                frame.render_widget(Clear, multiline_rect);
                frame.render_widget(multiline, multiline_rect);

                // render cursor
                frame.set_cursor(cursor_info.x as u16, cursor_info.y as u16);
            })
            .unwrap();

        if let Ok(event) = self.context.poll_event() {
            match event {
                AppEvent::Termion(Event::Key(key)) => {
                    match key {
                        Key::Backspace => {
                            if self.line_buffer.backspace(1) {
                                self.completion_tracker.take();
                            }
                        }
                        Key::Left => {
                            if self.line_buffer.move_backward(1) {
                                self.completion_tracker.take();
                            }
                        }
                        Key::Right => {
                            if self.line_buffer.move_forward(1) {
                                self.completion_tracker.take();
                            }
                        }
                        Key::Delete => {
                            if self.line_buffer.delete(1).is_some() {
                                self.completion_tracker.take();
                            }
                        }
                        Key::Home => {
                            self.line_buffer.move_home();
                            self.completion_tracker.take();
                        }
                        Key::End => {
                            self.line_buffer.move_end();
                            self.completion_tracker.take();
                        }
                        Key::Up => {
                            self.curr_history_index = if self.curr_history_index > 0 {
                                self.curr_history_index - 1
                            } else {
                                0
                            };
                            self.line_buffer.move_home();
                            self.line_buffer.kill_line();
                            if let Some(s) = self
                                .context
                                .commandline_context_ref()
                                .history_ref()
                                .get(self.curr_history_index)
                            {
                                self.line_buffer.insert_str(0, s);
                            }
                        }
                        Key::Down => {
                            self.curr_history_index = if self.curr_history_index
                                < self.context.commandline_context_ref().history_ref().len()
                            {
                                self.curr_history_index + 1
                            } else {
                                self.curr_history_index
                            };
                            self.line_buffer.move_home();
                            self.line_buffer.kill_line();
                            if let Some(s) = self
                                .context
                                .commandline_context_ref()
                                .history_ref()
                                .get(self.curr_history_index)
                            {
                                self.line_buffer.insert_str(0, s);
                            }
                        }
                        Key::Esc => {
                            let _ = terminal.hide_cursor();
                            return GetInputState::Stopped;
                        }
                        Key::Char('\t') => {
                            if self.completion_tracker.is_none() {
                                let res = completer.complete_path(
                                    self.line_buffer.as_str(),
                                    self.line_buffer.pos(),
                                );
                                if let Ok((pos, mut candidates)) = res {
                                    candidates.sort_by(|x, y| {
                                        x.display()
                                            .partial_cmp(y.display())
                                            .unwrap_or(std::cmp::Ordering::Less)
                                    });
                                    let ct = CompletionTracker::new(
                                        pos,
                                        candidates,
                                        String::from(self.line_buffer.as_str()),
                                    );
                                    self.completion_tracker = Some(ct);
                                }
                            }

                            if let Some(ref mut s) = self.completion_tracker {
                                if s.index < s.candidates.len() {
                                    let candidate = &s.candidates[s.index];
                                    completer.update(
                                        &mut self.line_buffer,
                                        s.pos,
                                        candidate.display(),
                                    );
                                    s.index += 1;
                                }
                            }
                        }
                        Key::Char('\n') => {
                            let _ = terminal.hide_cursor();
                            return if self.line_buffer.as_str().is_empty() {
                                GetInputState::Stopped
                            } else {
                                GetInputState::Finished(self.line_buffer.to_string())
                            };
                        }
                        Key::Char(c) => {
                            if self.line_buffer.insert(c, 1).is_some() {
                                self.completion_tracker.take();
                            }
                        }
                        _ => {}
                    }
                    self.context.flush_event();
                }
                AppEvent::Termion(_) => {
                    self.context.flush_event();
                }
                event => input::process_noninteractive(event, self.context),
            };
        }
        GetInputState::Running(self.line_buffer.to_string())
    }

    pub fn finished(mut self) -> Option<String> {
        loop {
            match self.next() {
                GetInputState::Running(_) => (),
                GetInputState::Stopped => return None,
                GetInputState::Finished(s) => return Some(s.to_string()),
            };
        }
    }
}
