use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ItermTheme {
    #[serde(rename = "Ansi 0 Color")]
    ansi_0_color: AnsiColor,
    #[serde(rename = "Ansi 1 Color")]
    ansi_1_color: AnsiColor,
    #[serde(rename = "Ansi 2 Color")]
    ansi_2_color: AnsiColor,
    #[serde(rename = "Ansi 3 Color")]
    ansi_3_color: AnsiColor,
    #[serde(rename = "Ansi 4 Color")]
    ansi_4_color: AnsiColor,
    #[serde(rename = "Ansi 5 Color")]
    ansi_5_color: AnsiColor,
    #[serde(rename = "Ansi 6 Color")]
    ansi_6_color: AnsiColor,
    #[serde(rename = "Ansi 7 Color")]
    ansi_7_color: AnsiColor,
    #[serde(rename = "Ansi 8 Color")]
    ansi_8_color: AnsiColor,
    #[serde(rename = "Ansi 9 Color")]
    ansi_9_color: AnsiColor,
    #[serde(rename = "Ansi 10 Color")]
    ansi_10_color: AnsiColor,
    #[serde(rename = "Ansi 11 Color")]
    ansi_11_color: AnsiColor,
    #[serde(rename = "Ansi 12 Color")]
    ansi_12_color: AnsiColor,
    #[serde(rename = "Ansi 13 Color")]
    ansi_13_color: AnsiColor,
    #[serde(rename = "Ansi 14 Color")]
    ansi_14_color: AnsiColor,
    #[serde(rename = "Ansi 15 Color")]
    ansi_15_color: AnsiColor,
    #[serde(rename = "Background Color")]
    background_color: AnsiColor,
    #[serde(rename = "Badge Color")]
    badge_color: BadgeColor,
    #[serde(rename = "Bold Color")]
    bold_color: AnsiColor,
    #[serde(rename = "Cursor Color")]
    cursor_color: AnsiColor,
    #[serde(rename = "Cursor Guide Color")]
    cursor_guide_color: CursorGuideColor,
    #[serde(rename = "Cursor Text Color")]
    cursor_text_color: AnsiColor,
    #[serde(rename = "Foreground Color")]
    foreground_color: AnsiColor,
    #[serde(rename = "Link Color")]
    link_color: AnsiColor,
    #[serde(rename = "Selected Text Color")]
    selected_text_color: AnsiColor,
    #[serde(rename = "Selection Color")]
    selection_color: SelectionColor,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnsiColor {
    #[serde(rename = "Alpha Component")]
    alpha_component: f32,
    #[serde(rename = "Blue Component")]
    blue_component: f32,
    #[serde(rename = "Color Space")]
    color_space: String,
    #[serde(rename = "Green Component")]
    green_component: f32,
    #[serde(rename = "Red Component")]
    red_component: f32,
}

impl AnsiColor {
    pub fn to_hex(&self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}",
            (self.red_component * 255.0) as u8,
            (self.green_component * 255.0) as u8,
            (self.blue_component * 255.0) as u8
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BadgeColor {
    #[serde(rename = "Alpha Component")]
    alpha_component: f32,
    #[serde(rename = "Blue Component")]
    blue_component: f32,
    #[serde(rename = "Color Space")]
    color_space: String,
    #[serde(rename = "Green Component")]
    green_component: f32,
    #[serde(rename = "Red Component")]
    red_component: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CursorGuideColor {
    #[serde(rename = "Alpha Component")]
    alpha_component: f32,
    #[serde(rename = "Blue Component")]
    blue_component: f32,
    #[serde(rename = "Color Space")]
    color_space: String,
    #[serde(rename = "Green Component")]
    green_component: f32,
    #[serde(rename = "Red Component")]
    red_component: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SelectionColor {
    #[serde(rename = "Alpha Component")]
    alpha_component: f32,
    #[serde(rename = "Blue Component")]
    blue_component: f32,
    #[serde(rename = "Color Space")]
    color_space: String,
    #[serde(rename = "Green Component")]
    green_component: f32,
    #[serde(rename = "Red Component")]
    red_component: f32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Theme {
    foreground: String,
    accent: String,
    background: String,
    terminal_colors: WarpTheme,
    details: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TerminalColors {
    normal: Normal,
    bright: Bright,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WarpTheme {
    background: String,
    foreground: String,
    terminal_colors: TerminalColors,
    accent: String,
    details: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Normal {
    yellow: String,
    red: String,
    green: String,
    blue: String,
    white: String,
    magenta: String,
    black: String,
    cyan: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bright {
    black: String,
    green: String,
    red: String,
    blue: String,
    cyan: String,
    white: String,
    magenta: String,
    yellow: String,
}

impl From<ItermTheme> for WarpTheme {
    fn from(value: ItermTheme) -> Self {
        WarpTheme {
            background: value.background_color.to_hex(),
            foreground: value.foreground_color.to_hex(),
            accent: value.foreground_color.to_hex(),
            details: "darker".to_owned(),
            terminal_colors: TerminalColors {
                normal: Normal {
                    yellow: value.ansi_3_color.to_hex(),
                    red: value.ansi_1_color.to_hex(),
                    green: value.ansi_2_color.to_hex(),
                    blue: value.ansi_4_color.to_hex(),
                    white: value.ansi_7_color.to_hex(),
                    magenta: value.ansi_5_color.to_hex(),
                    black: value.ansi_0_color.to_hex(),
                    cyan: value.ansi_6_color.to_hex(),
                },
                bright: Bright {
                    black: value.ansi_8_color.to_hex(),
                    green: value.ansi_10_color.to_hex(),
                    red: value.ansi_9_color.to_hex(),
                    blue: value.ansi_11_color.to_hex(),
                    cyan: value.ansi_14_color.to_hex(),
                    white: value.ansi_15_color.to_hex(),
                    magenta: value.ansi_13_color.to_hex(),
                    yellow: value.ansi_12_color.to_hex(),
                },
            },
        }
    }
}
