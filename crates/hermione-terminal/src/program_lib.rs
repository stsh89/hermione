use uuid::Uuid;

pub trait Render {
    fn render(&mut self, state: &State) -> anyhow::Result<()>;
}

#[derive(Default)]
pub struct State {
    pub form: Form,
    pub mode: Mode,
    pub list: List,
    pub context: Context,
    pub notice: Option<Notice>,
    pub workspace_id: Option<Uuid>,
    pub command_id: Option<Uuid>,
}

pub struct Notice {
    pub kind: NoticeKind,
    pub message: String,
}

pub enum NoticeKind {
    Success,
    Error,
}

#[derive(Default)]
pub struct Form {
    pub inputs: Vec<String>,
    pub cursor: usize,
}

#[derive(Default, Clone, Copy)]
pub enum Context {
    #[default]
    Workspaces,
    WorkspaceForm,
    Commands,
    CommandForm,
    NotionBackupCredentialsForm,
}

#[derive(Default)]
pub struct List {
    pub items: Vec<ListItem>,
    pub cursor: usize,
    pub filter: String,
}

pub struct ListItem {
    pub id: Uuid,
    pub text: String,
}

#[derive(Default, Clone, Copy)]
pub enum Mode {
    #[default]
    Normal,
    Input,
}
