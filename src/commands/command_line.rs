use std::str::FromStr;

use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::key_command::{AppExecute, Command};
use crate::ui::views::{GetInputState, TuiTextField};
use crate::ui::TuiBackend;

pub fn command_and_args(s: &str) -> (&str, &str) {
    match s.find(' ') {
        Some(i) => (&s[..i], s[i..].trim_start()),
        None => (s, ""),
    }
}

pub fn read_and_execute(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    keymap_t: &AppKeyMapping,
    prefix: &str,
    suffix: &str,
) -> JoshutoResult<()> {
    context.flush_event();
    let mut user_input_iter = TuiTextField::default()
        .prompt(":")
        .prefix(prefix)
        .suffix(suffix)
        .get_input(backend, context);

    loop {
        match user_input_iter.next() {
            GetInputState::Stopped => return Ok(()),
            GetInputState::Running(user_input) => {
                let (command, args) = command_and_args(&user_input);
                if command == crate::key_command::CMD_SEARCH_STRING {
                    crate::commands::search_string::search_string(context, args, true)
                        .expect("search cannot fail");
                }
            }
            GetInputState::Finished(user_input) => {
                let user_input = user_input.trim_start();
                context
                    .commandline_context_mut()
                    .history_mut()
                    .add(user_input);

                let command = Command::from_str(user_input)?;
                return command.execute(context, backend, keymap_t);
            }
        }
    }
}
