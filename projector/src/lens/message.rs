pub enum Message {
    CloseLens,

    CreateCommand,

    CreateWorkspace,

    DeleteChar,

    DeleteWorkspace,

    EnterWorkspace,

    EnterWorkspaceForm,

    EnterCommandForm,

    ExitCommandForm,

    ExitWorkspace,

    ExitWorkspaceForm,

    InputChar(char),

    MoveCusorLeft,

    MoveCusorRight,

    ToggleActiveInput,

    SelectNextCommand,

    SelectNextWorkspace,

    SelectPreviousCommand,

    SelectPreviousWorkspace,
}
