pub enum Message {
    CloseLens,

    CreateWorkspace,

    DeleteChar,

    DeleteWorkspace,

    EnterWorkspace,

    EnterWorkspaceForm,

    ExitWorkspace,

    ExitWorkspaceForm,

    InputChar(char),

    MoveCusorLeft,

    MoveCusorRight,

    SelectNextCommand,

    SelectNextWorkspace,

    SelectPreviousCommand,

    SelectPreviousWorkspace,
}
