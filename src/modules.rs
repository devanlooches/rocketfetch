// use crate::utils::handle_error_result;
use console::Style;
use libmacchina::traits::GeneralReadout as _;
use libmacchina::traits::KernelReadout as _;
use libmacchina::traits::PackageReadout as _;
use libmacchina::GeneralReadout;
use libmacchina::KernelReadout;
use libmacchina::PackageReadout;
use user_error::{UserFacingError, UFE};

use crate::cli::Mode;
use crate::config::Config;
use crate::handle_error;

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
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
        Self {
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

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct User {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
    separator_style: String,
    separator_char: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::new(),
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
            Style::from_dotted_str(&self.output_style).apply_to(hostname.trim())
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct Delimiter {
    style: String,
    repeat_num: usize,
    char: char,
}

impl Default for Delimiter {
    fn default() -> Self {
        Self {
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

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct Os {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Os {
    fn default() -> Self {
        Self {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from("OS: "),
            output_style: String::from("white"),
        }
    }
}

impl Os {
    pub fn get_os() -> String {
        let general_readout = GeneralReadout::new();
        if cfg!(target_os = "linux") {
            return handle_error!(general_readout.distribution(), "Failed to find distro");
        } else if cfg!(target_os = "windows") {
            let version = handle_error!(
                KernelReadout::new().os_release(),
                "Failed to get windows version"
            );
            return format!("Windows {}", version);
        }
        handle_error!(general_readout.os_name(), "Failed to find OS name")
    }
    pub fn get_info(&self) -> String {
        let os = Self::get_os();
        let build_version = Config::run_cmd("sw_vers -buildVersion", "Failed to get build version");
        let arch = Config::run_cmd("machine", "Failed to get arch");

        let output_style = Style::from_dotted_str(&self.output_style);
        format!(
            "{}{} {} {}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            output_style.apply_to(os.trim()),
            output_style.apply_to(build_version.trim()),
            output_style.apply_to(arch.trim())
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct Host {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Host {
    fn default() -> Self {
        Self {
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
            Style::from_dotted_str(&self.output_style).apply_to(machine.trim())
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct Kernel {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Kernel {
    fn default() -> Self {
        Self {
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
            Style::from_dotted_str(&self.output_style).apply_to(kernel.trim())
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct Uptime {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
    time_format: String,
}

impl Default for Uptime {
    fn default() -> Self {
        Self {
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
            Style::from_dotted_str(&self.output_style).apply_to(time.trim())
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct Packages {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Packages {
    fn default() -> Self {
        Self {
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
            Style::from_dotted_str(&self.output_style).apply_to(packages.trim())
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct Shell {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from("Shell: "),
            output_style: String::from("white"),
        }
    }
}

impl Shell {
    pub fn get_info(&self) -> String {
        use regex::Regex;
        let ver_regex = Regex::new(r"(0|[1-9]\d*)\.(0|[1-9]\d*)\.?(0|[1-9]\d*)?(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?").unwrap();
        let general_readout = &GeneralReadout::new();
        let shell = general_readout.shell(
            libmacchina::traits::ShellFormat::Relative,
            libmacchina::traits::ShellKind::Default,
        );
        let shell = handle_error!(shell, "Failed to get shell");
        let version_output = Config::run_cmd(
            format!("{shell} --version").as_str(),
            "Failed to get shell version",
        );
        let version: String;
        if let Some(locations) = ver_regex.find(&version_output) {
            version = version_output[locations.start()..locations.end()].to_string();
        } else {
            version = String::from("(Unknown Version)");
        }
        format!(
            "{}{} {}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(shell.trim()),
            Style::from_dotted_str(&self.output_style).apply_to(version.trim())
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct Resolution {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Resolution {
    fn default() -> Self {
        Self {
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
            Style::from_dotted_str(&self.output_style).apply_to(resolution.trim()),
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct DesktopEnvironment {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for DesktopEnvironment {
    fn default() -> Self {
        Self {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from("Desktop Environment: "),
            output_style: String::from("white"),
        }
    }
}

impl DesktopEnvironment {
    pub fn get_info(&self) -> String {
        let general_readout = GeneralReadout::new();
        let desktop_environment = handle_error!(
            general_readout.desktop_environment(),
            "Failed to get desktop environment"
        );

        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(desktop_environment.trim()),
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct WindowManager {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for WindowManager {
    fn default() -> Self {
        Self {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from("Window Manager: "),
            output_style: String::from("white"),
        }
    }
}

impl WindowManager {
    pub fn get_info(&self) -> String {
        let general_readout = GeneralReadout::new();
        let window_manager = handle_error!(
            general_readout.window_manager(),
            "Failed to get window manager"
        );

        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(window_manager.trim()),
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct Terminal {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from("Terminal: "),
            output_style: String::from("white"),
        }
    }
}

impl Terminal {
    pub fn get_info(&self) -> String {
        let general_readout = GeneralReadout::new();
        let terminal = handle_error!(general_readout.terminal(), "Failed to get terminal name");

        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(terminal.trim()),
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct Cpu {
    pre_text_style: String,
    pre_text: String,
    output_style: String,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            pre_text_style: String::from("bold.yellow"),
            pre_text: String::from("CPU: "),
            output_style: String::from("white"),
        }
    }
}

impl Cpu {
    pub fn get_info(&self) -> String {
        let general_readout = GeneralReadout::new();
        let cpu = handle_error!(general_readout.cpu_model_name(), "Failed to get CPU name");

        format!(
            "{}{}",
            Style::from_dotted_str(&self.pre_text_style).apply_to(&self.pre_text),
            Style::from_dotted_str(&self.output_style).apply_to(cpu.trim()),
        )
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
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
            Style::from_dotted_str(&self.output_style).apply_to(output.trim())
        )
    }
}

impl Default for Module {
    fn default() -> Self {
        Self {
            command: String::new(),
            output_style: String::from("white"),
            pre_text: String::new(),
            pre_text_style: String::from("bold.yellow"),
        }
    }
}

#[cfg(test)]
#[cfg(target_os = "macos")]
mod module_tests {
    use super::*;

    fn run_cmd_unsafe(cmd: &str) -> String {
        use std::process::Command;
        let output = Command::new("sh")
            .args(["-c", cmd])
            .output()
            .unwrap()
            .stdout;
        String::from_utf8(output).unwrap().trim().to_string()
    }

    #[test]
    fn get_username() {
        let general_readout = GeneralReadout::new();
        println!("Username: {}", general_readout.username().unwrap());
    }

    #[test]
    fn get_hostname() {
        let general_readout = GeneralReadout::new();
        println!("Hostname: {}", general_readout.hostname().unwrap());
    }

    #[test]
    fn get_os() {
        let general_readout = GeneralReadout::new();
        if cfg!(target_os = "linux") {
            println!("Linux Distro: {}", general_readout.distribution().unwrap());
        } else {
            println!("OS Name: {}", general_readout.os_name().unwrap());
        }
        println!("Build Version: {}", run_cmd_unsafe("sw_vers -buildVersion"));
        println!("Arch: {}", run_cmd_unsafe("machine"));
    }

    #[test]
    fn get_host() {
        let general_readout = GeneralReadout::new();
        println!("Host: {}", general_readout.machine().unwrap());
    }

    #[test]
    fn get_kernel() {
        let kernel_readout = KernelReadout::new();
        println!("Kernel: {}", kernel_readout.pretty_kernel().unwrap());
    }

    #[test]
    fn get_uptime() {
        let general_readout = GeneralReadout::new();
        println!("Uptime: {}", general_readout.uptime().unwrap());
    }

    #[test]
    fn get_packages() {
        let package_readout = PackageReadout::new();
        let package = package_readout.count_pkgs();
        let mut packages = String::new();
        for (name, num) in package {
            packages.push_str(format!("{} ({}) ", num, name.to_string()).as_str());
        }
        println!("Packages: {}", packages);
    }

    #[test]
    fn get_shell() {
        use regex::Regex;
        let ver_regex = Regex::new(r"\d+\.\d+\.\d+").unwrap();
        let general_readout = &GeneralReadout::new();
        let shell = general_readout
            .shell(
                libmacchina::traits::ShellFormat::Relative,
                libmacchina::traits::ShellKind::Default,
            )
            .unwrap();
        let version = run_cmd_unsafe(format!("{} --version", shell).as_str());
        let locations = ver_regex.find(&version).unwrap();
        let version = &version[locations.start()..locations.end()];
        println!("Shell: {} version {}", shell, version);
    }

    #[test]
    fn get_resolution() {
        let general_readout = GeneralReadout::new();
        println!("Resolution: {}", general_readout.resolution().unwrap());
    }

    #[test]
    fn get_desktop_environment() {
        let general_readout = GeneralReadout::new();
        println!(
            "Desktop Environment: {}",
            general_readout.desktop_environment().unwrap()
        );
    }

    #[test]
    fn get_window_manager() {
        let general_readout = GeneralReadout::new();
        println!(
            "Window Manager: {}",
            general_readout.window_manager().unwrap()
        );
    }

    #[test]
    #[ignore = "Metric not available on virtual machines"]
    fn get_terminal_name() {
        let general_readout = GeneralReadout::new();
        println!("Terminal: {}", general_readout.terminal().unwrap());
    }

    #[test]
    fn get_cpu() {
        let general_readout = GeneralReadout::new();
        println!("CPU: {}", general_readout.cpu_model_name().unwrap());
    }
}

#[cfg(test)]
#[cfg(target_os = "windows")]
mod module_tests {
    use super::*;

    fn run_cmd_unsafe(cmd: &str) -> String {
        use std::process::Command;
        let output = Command::new("sh")
            .args(["-c", cmd])
            .output()
            .unwrap()
            .stdout;
        String::from_utf8(output).unwrap().trim().to_string()
    }

    #[test]
    fn get_username() {
        let general_readout = GeneralReadout::new();
        println!("Username: {}", general_readout.username().unwrap());
    }

    #[test]
    fn get_hostname() {
        let general_readout = GeneralReadout::new();
        println!("Hostname: {}", general_readout.hostname().unwrap());
    }

    #[test]
    fn get_os() {
        let general_readout = GeneralReadout::new();
        if cfg!(target_os = "linux") {
            println!("Linux Distro: {}", general_readout.distribution().unwrap());
        } else {
            println!("OS Name: {}", general_readout.os_name().unwrap());
        }
        println!("Build Version: {}", run_cmd_unsafe("sw_vers -buildVersion"));
        println!("Arch: {}", run_cmd_unsafe("machine"));
    }

    #[test]
    fn get_host() {
        let general_readout = GeneralReadout::new();
        println!("Host: {}", general_readout.machine().unwrap());
    }

    #[test]
    fn get_kernel() {
        let kernel_readout = KernelReadout::new();
        println!("Kernel: {}", kernel_readout.pretty_kernel().unwrap());
    }

    #[test]
    fn get_uptime() {
        let general_readout = GeneralReadout::new();
        println!("Uptime: {}", general_readout.uptime().unwrap());
    }

    #[test]
    fn get_packages() {
        let package_readout = PackageReadout::new();
        let package = package_readout.count_pkgs();
        let mut packages = String::new();
        for (name, num) in package {
            packages.push_str(format!("{} ({}) ", num, name.to_string()).as_str());
        }
        println!("Packages: {}", packages);
    }

    #[test]
    fn get_cpu() {
        let general_readout = GeneralReadout::new();
        println!("CPU: {}", general_readout.cpu_model_name().unwrap());
    }
}

#[cfg(test)]
#[cfg(target_os = "linux")]
mod module_tests {
    use super::*;

    fn run_cmd_unsafe(cmd: &str) -> String {
        use std::process::Command;
        let output = Command::new("sh")
            .args(["-c", cmd])
            .output()
            .unwrap()
            .stdout;
        String::from_utf8(output).unwrap().trim().to_string()
    }

    #[test]
    fn get_username() {
        let general_readout = GeneralReadout::new();
        println!("Username: {}", general_readout.username().unwrap());
    }

    #[test]
    fn get_hostname() {
        let general_readout = GeneralReadout::new();
        println!("Hostname: {}", general_readout.hostname().unwrap());
    }

    #[test]
    fn get_os() {
        let general_readout = GeneralReadout::new();
        if cfg!(target_os = "linux") {
            println!("Linux Distro: {}", general_readout.distribution().unwrap());
        } else {
            println!("OS Name: {}", general_readout.os_name().unwrap());
        }
        println!("Build Version: {}", run_cmd_unsafe("sw_vers -buildVersion"));
        println!("Arch: {}", run_cmd_unsafe("machine"));
    }

    #[test]
    #[ignore = "Metric unavailable on virtual machine"]
    fn get_host() {
        let general_readout = GeneralReadout::new();
        println!("Host: {}", general_readout.machine().unwrap());
    }

    #[test]
    fn get_kernel() {
        let kernel_readout = KernelReadout::new();
        println!("Kernel: {}", kernel_readout.pretty_kernel().unwrap());
    }

    #[test]
    fn get_uptime() {
        let general_readout = GeneralReadout::new();
        println!("Uptime: {}", general_readout.uptime().unwrap());
    }

    #[test]
    fn get_packages() {
        let package_readout = PackageReadout::new();
        let package = package_readout.count_pkgs();
        let mut packages = String::new();
        for (name, num) in package {
            packages.push_str(format!("{} ({}) ", num, name.to_string()).as_str());
        }
        println!("Packages: {}", packages);
    }

    #[test]
    fn get_shell() {
        use regex::Regex;
        let ver_regex = Regex::new(r"\d+\.\d+\.\d+").unwrap();
        let general_readout = &GeneralReadout::new();
        let shell = general_readout
            .shell(
                libmacchina::traits::ShellFormat::Relative,
                libmacchina::traits::ShellKind::Default,
            )
            .unwrap();
        let version = run_cmd_unsafe(format!("{} --version", shell).as_str());
        let locations = ver_regex.find(&version).unwrap();
        let version = &version[locations.start()..locations.end()];
        println!("Shell: {} version {}", shell, version);
    }

    #[test]
    fn get_resolution() {
        let general_readout = GeneralReadout::new();
        println!("Resolution: {}", general_readout.resolution().unwrap());
    }

    #[test]
    #[ignore = "Metric unavailable on virtual machine"]
    fn get_desktop_environment() {
        let general_readout = GeneralReadout::new();
        println!(
            "Desktop Environment: {}",
            general_readout.desktop_environment().unwrap()
        );
    }

    #[test]
    #[ignore = "Metric unavailable on virtual machine"]
    fn get_window_manager() {
        let general_readout = GeneralReadout::new();
        println!(
            "Window Manager: {}",
            general_readout.window_manager().unwrap()
        );
    }

    #[test]
    #[ignore = "Metric not available on virtual machines"]
    fn get_terminal_name() {
        let general_readout = GeneralReadout::new();
        println!("Terminal: {}", general_readout.terminal().unwrap());
    }

    #[test]
    fn get_cpu() {
        let general_readout = GeneralReadout::new();
        println!("CPU: {}", general_readout.cpu_model_name().unwrap());
    }
}
