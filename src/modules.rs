use crate::cli::Mode;
use crate::config::Config;
// use crate::utils::handle_error_result;
use console::Style;
use libmacchina::traits::GeneralReadout as _;
use libmacchina::traits::KernelReadout as _;
use libmacchina::traits::PackageReadout as _;
use libmacchina::GeneralReadout;
use libmacchina::KernelReadout;
use libmacchina::PackageReadout;
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
    pub async fn get_info(&self, general_readout: &GeneralReadout) -> String {
        let hostname = match general_readout.hostname() {
            Ok(v) => v,
            Err(r) => {
                UserFacingError::new("Failed to get hostname")
                    .reason(r.to_string())
                    .print_and_exit();
                unreachable!()
            }
        };
        let username = match general_readout.username() {
            Ok(v) => v,
            Err(r) => {
                UserFacingError::new("Failed to get username")
                    .reason(r.to_string())
                    .print_and_exit();
                unreachable!()
            }
        };
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
    pub async fn get_os(&self, general_readout: &GeneralReadout) -> String {
        let os: String;
        if cfg!(target_os = "linux") {
            os = match general_readout.distribution() {
                Ok(v) => v,
                Err(r) => {
                    UserFacingError::new("Failed to find distro")
                        .reason(r.to_string())
                        .print_and_exit();
                    unreachable!()
                }
            };
        } else {
            os = match general_readout.os_name() {
                Ok(v) => v,
                Err(r) => {
                    UserFacingError::new("Failed to find OS name")
                        .reason(r.to_string())
                        .print_and_exit();
                    unreachable!()
                }
            };
        }
        os
    }
    pub async fn get_info(&self, general_readout: &GeneralReadout) -> String {
        let os = self.get_os(general_readout).await;
        let build_version = Config::run_cmd("sw_vers -buildVersion").await;
        let arch = Config::run_cmd("uname -m").await;

        let output_style = Style::from_dotted_str(&self.output_style);
        format!(
            "{}{} {} {}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            output_style.apply_to(os),
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
    pub async fn get_info(&self, general_readout: &GeneralReadout) -> String {
        let machine = match general_readout.machine() {
            Ok(v) => v,
            Err(r) => {
                UserFacingError::new("Failed to find machine name")
                    .reason(r.to_string())
                    .print_and_exit();
                unreachable!()
            }
        };
        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(machine)
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
    pub async fn get_info(&self, kernel_readout: &KernelReadout) -> String {
        let kernel = match kernel_readout.pretty_kernel() {
            Ok(v) => v,
            Err(r) => {
                UserFacingError::new("Failed to find kernel version")
                    .reason(r.to_string())
                    .print_and_exit();
                unreachable!()
            }
        };
        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(kernel)
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
    pub async fn get_info(&self, general_readout: &GeneralReadout) -> String {
        let uptime = match general_readout.uptime() {
            Ok(v) => v,
            Err(r) => {
                UserFacingError::new("Failed to get uptime")
                    .reason(r.to_string())
                    .print_and_exit();
                unreachable!()
            }
        };
        let shr = secfmt::from(uptime as u64);
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
pub struct Packages {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Packages {
    fn default() -> Self {
        Packages {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from("Packages: "),
            output_style: String::from("white"),
        }
    }
}

impl Packages {
    pub async fn get_info(&self, package_readout: &PackageReadout) -> String {
        let package = package_readout.count_pkgs();
        let mut packages = String::new();
        for (name, num) in package {
            packages.push_str(format!("{} ({}) ", num, name.to_string()).as_str());
        }
        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(packages)
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
