use crate::cli::Mode;
use crate::config::Config;
use console::Style;
use rsys::Rsys;
use user_error::{UserFacingError, UFE};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
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
            padding_top: 0,
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct User {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
    separator_style: String,
    separator_char: String,
}

impl Default for User {
    fn default() -> Self {
        User {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from(""),
            output_style: String::from("bold.yellow"),
            separator_style: String::from("white"),
            separator_char: String::from("@"),
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
            Style::from_dotted_str(&self.separator_style).apply_to(&self.separator_char),
            Style::from_dotted_str(&self.output_style).apply_to(hostname)
        )
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
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

#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Host {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Host {
    fn default() -> Self {
        Host {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from("Host: "),
            output_style: String::from("white"),
        }
    }
}

impl Host {
    pub async fn get_info(&self) -> String {
        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style)
                .apply_to(Config::run_cmd("sysctl -n hw.model").await)
        )
    }
}
#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Kernel {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Kernel {
    fn default() -> Self {
        Kernel {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from("Kernel: "),
            output_style: String::from("white"),
        }
    }
}

impl Kernel {
    pub async fn get_info(&self) -> String {
        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(Config::run_cmd("uname -r").await)
        )
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Uptime {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
    time_format: String,
}

impl Default for Uptime {
    fn default() -> Self {
        Uptime {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from("Uptime: "),
            output_style: String::from("white"),
            time_format: String::from("$days days, $hours hours, $minutes minutes"),
        }
    }
}

impl Uptime {
    pub async fn get_info(&self) -> String {
        let shr = secfmt::from(Rsys::new().uptime().unwrap());
        let mut time = self.time_format.clone();
        time = time.replace("$years", &shr.years.to_string());
        time = time.replace("${years}", &shr.years.to_string());
        time = time.replace("$days", &shr.days.to_string());
        time = time.replace("${days}", &shr.days.to_string());
        time = time.replace("$hours", &shr.hours.to_string());
        time = time.replace("${hours}", &shr.hours.to_string());
        time = time.replace("$minutes", &shr.minutes.to_string());
        time = time.replace("${minutes}", &shr.minutes.to_string());
        time = time.replace("$seconds", &shr.seconds.to_string());
        time = time.replace("${seconds}", &shr.seconds.to_string());
        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(time)
        )
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Module {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
    command: String,
}

impl Module {
    pub async fn get_info(&self) -> String {
        let output = Config::run_cmd(&self.command).await;

        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(output)
        )
    }
}

impl Default for Module {
    fn default() -> Self {
        Module {
            command: String::from(""),
            output_style: String::from("white"),
            pre_text: String::from(""),
            pre_text_style: String::from("bold.yellow"),
        }
    }
}
