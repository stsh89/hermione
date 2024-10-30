use ratatui::crossterm::style::Color;

pub const BACKGROUND_COLOR: Color = Color::Rgb {
    r: 13,
    g: 17,
    b: 23,
};

pub const FOREGROUND_COLOR: Color = Color::Rgb {
    r: 129,
    g: 148,
    b: 158,
};

pub const HIGHLIGHT_COLOR: Color = Color::Rgb {
    r: 201,
    g: 209,
    b: 217,
};

pub const HIGHLIGHT_SYMBOL: &str = "➤ ";

pub const INPUT_COLOR: Color = Color::Rgb {
    r: 201,
    g: 209,
    b: 217,
};

pub const STATUS_BAR_BACKGROUND_COLOR: Color = Color::Rgb {
    r: 33,
    g: 38,
    b: 45,
};

pub const STATUS_BAR_FOREGROUND_COLOR: Color = Color::Rgb {
    r: 129,
    g: 148,
    b: 158,
};
