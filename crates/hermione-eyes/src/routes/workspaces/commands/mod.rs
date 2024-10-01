use crate::parameters;

pub enum Route {
    List(parameters::workspaces::commands::list::Parameters),
    Create(parameters::workspaces::commands::create::Parameters),
    New(parameters::workspaces::commands::new::Parameters),
    Get(parameters::workspaces::commands::get::Parameters),
    Delete(parameters::workspaces::commands::delete::Parameters),
    Edit(parameters::workspaces::commands::edit::Parameters),
    Update(parameters::workspaces::commands::update::Parameters),
}
