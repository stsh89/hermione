use hermione_nexus::definitions::WorkspaceId;

pub enum Sygnal {
    Exit,
    ClearInput,
    CreateWorkspace,
    DeleteChar,
    EnterChar(char),
    EnterInputMode,
    ExitInputMode,
    ListWorkspaces,
    MoveCusorLeft,
    MoveCusorRight,
    SelectNextInput,
    UpdateWorkspace(WorkspaceId),
}
