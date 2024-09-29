pub enum Router {
    CopyToClipboard(CopyToClipboardParameters),
    Create(CreateParameters),
    Delete(DeleteParameters),
    Edit(EditParameters),
    Execute(ExecuteParameters),
    Get(GetParameters),
    List(ListParameters),
    New(NewParameters),
    Update(UpdateParameters),
}

pub struct CopyToClipboardParameters {
    pub command_id: String,
    pub workspace_id: String,
}

pub struct CreateParameters {
    pub name: String,
    pub program: String,
    pub workspace_id: String,
}

pub struct DeleteParameters {
    pub command_id: String,
    pub workspace_id: String,
}

pub struct EditParameters {
    pub command_id: String,
    pub workspace_id: String,
}

pub struct ExecuteParameters {
    pub command_id: String,
    pub workspace_id: String,
}

pub struct GetParameters {
    pub command_id: String,
    pub workspace_id: String,
}

pub struct ListParameters {
    pub workspace_id: String,
    pub search_query: String,
}

pub struct NewParameters {
    pub workspace_id: String,
}

pub struct UpdateParameters {
    pub command_id: String,
    pub name: String,
    pub program: String,
    pub workspace_id: String,
}

macro_rules! from_parameters {
    ($action:ident, $parameters:ident) => {
        impl From<$parameters> for crate::router::Router {
            fn from(parameters: $parameters) -> Self {
                Self::Workspaces(super::Router::Commands(Router::$action(parameters)))
            }
        }
    };
}

from_parameters!(CopyToClipboard, CopyToClipboardParameters);
from_parameters!(Create, CreateParameters);
from_parameters!(Delete, DeleteParameters);
from_parameters!(Edit, EditParameters);
from_parameters!(Execute, ExecuteParameters);
from_parameters!(Get, GetParameters);
from_parameters!(List, ListParameters);
from_parameters!(New, NewParameters);
from_parameters!(Update, UpdateParameters);
