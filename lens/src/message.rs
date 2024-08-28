pub enum Message {
    DeleteWorkspace,
    EnterWorkspace,
    Exit,
    ExitWorkspace,
    SelectCommand(usize),
    SelectWorkspace(usize),
    UnselectWorkspace,
    NewWorkspace,
    CreateWorkspace(String),
    CancelNewWorkspace,
}
