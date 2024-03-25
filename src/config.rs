use std::cmp::Ordering;
use std::collections::HashMap;

use any_terminal_size::any_terminal_size;
use console::measure_text_width;
use console::style;
use structopt::StructOpt;
use user_error::{UserFacingError, UFE};

use crate::cli::Mode;
use crate::cli::Opt;
use crate::handle_error;
use crate::modules::{
    Cpu, Delimiter, DesktopEnvironment, Format, Host, Kernel, Module, Os, Packages, Resolution,
    Shell, Terminal, Uptime, User, WindowManager,
};

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "kebab-case", default)]
pub struct Config {
    module_order: String,
    wrap_lines: bool,
    offset: usize,
    logo_cmd: String,
    format: Format,
    user: User,
    delimiter: Delimiter,
    os: Os,
    host: Host,
    kernel: Kernel,
    uptime: Uptime,
    packages: Packages,
    shell: Shell,
    resolution: Resolution,
    desktop_environment: DesktopEnvironment,
    window_manager: WindowManager,
    terminal: Terminal,
    cpu: Cpu,

    #[serde(flatten)]
    custom_modules: HashMap<String, Module>,
}

impl Config {
    pub fn get_args() -> Opt {
        Opt::from_args()
    }
    pub fn path() -> String {
        let matches = Self::get_args();
        let home_dir = handle_error!(dirs::home_dir().ok_or(""), "Failed to find home directory");

        matches.config.unwrap_or(format!(
            "{}/.config/rocketfetch/config.toml",
            home_dir.to_string_lossy()
        ))
    }
    pub fn from_config(path: String) -> Self {
        match std::fs::read_to_string(path) {
            Ok(string) => match toml::from_str::<Self>(&string) {
                Ok(v) => v,
                Err(r) => {
                    let mut line: usize = 0;
                    let mut column: usize = 0;
                    let mut last = String::new();
                    let r = r.to_string();
                    println!("{r}");
                    let error = handle_error!(
                        r.split("at").last().ok_or(""),
                        "Failed to get line and column number of configuration error"
                    );
                    for word in error.split_whitespace() {
                        if last == "line" {
                            line = handle_error!(
                                word.parse::<usize>(),
                                "Failed to get line number of configuration error"
                            );
                        } else if last == "column" {
                            column = handle_error!(
                                word.parse::<usize>(),
                                "Failed to get column number of configuration error"
                            );
                        }
                        last = word.to_string();
                    }
                    UserFacingError::new("Unable to parse toml file")
                        .reason(format!(
                            "--> {line}:{col}
{line_len_sep} |
{line} | {line_contents}
{line_len_sep} |{col_len_sep}^--- {error}
{line_len_sep} |",
                            error = r,
                            line_len_sep = " ".repeat(line.to_string().len()),
                            col_len_sep = " ".repeat(column),
                            line = line,
                            line_contents = string.lines().collect::<Vec<&str>>()[line - 1],
                            col = column,
                        ))
                        .print_and_exit();
                    unreachable!()
                }
            },

            Err(r) => {
                println!(
                    "{}: Could not find default configuration file: {}. Falling back to default configuration.\n",
                    style("WARNING").yellow(),
                    r
                );
                Self::default()
            }
        }
    }

    fn get_module_order(&self) -> Vec<String> {
        use std::thread;
        let modules = self.module_order.split_whitespace().collect::<Vec<&str>>();
        let mut modules_unordered = HashMap::new();

        thread::scope(|s| {
            let mut handles = Vec::new();
            macro_rules! add_module {
                ($name:expr, $name_lit:literal) => {
                    if modules.contains(&$name_lit) {
                        let handle = s.spawn(|| -> (String, String) {
                            (String::from($name_lit), $name.get_info().replace('\n', " "))
                        });
                        handles.push(handle);
                    }
                };
            }
            add_module!(self.user, "user");
            add_module!(self.os, "os");
            add_module!(self.host, "host");
            add_module!(self.kernel, "kernel");
            add_module!(self.uptime, "uptime");
            add_module!(self.packages, "packages");
            add_module!(self.shell, "shell");
            add_module!(self.resolution, "resolution");
            add_module!(self.desktop_environment, "desktop-environment");
            add_module!(self.window_manager, "window-manager");
            add_module!(self.terminal, "terminal");
            add_module!(self.cpu, "cpu");
            for (name, module) in &self.custom_modules {
                let handle = s.spawn(|| -> (String, String) {
                    (name.clone(), module.get_info().replace('\n', " "))
                });
                handles.push(handle);
            }
            for handle in handles {
                let joined_handle = handle.join().unwrap();
                modules_unordered.insert(joined_handle.0, joined_handle.1);
            }
        });
        // let modules_unordered = modules_unordered.into_inner().unwrap();
        let mut vec: Vec<String> = Vec::new();
        for (i, module) in self.module_order.split_whitespace().enumerate() {
            match module {
                "delimiter" => vec.push(
                    self.delimiter
                        .get_info(measure_text_width(&vec[i - 1]))
                        .replace('\n', " "),
                ),
                v => {
                    let error = format!("Unknown module: {v}");
                    vec.push(
                        handle_error!(
                            modules_unordered.get(&String::from(v)).ok_or(""),
                            error,
                            "Make sure you have this module defined"
                        )
                        .to_string(),
                    );
                }
            }
        }
        vec
    }

    #[cfg(target_os = "macos")]
    fn logo() -> Vec<String> {
        include_str!("logos/macos")
            .lines()
            .map(str::to_string)
            .collect()
    }

    #[cfg(target_os = "linux")]
    fn logo() -> Vec<String> {
        let os = crate::modules::Os::get_os();
        match os.trim() {
            "Arch Linux" => include_str!("logos/arch_linux")
                .lines()
                .map(str::to_string)
                .collect(),
            x if x.contains("Ubuntu") => include_str!("logos/ubuntu_linux")
                .lines()
                .map(str::to_string)
                .collect(),
            x if x.contains("Debian") => {
                include_str!("logos/debian")
                    .lines()
                    .map(str::to_string)
                    .collect()
            }
            v => {
                UserFacingError::new(format!("Unknown OS: {}", v))
                    .help("Please file a new issue on github to request support for a new OS: https://github.com/devanlooches/rocketfetch/issues/new")
                    .print_and_exit();
                unreachable!()
            }
        }
    }

    #[cfg(target_os = "windows")]
    fn logo() -> Vec<String> {
        include_str!("logos/windows")
            .lines()
            .map(str::to_string)
            .collect()
    }

    fn get_logo(&self) -> Vec<String> {
        if self.logo_cmd.is_empty() || self.logo_cmd == "auto" {
            Self::logo()
        } else {
            Self::run_cmd(&self.logo_cmd, "Failed to run logo command")
                .lines()
                .map(str::to_string)
                .collect::<Vec<String>>()
        }
    }

    fn wrap_lines(offset: usize, module_order: &[String], logo_maxlength: usize) -> Vec<String> {
        let terminal_width =
            handle_error!(any_terminal_size().ok_or(""), "Failed to get terminal size")
                .0
                 .0 as usize;
        let mut module_order_wrapped = Vec::new();
        for module in module_order {
            let options = textwrap::Options::new(terminal_width - offset - logo_maxlength)
                .break_words(true)
                .word_separator(textwrap::WordSeparator::UnicodeBreakProperties);
            let module_wrapped = textwrap::wrap(module, &options);
            module_wrapped
                .into_iter()
                .for_each(|x| module_order_wrapped.push(x));
        }

        module_order_wrapped
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
    }

    fn print_classic(&self, wrap_lines: bool) {
        let mut sidelogo = self.get_logo();
        let mut order = self.get_module_order();
        let maxlength = self.logo_maxlength();
        if self.wrap_lines && wrap_lines {
            order = Self::wrap_lines(self.offset, &order, maxlength);
        }
        match sidelogo.len().cmp(&order.len()) {
            Ordering::Greater => order.resize(sidelogo.len(), String::new()),
            Ordering::Less => sidelogo.resize(order.len(), String::new()),
            Ordering::Equal => (),
        }

        for (i, line) in sidelogo.iter().enumerate() {
            println!(
                "{}{}{}",
                line,
                " ".repeat(maxlength - measure_text_width(line) + self.offset),
                &order[i]
            );
        }
    }

    pub fn run_cmd(cmd: &str, error_msg: &str) -> String {
        use std::process::Command;
        let output = if cfg!(target_os = "windows") {
            let command_run = Command::new("cmd").args(["/C", cmd]).output();
            handle_error!(command_run, error_msg)
        } else {
            let command_run = Command::new("sh").args(["-c", cmd]).output();
            handle_error!(command_run, error_msg)
        }
        .stdout;
        handle_error!(
            String::from_utf8(output.clone()),
            "Failed to read stdout from command"
        )
        .trim()
        .to_string()
    }

    fn logo_maxlength(&self) -> usize {
        if let Some(v) = self
            .get_logo()
            .iter()
            .max_by_key(|&x| measure_text_width(x))
        {
            return measure_text_width(v);
        }
        UserFacingError::new("Failed to find logo line with greatest length")
            .help("Make sure that the logo as at least one line.")
            .print_and_exit();
        unreachable!()
    }

    fn info_maxlength(info: &[String]) -> usize {
        if let Some(v) = info.iter().max_by_key(|&x| measure_text_width(x)) {
            return measure_text_width(v);
        }
        UserFacingError::new("Failed to find info line with the greatest length")
            .help("Make sure that you have some modules defined.")
            .print_and_exit();
        unreachable!()
    }

    fn print_side_block(&self, wrap_lines: bool) {
        let mut sidelogo = self.get_logo();
        let mut info = self.get_module_order();
        let logo_maxlength = self.logo_maxlength();
        if self.wrap_lines && wrap_lines {
            info = Self::wrap_lines(
                self.offset + self.format.padding_top + self.format.padding_left + 1 + 2,
                &info,
                logo_maxlength,
            );
        }
        if (sidelogo.len()).cmp(&(info.len() + self.format.padding_top + 2)) == Ordering::Less {
            sidelogo.resize(info.len() + self.format.padding_top + 2, String::new());
        }

        let mut counter = 0;

        let info_maxlength = Self::info_maxlength(&info);

        println!(
            "{}{}{}{}{}",
            sidelogo[0],
            " ".repeat(logo_maxlength - measure_text_width(&sidelogo[0]) + self.offset),
            self.format.top_left_corner_char,
            self.format
                .horizontal_char
                .to_string()
                .repeat(info_maxlength + self.format.padding_left + self.format.padding_right),
            self.format.top_right_corner_char,
        );
        counter += 1;

        for _ in 0..self.format.padding_top {
            println!(
                "{}{}{vertical}{}{vertical}",
                sidelogo[counter],
                " ".repeat(logo_maxlength - measure_text_width(&sidelogo[counter]) + self.offset),
                " ".repeat(info_maxlength + self.format.padding_right + self.format.padding_left),
                vertical = self.format.vertical_char
            );
            counter += 1;
        }

        for i in &info {
            println!(
                "{}{}{vertical}{}{}{}{}{vertical}",
                sidelogo[counter],
                " ".repeat(logo_maxlength - measure_text_width(&sidelogo[counter]) + self.offset),
                " ".repeat(self.format.padding_left),
                i,
                " ".repeat(self.format.padding_right),
                " ".repeat(info_maxlength - measure_text_width(i)),
                vertical = self.format.vertical_char
            );
            counter += 1;
        }

        println!(
            "{}{}{}{}{}",
            sidelogo[counter],
            " ".repeat(logo_maxlength - measure_text_width(&sidelogo[counter]) + self.offset),
            self.format.bottom_left_corner_char,
            self.format
                .horizontal_char
                .to_string()
                .repeat(info_maxlength + self.format.padding_left + self.format.padding_right),
            self.format.bottom_right_corner_char,
        );
        counter += 1;

        sidelogo.iter().skip(counter).for_each(|i| {
            println!("{i}");
        });
    }

    fn print_bottom_block(&self, wrap_lines: bool) {
        let sidelogo = self.get_logo();
        let mut info = self.get_module_order();
        if self.wrap_lines && wrap_lines {
            info = Self::wrap_lines(self.offset, &info, 0);
        }
        let info_maxlength = Self::info_maxlength(&info);

        for line in sidelogo {
            println!("{line}");
        }
        println!(
            "{}{}{}",
            self.format.top_left_corner_char,
            self.format
                .horizontal_char
                .to_string()
                .repeat(info_maxlength + self.format.padding_right + self.format.padding_left),
            self.format.top_right_corner_char
        );
        for _ in 0..self.format.padding_top {
            println!(
                "{vertical}{}{vertical}",
                " ".repeat(info_maxlength + self.format.padding_right + self.format.padding_left),
                vertical = self.format.vertical_char
            );
        }
        for line in info {
            println!(
                "{vertical}{}{}{}{}{vertical}",
                " ".repeat(self.format.padding_left),
                line,
                " ".repeat(info_maxlength - measure_text_width(&line)),
                " ".repeat(self.format.padding_right),
                vertical = self.format.vertical_char
            );
        }
        println!(
            "{}{}{}",
            self.format.bottom_left_corner_char,
            self.format
                .horizontal_char
                .to_string()
                .repeat(info_maxlength + self.format.padding_left + self.format.padding_right),
            self.format.bottom_right_corner_char
        );
    }

    pub fn print(&self) {
        let matches = Self::get_args();
        let wrap_lines = !matches.no_line_wrap;
        matches.mode.map_or_else(
            || match self.format.mode {
                Mode::Classic => self.print_classic(wrap_lines),
                Mode::BottomBlock => self.print_bottom_block(wrap_lines),
                Mode::SideBlock => self.print_side_block(wrap_lines),
            },
            |v| match v {
                Mode::Classic => self.print_classic(wrap_lines),
                Mode::BottomBlock => self.print_bottom_block(wrap_lines),
                Mode::SideBlock => self.print_side_block(wrap_lines),
            },
        );
    }
}

impl Default for Config {
    #[cfg(not(target_os = "windows"))]
    fn default() -> Self {
        Self {
            offset: 4,
            wrap_lines: true,
            module_order: String::from(
                "user delimiter os host kernel uptime packages shell resolution desktop-environment window-manager terminal cpu",
            ),
            logo_cmd: String::from("auto"),
            format: Format::default(),
            user: User::default(),
            delimiter: Delimiter::default(),
            os: Os::default(),
            host: Host::default(),
            kernel: Kernel::default(),
            uptime: Uptime::default(),
            custom_modules: HashMap::new(),
            packages: Packages::default(),
            shell: Shell::default(),
            resolution: Resolution::default(),
            desktop_environment: DesktopEnvironment::default(),
            window_manager: WindowManager::default(),
            terminal: Terminal::default(),
            cpu: Cpu::default(),
        }
    }

    #[cfg(target_os = "windows")]
    fn default() -> Self {
        Self {
            offset: 4,
            wrap_lines: true,
            module_order: String::from("user delimiter os host kernel uptime packages cpu"),
            logo_cmd: String::from("auto"),
            format: Format::default(),
            user: User::default(),
            delimiter: Delimiter::default(),
            os: Os::default(),
            host: Host::default(),
            kernel: Kernel::default(),
            uptime: Uptime::default(),
            custom_modules: HashMap::new(),
            packages: Packages::default(),
            shell: Shell::default(),
            resolution: Resolution::default(),
            desktop_environment: DesktopEnvironment::default(),
            window_manager: WindowManager::default(),
            terminal: Terminal::default(),
            cpu: Cpu::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::Config;

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn check_default_config() {
        let config = Config::from_config(String::from("config.toml"));
        assert_eq!(Config::default(), config);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn check_default_config() {
        let config = Config::from_config(String::from("windows_default.toml"));
        assert_eq!(Config::default(), config);
    }

    #[test]
    fn check_os() {
        println!("\n\n{}", Config::default().get_logo().join("\n"));
    }

    #[test]
    fn check_classic_print() {
        let config = Config {
            module_order: String::from(
                "user user user user user user user user user user user user",
            ),
            wrap_lines: false,
            ..Config::default()
        };
        config.print_classic(false);
    }

    #[test]
    fn check_classic_print_longer_info() {
        let config = Config {
            module_order: String::from("user user user user user user user user user user user user user user user user user user user user user user user"),
            wrap_lines: false,
            ..Config::default()
        };
        config.print_classic(false);
    }

    #[test]
    fn check_classic_print_longer_logo() {
        let config = Config {
            module_order: String::from(
                "user user user user user user user user user user user user user",
            ),
            wrap_lines: false,
            ..Config::default()
        };
        config.print_classic(false);
    }

    #[test]
    fn check_side_block_print() {
        let config = Config {
            module_order: String::from(
                "user user user user user user user user user user user user user",
            ),
            wrap_lines: false,
            ..Config::default()
        };
        config.print_side_block(false);
    }

    #[test]
    fn check_side_block_print_longer_info() {
        let config = Config {
            module_order: String::from("user user user user user user user user user user user user user user user user user user user user user user user"),
            wrap_lines: false,
            ..Config::default()
        };
        config.print_side_block(false);
    }

    #[test]
    fn check_side_block_print_longer_logo() {
        let config = Config {
            module_order: String::from("user"),
            wrap_lines: false,
            ..Config::default()
        };
        config.print_side_block(false);
    }

    #[test]
    fn check_bottom_block_print() {
        let config = Config {
            module_order: String::from(
                "user user user user user user user user user user user user user user",
            ),
            wrap_lines: false,
            ..Config::default()
        };
        config.print_bottom_block(false);
    }

    #[test]
    fn check_bottom_block_print_longer_info() {
        let config = Config {
            module_order: String::from("user user user user user user user user user user user user user user user user user user user user user user user"),
            wrap_lines: false,
            ..Config::default()
        };
        config.print_bottom_block(false);
    }

    #[test]
    fn check_bottom_block_print_longer_logo() {
        let config = Config {
            module_order: String::from("user"),
            wrap_lines: false,
            ..Config::default()
        };
        config.print_bottom_block(false);
    }
}
