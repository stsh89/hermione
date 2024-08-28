pub enum Message {
    DeleteWorkspace,
    EnterWorkspace,
    Exit,
    ExitWorkspace,
    SelectCommand(usize),
    SelectWorkspace(usize),
    UnselectWorkspace,
}
