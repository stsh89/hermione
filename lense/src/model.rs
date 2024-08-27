use crate::Message;

#[derive(Default)]
pub struct Model {
    state: State,
}

#[derive(Default)]
pub enum State {
    #[default]
    Running,

    Exited,
}

impl Model {
    fn exit(&mut self) {
        self.state = State::Exited;
    }

    pub fn is_exited(&self) -> bool {
        matches!(self.state, State::Exited)
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::AddWorkspace => todo!(),
            Message::EnterWorkspace => todo!(),
            Message::Exit => self.exit(),
            Message::ExitWorkspace => todo!(),
            Message::NewWorkspace => todo!(),
            Message::RemoveWorkspace => todo!(),
            Message::SelectWorkspace => todo!(),
        }
    }
}
