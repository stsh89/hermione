pub mod commands;

use crate::parameters;

pub enum Route {
    Commands(commands::Route),
    Create(parameters::workspaces::create::Parameters),
    Delete(parameters::workspaces::delete::Parameters),
    Edit(parameters::workspaces::edit::Parameters),
    Home,
    List(parameters::workspaces::list::Parameters),
    New,
    Update(parameters::workspaces::update::Parameters),
}
