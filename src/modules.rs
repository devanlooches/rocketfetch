use crate::cli::Mode;
use crate::config::Config;
use crate::handle_error;
// use crate::utils::handle_error_result;
use console::Style;
use libmacchina::traits::GeneralReadout as _;
use libmacchina::traits::KernelReadout as _;
use libmacchina::traits::PackageReadout as _;
use libmacchina::GeneralReadout;
use libmacchina::KernelReadout;
use libmacchina::PackageReadout;
use user_error::{UserFacingError, UFE};
#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields, default)]
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

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields, default)]
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
    pub fn get_info(&self) -> String {
        let general_readout = GeneralReadout::new();
        let hostname = handle_error!(general_readout.hostname(), "Failed to get hostname");
        let username = handle_error!(general_readout.username(), "Failed to get username");
        format!(
            "{}{}{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(username),
            Style::from_dotted_str(&self.separator_style).apply_to(&self.separator_char),
            Style::from_dotted_str(&self.output_style).apply_to(hostname)
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields, default)]
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
    pub fn get_info(&self, num: usize) -> String {
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

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields, default)]
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
    pub fn get_os(&self) -> String {
        let general_readout = GeneralReadout::new();
        let os: String;
        if cfg!(target_os = "linux") {
            os = handle_error!(general_readout.distribution(), "Failed to find distro");
        } else {
            os = handle_error!(general_readout.os_name(), "Failed to find OS name");
        }
        os
    }
    pub fn get_info(&self) -> String {
        let os = self.get_os();
        let build_version = Config::run_cmd("sw_vers -buildVersion", "Failed to get build version");
        let arch = Config::run_cmd("uname -m", "Failed to get arch");

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

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields, default)]
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
    pub fn get_info(&self) -> String {
        let general_readout = GeneralReadout::new();
        let machine = handle_error!(general_readout.machine(), "Failed to find machine name");
        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(machine)
        )
    }
}
#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields, default)]
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
    pub fn get_info(&self) -> String {
        let kernel_readout = KernelReadout::new();
        let kernel = handle_error!(
            kernel_readout.pretty_kernel(),
            "Failed to find kernel version"
        );
        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(kernel)
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields, default)]
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
    pub fn get_info(&self) -> String {
        let general_readout = GeneralReadout::new();
        let uptime = handle_error!(general_readout.uptime(), "Failed to get uptime");
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
#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields, default)]
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
    pub fn get_info(&self) -> String {
        let package_readout = PackageReadout::new();
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

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields, default)]
pub struct Shell {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Shell {
    fn default() -> Self {
        Shell {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from("Shell: "),
            output_style: String::from("white"),
        }
    }
}

impl Shell {
    pub fn get_info(&self) -> String {
        use regex::Regex;
        let ver_regex = Regex::new(r"\d+\.\d+\.\d+").unwrap();
        let general_readout = &GeneralReadout::new();
        let shell = general_readout.shell(
            libmacchina::traits::ShellFormat::Relative,
            libmacchina::traits::ShellKind::Default,
        );
        let shell = handle_error!(shell, "Failed to get shell");
        let version = Config::run_cmd(
            format!("{} --version", shell).as_str(),
            "Failed to get shell version",
        );
        let locations = ver_regex.find(&version).unwrap();
        let version = &version[locations.start()..locations.end()];
        format!(
            "{}{} {}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(shell),
            Style::from_dotted_str(&self.output_style).apply_to(version)
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields, default)]
pub struct Resolution {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from("Resolution: "),
            output_style: String::from("white"),
        }
    }
}

impl Resolution {
    pub fn get_info(&self) -> String {
        let general_readout = GeneralReadout::new();
        let resolution = handle_error!(general_readout.resolution(), "Failed to get resolution");

        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(resolution),
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields, default)]
pub struct DesktopEnvironment {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for DesktopEnvironment {
    fn default() -> Self {
        DesktopEnvironment {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from("Desktop Environment: "),
            output_style: String::from("white"),
        }
    }
}

impl DesktopEnvironment {
    pub fn get_info(&self) -> String {
        let general_readout = GeneralReadout::new();
        let resolution = handle_error!(
            general_readout.desktop_environment(),
            "Failed to get desktop environment"
        );

        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(resolution),
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields, default)]
pub struct WindowManager {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for WindowManager {
    fn default() -> Self {
        WindowManager {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from("Window Manager: "),
            output_style: String::from("white"),
        }
    }
}

impl WindowManager {
    pub fn get_info(&self) -> String {
        let general_readout = GeneralReadout::new();
        let resolution = handle_error!(
            general_readout.window_manager(),
            "Failed to get window manager"
        );

        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(resolution),
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields, default)]
pub struct Terminal {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Terminal {
    fn default() -> Self {
        Terminal {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from("Terminal: "),
            output_style: String::from("white"),
        }
    }
}

impl Terminal {
    pub fn get_info(&self) -> String {
        let general_readout = GeneralReadout::new();
        let resolution = handle_error!(general_readout.terminal(), "Failed to get terminal name");

        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(resolution),
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(deny_unknown_fields, default)]
pub struct Module {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
    command: String,
}

impl Module {
    pub fn get_info(&self) -> String {
        let output = Config::run_cmd(&self.command, "Failed to run module command");

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
