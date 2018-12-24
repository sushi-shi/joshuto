extern crate ncurses;

use std::collections::HashMap;

#[allow(non_camel_case_types)]
#[derive(Debug,Clone)]
pub enum Keycode {
    CTRL = ncurses::BUTTON_CTRL as isize,
    SHIFT = ncurses::BUTTON_SHIFT as isize,
    ALT = ncurses::BUTTON_ALT as isize,

    CODE_YES = ncurses::KEY_CODE_YES as isize,
    BREAK = ncurses::KEY_BREAK as isize,
    SRESET = ncurses::KEY_SRESET as isize,
    RESET = ncurses::KEY_RESET as isize,
    DOWN = ncurses::KEY_DOWN as isize,
    UP = ncurses::KEY_UP as isize,
    LEFT = ncurses::KEY_LEFT as isize,
    RIGHT = ncurses::KEY_RIGHT as isize,
    HOME = ncurses::KEY_HOME as isize,
    BACKSPACE = ncurses::KEY_BACKSPACE as isize,
    F0 = ncurses::KEY_F0 as isize,
    F1 = ncurses::KEY_F1 as isize,
    F2 = ncurses::KEY_F2 as isize,
    F3 = ncurses::KEY_F3 as isize,
    F4 = ncurses::KEY_F4 as isize,
    F5 = ncurses::KEY_F5 as isize,
    F6 = ncurses::KEY_F6 as isize,
    F7 = ncurses::KEY_F7 as isize,
    F8 = ncurses::KEY_F8 as isize,
    F9 = ncurses::KEY_F9 as isize,
    F10 = ncurses::KEY_F10 as isize,
    F11 = ncurses::KEY_F11 as isize,
    F12 = ncurses::KEY_F12 as isize,
    F13 = ncurses::KEY_F13 as isize,
    F14 = ncurses::KEY_F14 as isize,
    F15 = ncurses::KEY_F15 as isize,
    DL = ncurses::KEY_DL as isize,          /* delete-line key */
    IL = ncurses::KEY_IL as isize,          /* insert-line key */
    DC = ncurses::KEY_DC as isize,          /* delete-character key */
    IC = ncurses::KEY_IC as isize,          /* insert-character key */
    EIC = ncurses::KEY_EIC as isize,
    CLEAR = ncurses::KEY_CLEAR as isize,
    EOS = ncurses::KEY_EOS as isize,
    EOL = ncurses::KEY_EOL as isize,
    SF = ncurses::KEY_SF as isize,
    SR = ncurses::KEY_SR as isize,
    NPAGE = ncurses::KEY_NPAGE as isize,    /* next-page key */
    PPAGE = ncurses::KEY_PPAGE as isize,    /* previous-page key */
    STAB = ncurses::KEY_STAB as isize,
    CTAB = ncurses::KEY_CTAB as isize,
    CATAB = ncurses::KEY_CATAB as isize,
//    ENTER = ncurses::KEY_ENTER as isize,
    PRINT = ncurses::KEY_PRINT as isize,    /* print key */
    LL = ncurses::KEY_LL as isize,
    A1 = ncurses::KEY_A1 as isize,
    A3 = ncurses::KEY_A3 as isize,
    B2 = ncurses::KEY_B2 as isize,
    C1 = ncurses::KEY_C1 as isize,
    C3 = ncurses::KEY_C3 as isize,
    BTAB = ncurses::KEY_BTAB as isize,
    BEG = ncurses::KEY_BEG as isize,
    CANCEL = ncurses::KEY_CANCEL as isize,
    CLOSE = ncurses::KEY_CLOSE as isize,
    COMMAND = ncurses::KEY_COMMAND as isize,
    COPY = ncurses::KEY_COPY as isize,
    CREATE = ncurses::KEY_CREATE as isize,
    END = ncurses::KEY_END as isize,
    EXIT = ncurses::KEY_EXIT as isize,
    FIND = ncurses::KEY_FIND as isize,
    HELP = ncurses::KEY_HELP as isize,
    MARK = ncurses::KEY_MARK as isize,
    MESSAGE = ncurses::KEY_MESSAGE as isize,
    MOVE = ncurses::KEY_MOVE as isize,
    NEXT = ncurses::KEY_NEXT as isize,
    OPEN = ncurses::KEY_OPEN as isize,
    OPTIONS = ncurses::KEY_OPTIONS as isize,
    PREVIOUS = ncurses::KEY_PREVIOUS as isize,
    REDO = ncurses::KEY_REDO as isize,
    REFERENCE = ncurses::KEY_REFERENCE as isize,
    REFRESH = ncurses::KEY_REFRESH as isize,
    REPLACE = ncurses::KEY_REPLACE as isize,
    RESTART = ncurses::KEY_RESTART as isize,
    RESUME = ncurses::KEY_RESUME as isize,
    SAVE = ncurses::KEY_SAVE as isize,
    SBEG = ncurses::KEY_SBEG as isize,
    SCANCEL = ncurses::KEY_SCANCEL as isize,
    SCOMMAND = ncurses::KEY_SCOMMAND as isize,
    SCOPY = ncurses::KEY_SCOPY as isize,
    SCREATE = ncurses::KEY_SCREATE as isize,
    SDC = ncurses::KEY_SDC as isize,	
    SDL = ncurses::KEY_SDL as isize,	
    SELECT = ncurses::KEY_SELECT as isize,
    SEND = ncurses::KEY_SEND as isize,
    SEOL = ncurses::KEY_SEOL as isize,
    SEXIT = ncurses::KEY_SEXIT as isize,
    SFIND = ncurses::KEY_SFIND as isize,
    SHELP = ncurses::KEY_SHELP as isize,
    SHOME = ncurses::KEY_SHOME as isize,
    SIC = ncurses::KEY_SIC as isize,	
    SLEFT = ncurses::KEY_SLEFT as isize,
    SMESSAGE = ncurses::KEY_SMESSAGE as isize,
    SMOVE = ncurses::KEY_SMOVE as isize,
    SNEXT = ncurses::KEY_SNEXT as isize,
    SOPTIONS = ncurses::KEY_SOPTIONS as isize,
    SPREVIOUS = ncurses::KEY_SPREVIOUS as isize,
    SPRINT = ncurses::KEY_SPRINT as isize,
    SREDO = ncurses::KEY_SREDO as isize,
    SREPLACE = ncurses::KEY_SREPLACE as isize,
    SRIGHT = ncurses::KEY_SRIGHT as isize,
    SRSUME = ncurses::KEY_SRSUME as isize,
    SSAVE = ncurses::KEY_SSAVE as isize,
    SSUSPEND = ncurses::KEY_SSUSPEND as isize,
    SUNDO = ncurses::KEY_SUNDO as isize,
    SUSPEND = ncurses::KEY_SUSPEND as isize,
    UNDO = ncurses::KEY_UNDO as isize,
    MOUSE = ncurses::KEY_MOUSE as isize,
    RESIZE = ncurses::KEY_RESIZE as isize,
    EVENT = ncurses::KEY_EVENT as isize,
    MAX = ncurses::KEY_MAX as isize,

    NEWLINE = 0xA,

    ZERO = 0x30,
    ONE = 0x31,
    TWO = 0x32,
    THREE = 0x33,
    FOUR = 0x34,
    FIVE = 0x35,
    SIX = 0x36,
    SEVEN = 0x37,
    EIGHT = 0x38,
    NINE = 0x39,

    UPPER_A = 0x41,
    UPPER_B = 0x42,
    UPPER_C = 0x43,
    UPPER_D = 0x44,
    UPPER_E = 0x45,
    UPPER_F = 0x46,
    UPPER_G = 0x47,
    UPPER_H = 0x48,
    UPPER_I = 0x49,
    UPPER_J = 0x4A,
    UPPER_K = 0x4B,
    UPPER_L = 0x4C,
    UPPER_M = 0x4D,
    UPPER_N = 0x4E,
    UPPER_O = 0x4F,
    UPPER_P = 0x50,
    UPPER_Q = 0x51,
    UPPER_R = 0x52,
    UPPER_S = 0x53,
    UPPER_T = 0x54,
    UPPER_U = 0x55,
    UPPER_V = 0x56,
    UPPER_W = 0x57,
    UPPER_X = 0x58,
    UPPER_Y = 0x59,
    UPPER_Z = 0x5A,

    LOWER_A = 0x61,
    LOWER_B = 0x62,
    LOWER_C = 0x63,
    LOWER_D = 0x64,
    LOWER_E = 0x65,
    LOWER_F = 0x66,
    LOWER_G = 0x67,
    LOWER_H = 0x68,
    LOWER_I = 0x69,
    LOWER_J = 0x6A,
    LOWER_K = 0x6B,
    LOWER_L = 0x6C,
    LOWER_M = 0x6D,
    LOWER_N = 0x6E,
    LOWER_O = 0x6F,
    LOWER_P = 0x70,
    LOWER_Q = 0x71,
    LOWER_R = 0x72,
    LOWER_S = 0x73,
    LOWER_T = 0x74,
    LOWER_U = 0x75,
    LOWER_V = 0x76,
    LOWER_W = 0x77,
    LOWER_X = 0x78,
    LOWER_Y = 0x79,
    LOWER_Z = 0x7A,
}

impl Keycode {
    pub fn from_str(keycode: &str) -> Option<Keycode>
    {
        match keycode {
            "0" => Some(Keycode::ZERO),
            "1" => Some(Keycode::ONE),
            "2" => Some(Keycode::TWO),
            "3" => Some(Keycode::THREE),
            "4" => Some(Keycode::FOUR),
            "5" => Some(Keycode::FIVE),
            "6" => Some(Keycode::SIX),
            "7" => Some(Keycode::SEVEN),
            "8" => Some(Keycode::EIGHT),
            "9" => Some(Keycode::NINE),

            "A" => Some(Keycode::UPPER_A),
            "B" => Some(Keycode::UPPER_B),
            "C" => Some(Keycode::UPPER_C),
            "D" => Some(Keycode::UPPER_D),
            "E" => Some(Keycode::UPPER_E),
            "F" => Some(Keycode::UPPER_F),
            "G" => Some(Keycode::UPPER_G),
            "H" => Some(Keycode::UPPER_H),
            "I" => Some(Keycode::UPPER_I),
            "J" => Some(Keycode::UPPER_J),
            "K" => Some(Keycode::UPPER_K),
            "L" => Some(Keycode::UPPER_L),
            "M" => Some(Keycode::UPPER_M),
            "N" => Some(Keycode::UPPER_N),
            "O" => Some(Keycode::UPPER_O),
            "P" => Some(Keycode::UPPER_P),
            "Q" => Some(Keycode::UPPER_Q),
            "R" => Some(Keycode::UPPER_R),
            "S" => Some(Keycode::UPPER_S),
            "T" => Some(Keycode::UPPER_T),
            "U" => Some(Keycode::UPPER_U),
            "V" => Some(Keycode::UPPER_V),
            "W" => Some(Keycode::UPPER_W),
            "X" => Some(Keycode::UPPER_X),
            "Y" => Some(Keycode::UPPER_Y),
            "Z" => Some(Keycode::UPPER_Z),

            "a" => Some(Keycode::LOWER_A),
            "b" => Some(Keycode::LOWER_B),
            "c" => Some(Keycode::LOWER_C),
            "d" => Some(Keycode::LOWER_D),
            "e" => Some(Keycode::LOWER_E),
            "f" => Some(Keycode::LOWER_F),
            "g" => Some(Keycode::LOWER_G),
            "h" => Some(Keycode::LOWER_H),
            "i" => Some(Keycode::LOWER_I),
            "j" => Some(Keycode::LOWER_J),
            "k" => Some(Keycode::LOWER_K),
            "l" => Some(Keycode::LOWER_L),
            "m" => Some(Keycode::LOWER_M),
            "n" => Some(Keycode::LOWER_N),
            "o" => Some(Keycode::LOWER_O),
            "p" => Some(Keycode::LOWER_P),
            "q" => Some(Keycode::LOWER_Q),
            "r" => Some(Keycode::LOWER_R),
            "s" => Some(Keycode::LOWER_S),
            "t" => Some(Keycode::LOWER_T),
            "u" => Some(Keycode::LOWER_U),
            "v" => Some(Keycode::LOWER_V),
            "w" => Some(Keycode::LOWER_W),
            "x" => Some(Keycode::LOWER_X),
            "y" => Some(Keycode::LOWER_Y),
            "z" => Some(Keycode::LOWER_Z),

            "CTRL" => Some(Keycode::CTRL),
            "SHIFT" => Some(Keycode::SHIFT),
            "ALT" => Some(Keycode::ALT),

            "CODE_YES" => Some(Keycode::CODE_YES),
            "BREAK" => Some(Keycode::BREAK),
            "SRESET" => Some(Keycode::SRESET),
            "RESET" => Some(Keycode::RESET),
            "DOWN" => Some(Keycode::DOWN),
            "UP" => Some(Keycode::UP),
            "LEFT" => Some(Keycode::LEFT),
            "RIGHT" => Some(Keycode::RIGHT),
            "HOME" => Some(Keycode::HOME),
            "BACKSPACE" => Some(Keycode::BACKSPACE),
            "F0" => Some(Keycode::F0),
            "F1" => Some(Keycode::F1),
            "F2" => Some(Keycode::F2),
            "F3" => Some(Keycode::F3),
            "F4" => Some(Keycode::F4),
            "F5" => Some(Keycode::F5),
            "F6" => Some(Keycode::F6),
            "F7" => Some(Keycode::F7),
            "F8" => Some(Keycode::F8),
            "F9" => Some(Keycode::F9),
            "F10" => Some(Keycode::F10),
            "F11" => Some(Keycode::F11),
            "F12" => Some(Keycode::F12),
            "F13" => Some(Keycode::F13),
            "F14" => Some(Keycode::F14),
            "F15" => Some(Keycode::F15),
            "DL" => Some(Keycode::DL),
            "IL" => Some(Keycode::IL),
            "DELETE" => Some(Keycode::DC),
            "INSERT" => Some(Keycode::IC),
            "EIC" => Some(Keycode::EIC),
            "CLEAR" => Some(Keycode::CLEAR),
            "EOS" => Some(Keycode::EOS),
            "EOL" => Some(Keycode::EOL),
            "SF" => Some(Keycode::SF),
            "SR" => Some(Keycode::SR),
            "NPAGE" => Some(Keycode::NPAGE),
            "PPAGE" => Some(Keycode::PPAGE),
            "STAB" => Some(Keycode::STAB),
            "CTAB" => Some(Keycode::CTAB),
            "CATAB" => Some(Keycode::CATAB),
            "ENTER" => Some(Keycode::NEWLINE),

//            "ENTER" => Some(Keycode::ENTER),
            "PRINT" => Some(Keycode::PRINT),
            "LL" => Some(Keycode::LL),
            "A1" => Some(Keycode::A1),
            "A3" => Some(Keycode::A3),
            "B2" => Some(Keycode::B2),
            "C1" => Some(Keycode::C1),
            "C3" => Some(Keycode::C3),
            "BTAB" => Some(Keycode::BTAB),
            "BEG" => Some(Keycode::BEG),
            "CANCEL" => Some(Keycode::CANCEL),
            "CLOSE" => Some(Keycode::CLOSE),
            "COMMAND" => Some(Keycode::COMMAND),
            "COPY" => Some(Keycode::COPY),
            "CREATE" => Some(Keycode::CREATE),
            "END" => Some(Keycode::END),
            "EXIT" => Some(Keycode::EXIT),
            "FIND" => Some(Keycode::FIND),
            "HELP" => Some(Keycode::HELP),
            "MARK" => Some(Keycode::MARK),
            "MESSAGE" => Some(Keycode::MESSAGE),
            "MOVE" => Some(Keycode::MOVE),
            "NEXT" => Some(Keycode::NEXT),
            "OPEN" => Some(Keycode::OPEN),
            "OPTIONS" => Some(Keycode::OPTIONS),
            "PREVIOUS" => Some(Keycode::PREVIOUS),
            "REDO" => Some(Keycode::REDO),
            "REFERENCE" => Some(Keycode::REFERENCE),
            "REFRESH" => Some(Keycode::REFRESH),
            "REPLACE" => Some(Keycode::REPLACE),
            "RESTART" => Some(Keycode::RESTART),
            "RESUME" => Some(Keycode::RESUME),
            "SAVE" => Some(Keycode::SAVE),
            "SBEG" => Some(Keycode::SBEG),
            "SCANCEL" => Some(Keycode::SCANCEL),
            "SCOMMAND" => Some(Keycode::SCOMMAND),
            "SCOPY" => Some(Keycode::SCOPY),
            "SCREATE" => Some(Keycode::SCREATE),
            "SDC" => Some(Keycode::SDC),	
            "SDL" => Some(Keycode::SDL),	
            "SELECT" => Some(Keycode::SELECT),
            "SEND" => Some(Keycode::SEND),
            "SEOL" => Some(Keycode::SEOL),
            "SEXIT" => Some(Keycode::SEXIT),
            "SFIND" => Some(Keycode::SFIND),
            "SHELP" => Some(Keycode::SHELP),
            "SHOME" => Some(Keycode::SHOME),
            "SIC" => Some(Keycode::SIC),	
            "SLEFT" => Some(Keycode::SLEFT),
            "SMESSAGE" => Some(Keycode::SMESSAGE),
            "SMOVE" => Some(Keycode::SMOVE),
            "SNEXT" => Some(Keycode::SNEXT),
            "SOPTIONS" => Some(Keycode::SOPTIONS),
            "SPREVIOUS" => Some(Keycode::SPREVIOUS),
            "SPRINT" => Some(Keycode::SPRINT),
            "SREDO" => Some(Keycode::SREDO),
            "SREPLACE" => Some(Keycode::SREPLACE),
            "SRIGHT" => Some(Keycode::SRIGHT),
            "SRSUME" => Some(Keycode::SRSUME),
            "SSAVE" => Some(Keycode::SSAVE),
            "SSUSPEND" => Some(Keycode::SSUSPEND),
            "SUNDO" => Some(Keycode::SUNDO),
            "SUSPEND" => Some(Keycode::SUSPEND),
            "UNDO" => Some(Keycode::UNDO),
            "MOUSE" => Some(Keycode::MOUSE),
            "RESIZE" => Some(Keycode::RESIZE),
            "EVENT" => Some(Keycode::EVENT),
            "MAX" => Some(Keycode::MAX),

            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum JoshutoCommand {
    Quit,

    ReloadDirList,

    MoveUp,
    MoveDown,
    MovePageUp,
    MovePageDown,
    MoveHome,
    MoveEnd,
    ParentDirectory,

    DeleteFile,
    RenameFile,
    CutFile,
    CopyFile,
    PasteFile,
    Open,
    OpenWith,
    ToggleHiddenFiles,

    CompositeKeybind(HashMap<i32, JoshutoCommand>),
}

impl JoshutoCommand {

    pub fn from_str(keybind: &str) -> Option<Self>
    {
        match keybind {
            "Quit" => Some(JoshutoCommand::Quit),

            "ReloadDirList" => Some(JoshutoCommand::ReloadDirList),

            "MoveUp" => Some(JoshutoCommand::MoveUp),
            "MoveDown" => Some(JoshutoCommand::MoveDown),
            "MovePageUp" => Some(JoshutoCommand::MovePageUp),
            "MovePageDown" => Some(JoshutoCommand::MovePageDown),
            "MoveHome" => Some(JoshutoCommand::MoveHome),
            "MoveEnd" => Some(JoshutoCommand::MoveEnd),
            "ParentDirectory" => Some(JoshutoCommand::ParentDirectory),

            "DeleteFile" => Some(JoshutoCommand::DeleteFile),
            "RenameFile" => Some(JoshutoCommand::RenameFile),
            "CutFile" => Some(JoshutoCommand::CutFile),
            "CopyFile" => Some(JoshutoCommand::CopyFile),
            "PasteFile" => Some(JoshutoCommand::PasteFile),
            "Open" => Some(JoshutoCommand::Open),
            "OpenWith" => Some(JoshutoCommand::OpenWith),
            "ToggleHiddenFiles" => Some(JoshutoCommand::ToggleHiddenFiles),
            _ => None,
        }
    }

    pub fn clone(&self) -> Self
    {
        match self {
            JoshutoCommand::Quit => JoshutoCommand::Quit,
            JoshutoCommand::ReloadDirList => JoshutoCommand::ReloadDirList,

            JoshutoCommand::MoveUp => JoshutoCommand::MoveUp,
            JoshutoCommand::MoveDown => JoshutoCommand::MoveDown,
            JoshutoCommand::MovePageUp => JoshutoCommand::MovePageUp,
            JoshutoCommand::MovePageDown => JoshutoCommand::MovePageDown,
            JoshutoCommand::MoveHome => JoshutoCommand::MoveHome,
            JoshutoCommand::MoveEnd => JoshutoCommand::MoveEnd,
            JoshutoCommand::ParentDirectory => JoshutoCommand::ParentDirectory,

            JoshutoCommand::DeleteFile => JoshutoCommand::DeleteFile,
            JoshutoCommand::RenameFile => JoshutoCommand::RenameFile,
            JoshutoCommand::CutFile => JoshutoCommand::CutFile,
            JoshutoCommand::CopyFile => JoshutoCommand::CopyFile,
            JoshutoCommand::PasteFile => JoshutoCommand::PasteFile,
            JoshutoCommand::Open => JoshutoCommand::Open,
            JoshutoCommand::OpenWith => JoshutoCommand::OpenWith,
            JoshutoCommand::ToggleHiddenFiles => JoshutoCommand::ToggleHiddenFiles,

            JoshutoCommand::CompositeKeybind(_) => JoshutoCommand::Quit,
        }
    }
}
