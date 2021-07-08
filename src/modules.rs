use crate::cli::Mode;
use crate::config::Config;
use console::Style;
// use nixinfo;
use rsys::Rsys;
use user_error::{UserFacingError, UFE};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Format {
    pub mode: Mode,
    pub top_left_corner_char: char,
    pub top_right_corner_char: char,
    pub bottom_left_corner_char: char,
    pub bottom_right_corner_char: char,
    pub horizontal_char: char,
    pub vertical_char: char,
    pub padding_right: usize,
    pub padding_left: usize,
    pub padding_bottom: usize,
    pub padding_top: usize,
}

impl Default for Format {
    fn default() -> Self {
        Format {
            mode: Mode::Classic,
            top_left_corner_char: '╭',
            top_right_corner_char: '╮',
            bottom_left_corner_char: '╰',
            bottom_right_corner_char: '╯',
            horizontal_char: '─',
            vertical_char: '│',
            padding_right: 1,
            padding_left: 1,
            padding_bottom: 0,
            padding_top: 0,
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct User {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
    seperator_style: String,
    seperator_char: String,
}

impl Default for User {
    fn default() -> Self {
        User {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from(""),
            output_style: String::from("bold.yellow"),
            seperator_style: String::from("white"),
            seperator_char: String::from("@"),
        }
    }
}

impl User {
    pub async fn get_info(&self) -> String {
        let hostname = match Rsys::new().hostname() {
            Ok(v) => v,
            Err(r) => {
                UserFacingError::new("Failed to get hostname")
                    .reason(r.to_string())
                    .print_and_exit();
                unreachable!()
            }
        };
        let username = Config::run_cmd("whoami").await;
        format!(
            "{}{}{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(username),
            Style::from_dotted_str(&self.seperator_style).apply_to(&self.seperator_char),
            Style::from_dotted_str(&self.output_style).apply_to(hostname)
        )
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Delimiter {
    style: String,
    repeat_num: usize,
    char: char,
}

impl Default for Delimiter {
    fn default() -> Self {
        Delimiter {
            style: String::from("white"),
            repeat_num: 0,
            char: '-',
        }
    }
}

impl Delimiter {
    pub async fn get_info(&self, num: usize) -> String {
        let mut repeat = self.repeat_num;
        if repeat == 0 {
            repeat = num;
        }
        format!(
            "{}",
            Style::from_dotted_str(&self.style).apply_to(self.char.to_string().repeat(repeat))
        )
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Os {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Os {
    fn default() -> Self {
        Os {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from("OS: "),
            output_style: String::from("white"),
        }
    }
}

impl Os {
    pub async fn get_os(&self) -> String {
        let os: String;
        if cfg!(target_os = "linux") {
            os = match nixinfo::distro() {
                Ok(v) => v,
                Err(r) => {
                    UserFacingError::new("Failed to find distro")
                        .reason(r.to_string())
                        .print_and_exit();
                    unreachable!()
                }
            };
        } else {
            os = std::env::consts::OS.to_string();
        }
        os
    }
    pub async fn get_info(&self) -> String {
        let os = self.get_os().await;
        let build_version = Config::run_cmd("sw_vers -buildVersion").await;
        let arch = Config::run_cmd("machine").await;
        let version: String;
        if cfg!(target_os = "macos") {
            version = Config::run_cmd("sw_vers -productVersion").await;
        } else {
            version = String::from("");
        }

        let output_style = Style::from_dotted_str(&self.output_style);
        format!(
            "{}{} {} {} {}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            output_style.apply_to(os),
            output_style.apply_to(version),
            output_style.apply_to(build_version),
            output_style.apply_to(arch)
        )
    }
}
