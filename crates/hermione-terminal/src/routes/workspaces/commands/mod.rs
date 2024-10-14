use crate::parameters;

pub enum Route {
    Create(parameters::workspaces::commands::create::Parameters),
    Delete(parameters::workspaces::commands::delete::Parameters),
    Edit(parameters::workspaces::commands::edit::Parameters),
    List(parameters::workspaces::commands::list::Parameters),
    New(parameters::workspaces::commands::new::Parameters),
    Update(parameters::workspaces::commands::update::Parameters),
}
