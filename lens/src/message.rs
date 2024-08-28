pub enum Message {
    EnterWorkspace,
    ExitWorkspace,
    Exit,
    SelectWorkspace(usize),
    SelectCommand(usize),
    UnselectWorkspace,
}
