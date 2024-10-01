use crate::parameters;

pub mod commands;

pub enum Route {
    Commands(commands::Route),
    List(parameters::workspaces::list::Parameters),
    Create(parameters::workspaces::create::Parameters),
    New,
    Update(parameters::workspaces::update::Parameters),
    Delete(parameters::workspaces::delete::Parameters),
    Edit(parameters::workspaces::edit::Parameters),
}
