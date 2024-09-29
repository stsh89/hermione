pub mod workspaces;

pub enum Router {
    Workspaces(workspaces::Router),
}
