use crate::{app::Hook, clients::memories::Client, Result};

pub mod powershell;
pub mod workspaces;

pub enum Router {
    Workspaces(workspaces::Router),
    Powershell(powershell::Router),
}

pub struct RouterParameters<'a> {
    pub memories: &'a Client,
}

impl Router {
    pub fn handle(self, parameters: RouterParameters) -> Result<Option<Box<dyn Hook>>> {
        let RouterParameters { memories } = parameters;

        match self {
            Router::Workspaces(router) => router.handle(workspaces::RouterParameters { memories }),
            Router::Powershell(router) => router.handle(powershell::RouterParameters { memories }),
        }
    }
}

macro_rules! impl_from_parameters {
    ($route:ident, $subrouter:ident, $subroute:ident, $parameters:ident) => {
        impl From<$subrouter::$parameters::Parameters> for Router {
            fn from(parameters: $subrouter::$parameters::Parameters) -> Self {
                Router::$route($subrouter::Router::$subroute(parameters))
            }
        }
    };

    ($route:ident, $subrouter:ident, $subroute:ident, $subsubrouter:ident, $subsubroute:ident, $parameters:ident) => {
        impl From<$subrouter::$subsubrouter::$parameters::Parameters> for Router {
            fn from(parameters: $subrouter::$subsubrouter::$parameters::Parameters) -> Self {
                Router::$route($subrouter::Router::$subroute(
                    $subrouter::$subsubrouter::Router::$subsubroute(parameters),
                ))
            }
        }
    };
}

impl_from_parameters!(Workspaces, workspaces, Create, create);
impl_from_parameters!(Workspaces, workspaces, Delete, delete);
impl_from_parameters!(Workspaces, workspaces, Edit, edit);
impl_from_parameters!(Workspaces, workspaces, List, list);
impl_from_parameters!(Workspaces, workspaces, Update, update);

impl_from_parameters!(Workspaces, workspaces, Commands, commands, New, new);
impl_from_parameters!(Workspaces, workspaces, Commands, commands, Edit, edit);
impl_from_parameters!(Workspaces, workspaces, Commands, commands, Delete, delete);
impl_from_parameters!(Workspaces, workspaces, Commands, commands, List, list);
impl_from_parameters!(Workspaces, workspaces, Commands, commands, Get, get);
impl_from_parameters!(Workspaces, workspaces, Commands, commands, Update, update);
impl_from_parameters!(Workspaces, workspaces, Commands, commands, Create, create);

impl_from_parameters!(Powershell, powershell, CopyToClipboard, copy_to_clipboard);
impl_from_parameters!(Powershell, powershell, ExecuteCommand, execute_command);
impl_from_parameters!(
    Powershell,
    powershell,
    StartWindowsTerminal,
    start_windows_terminal
);
