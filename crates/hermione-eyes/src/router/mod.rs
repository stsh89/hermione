pub mod powershell;
pub mod workspaces;

pub enum Router {
    Workspaces(workspaces::Router),
    Powershell(powershell::Router),
}

macro_rules! impl_from_parameters {
    ($route:ident, $subrouter:ident, $subroute:ident, $parameters:ident) => {
        impl From<$subrouter::$parameters> for Router {
            fn from(parameters: $subrouter::$parameters) -> Self {
                Router::$route($subrouter::Router::$subroute(parameters))
            }
        }
    };

    ($route:ident, $subrouter:ident, $subroute:ident, $subsubrouter:ident, $subsubroute:ident, $parameters:ident) => {
        impl From<$subrouter::$subsubrouter::$parameters> for Router {
            fn from(parameters: $subrouter::$subsubrouter::$parameters) -> Self {
                Router::$route($subrouter::Router::$subroute(
                    $subrouter::$subsubrouter::Router::$subsubroute(parameters),
                ))
            }
        }
    };
}

impl_from_parameters!(Workspaces, workspaces, Create, CreateParameters);
impl_from_parameters!(Workspaces, workspaces, Delete, DeleteParameters);
impl_from_parameters!(Workspaces, workspaces, Edit, EditParameters);
impl_from_parameters!(Workspaces, workspaces, List, ListParameters);
impl_from_parameters!(Workspaces, workspaces, New, NewParameters);
impl_from_parameters!(Workspaces, workspaces, Update, UpdateParameters);

impl_from_parameters!(
    Workspaces,
    workspaces,
    Commands,
    commands,
    New,
    NewParameters
);
impl_from_parameters!(
    Workspaces,
    workspaces,
    Commands,
    commands,
    Edit,
    EditParameters
);
impl_from_parameters!(
    Workspaces,
    workspaces,
    Commands,
    commands,
    Delete,
    DeleteParameters
);
impl_from_parameters!(
    Workspaces,
    workspaces,
    Commands,
    commands,
    List,
    ListParameters
);
impl_from_parameters!(
    Workspaces,
    workspaces,
    Commands,
    commands,
    Get,
    GetParameters
);
impl_from_parameters!(
    Workspaces,
    workspaces,
    Commands,
    commands,
    Update,
    UpdateParameters
);
impl_from_parameters!(
    Workspaces,
    workspaces,
    Commands,
    commands,
    Create,
    CreateParameters
);

impl_from_parameters!(
    Powershell,
    powershell,
    CopyToClipboard,
    CopyToClipboardParameters
);
impl_from_parameters!(
    Powershell,
    powershell,
    ExecuteCommand,
    ExecuteCommandParameters
);
impl_from_parameters!(
    Powershell,
    powershell,
    StartWindowsTerminal,
    StartWindowsTerminalParameters
);
