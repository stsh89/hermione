pub mod powershell;
pub mod workspaces;

pub enum Route {
    Workspaces(workspaces::Route),
    Powershell(powershell::Route),
}
