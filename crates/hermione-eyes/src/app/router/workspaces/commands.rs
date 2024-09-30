pub enum Router {
    Create(CreateParameters),
    Delete(DeleteParameters),
    Edit(EditParameters),
    Get(GetParameters),
    List(ListParameters),
    New(NewParameters),
    Update(UpdateParameters),
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
