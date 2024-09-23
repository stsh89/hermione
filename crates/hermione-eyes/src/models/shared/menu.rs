use ratatui::widgets::{ListItem, ListState};

pub struct Menu {
    items: Vec<MenuItem>,
    state: ListState,
    is_active: bool,
}

pub enum MenuItem {
    Back,
    CreateWorkspace,
    Exit,
}

impl<'a> From<&MenuItem> for ListItem<'a> {
    fn from(menu_item: &MenuItem) -> Self {
        let name = match menu_item {
            MenuItem::Exit => "Exit",
            MenuItem::CreateWorkspace => "Create workspace",
            MenuItem::Back => "Back",
        };

        ListItem::new(name)
    }
}

impl Menu {
    pub fn new(items: Vec<MenuItem>) -> Self {
        let mut menu = Self {
            items,
            state: ListState::default(),
            is_active: false,
        };

        if !menu.items.is_empty() {
            menu.state.select_first();
        }

        menu
    }

    pub fn select_next(&mut self) {
        self.state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn items(&self) -> &[MenuItem] {
        &self.items
    }

    pub fn state(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn item(&self) -> Option<&MenuItem> {
        self.state.selected().map(|index| &self.items[index])
    }
}
