pub enum Message {
    Close,

    DeleteChar,

    DeleteSelection,

    Enter,

    ExitContext,

    EnterChar(char),

    MoveCusorLeft,

    MoveCusorRight,

    New,

    ToggleActiveInput,

    SelectNext,

    SelectPrevious,
}
