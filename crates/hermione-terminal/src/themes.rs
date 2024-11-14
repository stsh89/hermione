use ratatui::style::Color;

pub const HIGHLIGHT_SYMBOL: &str = "➤ ";

#[derive(Clone, Copy, Default)]
pub struct Theme {
    pub background_color: Color,
    pub danger_color: Color,
    pub foreground_color: Color,
    pub highlight_color: Color,
    pub input_color: Color,
    pub popup_background_color: Color,
    pub status_bar_background_color: Color,
    pub status_bar_foreground_color: Color,
}

pub fn github_dark() -> Theme {
    Theme {
        // #0d1117
        background_color: Color::Rgb(13, 17, 23),

        // #f85149
        danger_color: Color::Rgb(248, 81, 73),

        // #8b949e
        foreground_color: Color::Rgb(139, 148, 158),

        // #58a6ff
        highlight_color: Color::Rgb(88, 166, 255),

        // #c9d1d9
        input_color: Color::Rgb(201, 209, 217),

        // #161b22
        popup_background_color: Color::Rgb(22, 27, 34),

        // #21262d
        status_bar_background_color: Color::Rgb(33, 38, 45),

        // #8b949e
        status_bar_foreground_color: Color::Rgb(139, 148, 158),
    }
}
