pub mod powershell;
pub mod workspaces;

pub enum Route {
    Powershell(powershell::Route),
    Workspaces(workspaces::Route),
}
