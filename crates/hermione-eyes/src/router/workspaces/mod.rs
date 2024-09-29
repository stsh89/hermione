pub mod commands;

pub enum Router {
    Commands(commands::Router),
    Create(CreateParameters),
    Delete(DeleteParameters),
    Edit(EditParameters),
    List(ListParameters),
    New(NewParameters),
    Update(UpdateParameters),
}

pub struct CreateParameters {
    pub name: String,
    pub location: String,
}

pub struct DeleteParameters {
    pub id: String,
}

pub struct EditParameters {
    pub id: String,
}

#[derive(Default)]
pub struct ListParameters {
    pub search_query: String,
}

pub struct NewParameters {}

pub struct UpdateParameters {
    pub id: String,
    pub name: String,
    pub location: String,
}

impl From<Router> for super::Router {
    fn from(router: Router) -> Self {
        super::Router::Workspaces(router)
    }
}

macro_rules! from_parameters {
    ($action:ident, $parameters:ident) => {
        impl From<$parameters> for super::Router {
            fn from(parameters: $parameters) -> Self {
                Self::Workspaces(Router::$action(parameters))
            }
        }
    };
}

from_parameters!(Create, CreateParameters);
from_parameters!(Delete, DeleteParameters);
from_parameters!(Edit, EditParameters);
from_parameters!(List, ListParameters);
from_parameters!(New, NewParameters);
from_parameters!(Update, UpdateParameters);
