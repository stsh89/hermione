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
