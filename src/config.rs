use crate::cli::Mode;
use crate::cli::Opt;
use crate::handle_error;
use crate::modules::*;
use console::measure_text_width;
use console::style;
use console::Style;
use std::cmp::Ordering;
use std::collections::HashMap;
use structopt::StructOpt;
use user_error::{UserFacingError, UFE};

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case", default)]
pub struct Config {
    module_order: String,
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

    #[serde(flatten)]
    custom_modules: HashMap<String, Module>,
}

impl Config {
    pub fn get_args() -> Opt {
        Opt::from_args()
    }
    pub fn path() -> String {
        let matches = Config::get_args();
        let home_dir = handle_error!(dirs::home_dir().ok_or(""), "Failed to find home directory");
        let path = matches.config.unwrap_or(format!(
            "{}/.config/rocketfetch/config.toml",
            home_dir.to_string_lossy()
        ));
        path
    }
    pub fn from_config(path: String) -> Self {
        match std::fs::read_to_string(path) {
            Ok(string) => match toml::from_str::<Config>(&string) {
                Ok(v) => v,
                Err(r) => {
                    let mut line: u64 = 0;
                    let mut column: u64 = 0;
                    let mut last = String::new();
                    let r = r.to_string();
                    println!("{}", r);
                    let error = handle_error!(
                        r.split("at").last().ok_or(""),
                        "Failed to get line and column number of configuration error"
                    );
                    for word in error.split_whitespace() {
                        if last == "line" {
                            line = handle_error!(
                                word.parse::<u64>(),
                                "Failed to get line number of configuration error"
                            );
                        } else if last == "column" {
                            column = handle_error!(
                                word.parse::<u64>(),
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
                            col_len_sep = " ".repeat(column as usize),
                            line = line,
                            line_contents =
                                string.lines().collect::<Vec<&str>>()[(line - 1) as usize],
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
                Config::default()
            }
        }
    }

    fn module_order(&self) -> Vec<String> {
        use std::thread;
        let modules = self.module_order.split_whitespace().collect::<Vec<&str>>();
        let mut modules_unordered = HashMap::new();
        let mut handles = Vec::new();
        let self_clone = self.clone();
        let user = self_clone.user;
        let os = self_clone.os;
        let host = self_clone.host;
        let kernel = self_clone.kernel;
        let uptime = self_clone.uptime;
        let packages = self_clone.packages;
        let custom = self_clone.custom_modules;
        if modules.contains(&"user") {
            let handle = thread::spawn(move || (String::from("user"), user.get_info()));
            handles.push(handle);
        }
        if modules.contains(&"os") {
            let handle = thread::spawn(move || (String::from("os"), os.get_info()));
            handles.push(handle);
        }
        if modules.contains(&"host") {
            let handle = thread::spawn(move || (String::from("host"), host.get_info()));
            handles.push(handle);
        }
        if modules.contains(&"kernel") {
            let handle = thread::spawn(move || (String::from("kernel"), kernel.get_info()));
            handles.push(handle);
        }
        if modules.contains(&"uptime") {
            let handle = thread::spawn(move || (String::from("uptime"), uptime.get_info()));
            handles.push(handle);
        }
        if modules.contains(&"packages") {
            let handle = thread::spawn(move || (String::from("packages"), packages.get_info()));
            handles.push(handle);
        }
        for (name, module) in custom {
            let handle = thread::spawn(move || (name, module.get_info()));
            handles.push(handle);
        }
        for handle in handles {
            let joined_handle = handle.join().unwrap();
            modules_unordered.insert(joined_handle.0, joined_handle.1);
        }
        let mut vec: Vec<String> = Vec::new();
        for (i, module) in self.module_order.split_whitespace().enumerate() {
            match module {
                "user" => vec.push(
                    modules_unordered
                        .get(&String::from("user"))
                        .unwrap()
                        .to_string(),
                ),
                "delimiter" => vec.push(self.delimiter.get_info(measure_text_width(&vec[i - 1]))),
                "os" => vec.push(
                    modules_unordered
                        .get(&String::from("os"))
                        .unwrap()
                        .to_string(),
                ),
                "host" => vec.push(
                    modules_unordered
                        .get(&String::from("host"))
                        .unwrap()
                        .to_string(),
                ),
                "kernel" => vec.push(
                    modules_unordered
                        .get(&String::from("kernel"))
                        .unwrap()
                        .to_string(),
                ),
                "uptime" => vec.push(
                    modules_unordered
                        .get(&String::from("uptime"))
                        .unwrap()
                        .to_string(),
                ),
                "packages" => vec.push(
                    modules_unordered
                        .get(&String::from("packages"))
                        .unwrap()
                        .to_string(),
                ),
                v => {
                    let error = format!("Unknown module: {}", v);
                    let module = handle_error!(
                        modules_unordered.get(v).ok_or(""),
                        error,
                        "Make sure you have this module defined."
                    );
                    vec.push(module.to_string());
                }
            }
        }
        vec
    }

    fn logo(&self) -> Vec<String> {
        let os = self.os.get_os();
        match os.trim() {
            x if x.contains("macOS") => {
                let yellow = Style::from_dotted_str("yellow.bold");
                let red = Style::from_dotted_str("red.bold");
                let blue = Style::from_dotted_str("blue.bold");
                let green = Style::from_dotted_str("green.bold");
                let purple = Style::new().color256(5).bold();
                vec![
                    green.apply_to("                 ,xNMM.").to_string(),
                    green.apply_to("               .OMMMMo").to_string(),
                    green.apply_to("               OMMM0,").to_string(),
                    yellow.apply_to("     .;loddo:' loolloddol;.").to_string(),
                    yellow.apply_to("   cKMMMMMMMMMMNWMMMMMMMMMM0:").to_string(),
                    yellow.apply_to(" .KMMMMMMMMMMMMMMMMMMMMMMMWd.").to_string(),
                    yellow.apply_to(" XMMMMMMMMMMMMMMMMMMMMMMMX.").to_string(),
                    yellow.apply_to(";MMMMMMMMMMMMMMMMMMMMMMMM:").to_string(),
                    red.apply_to(":MMMMMMMMMMMMMMMMMMMMMMMM:").to_string(),
                    red.apply_to(".MMMMMMMMMMMMMMMMMMMMMMMMX.").to_string(),
                    red.apply_to(" kMMMMMMMMMMMMMMMMMMMMMMMMWd.").to_string(),
                    red.apply_to(" .XMMMMMMMMMMMMMMMMMMMMMMMMMMk").to_string(),
                    purple
                        .apply_to("  .XMMMMMMMMMMMMMMMMMMMMMMMMK.")
                        .to_string(),
                    purple.apply_to("    kMMMMMMMMMMMMMMMMMMMMMMd").to_string(),
                    blue.apply_to("     ;KMMMMMMMWXXWMMMMMMMk.").to_string(),
                    blue.apply_to("       .cooc,.    .,coo:.").to_string(),
                ]
            }
            "Arch Linux" => {
                let lightblue = Style::new().blue().bright();
                vec![
                    "                     -`                 ",
                    "                    .o+`                ",
                    "                   `ooo/                ",
                    "                  `+oooo:               ",
                    "                 `+oooooo:              ",
                    "                 -+oooooo+:             ",
                    "               `/:-:++oooo+:            ",
                    "              `/++++/+++++++:           ",
                    "             `/++++++++++++++:          ",
                    "            `/+++ooooooooooooo/`        ",
                    "           ./ooosssso++osssssso+`       ",
                    "          .oossssso-````/ossssss+`      ",
                    "         -osssssso.      :ssssssso.     ",
                    "        :osssssss/        osssso+++.    ",
                    "       /ossssssss/        +ssssooo/-    ",
                    "     `/ossssso+/:-        -:/+osssso+-  ",
                    "    `+sso+:-`                 `.-/+oso: ",
                    "   `++:.                           `-/+/",
                    "   .`                                 `/",
                ]
                .iter()
                .map(|&x| lightblue.apply_to(x).to_string())
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

    fn get_logo(&self) -> Vec<String> {
        if self.logo_cmd.is_empty() || self.logo_cmd == "auto" {
            self.logo()
        } else {
            Config::run_cmd(&self.logo_cmd)
                .lines()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        }
    }

    fn print_classic(&self) {
        let mut sidelogo = self.get_logo();
        let mut order = self.module_order();

        let maxlength = self.logo_maxlength();

        match sidelogo.len().cmp(&order.len()) {
            Ordering::Greater => order.resize(sidelogo.len(), String::from("")),
            Ordering::Less => sidelogo.resize(order.len(), String::from("")),
            Ordering::Equal => (),
        }
        for (i, line) in sidelogo.iter().enumerate() {
            println!(
                "{}{}{}",
                line,
                " ".repeat(maxlength - measure_text_width(line) + self.offset),
                order[i]
            );
        }
    }

    pub fn run_cmd(cmd: &str) -> String {
        use std::process::Command;
        let output = if cfg!(target_os = "windows") {
            let command_run = Command::new("cmd").args(&["/C", cmd]).output();
            handle_error!(command_run, "Failed to run command")
        } else {
            let command_run = Command::new("sh").args(["-c", cmd]).output();
            handle_error!(command_run, "Failed to run command")
        };
        handle_error!(
            String::from_utf8(output.stdout.clone()),
            "Failed to read stdout from command"
        )
        .trim()
        .to_string()
    }

    fn logo_maxlength(&self) -> usize {
        match self
            .get_logo()
            .iter()
            .max_by_key(|&x| measure_text_width(x))
        {
            Some(v) => measure_text_width(v),
            None => {
                UserFacingError::new("Failed to find logo line with greatest length")
                    .print_and_exit();
                unreachable!()
            }
        }
    }

    fn info_maxlength(info: &[String]) -> usize {
        match info.iter().max_by_key(|&x| measure_text_width(x)) {
            Some(v) => measure_text_width(v),
            None => {
                UserFacingError::new("Failed to find info line with the greatest length")
                    .help("Make sure that you have some modules defined.")
                    .print_and_exit();
                unreachable!()
            }
        }
    }

    fn print_side_table(&self) {
        let mut sidelogo = self.get_logo();
        let mut info = self.module_order();
        let mut counter = 0;
        info.resize(sidelogo.len() + self.format.padding_top, String::from(""));
        sidelogo.resize(info.len() + self.format.padding_top, String::from(""));

        let logo_maxlength = self.logo_maxlength();
        let info_maxlength = Config::info_maxlength(&info);

        println!(
            "{}{}{}{}{}",
            handle_error!(
                sidelogo.first().ok_or(""),
                "Failed to get first line of sidelogo"
            ),
            " ".repeat(logo_maxlength - measure_text_width(&sidelogo[0]) + self.offset),
            self.format.top_left_corner_char,
            self.format
                .horizontal_char
                .to_string()
                .repeat(info_maxlength + self.format.padding_left + self.format.padding_right),
            self.format.top_right_corner_char,
        );
        counter += 1;

        for i in 0..self.format.padding_top {
            println!(
                "{}{}{vertical}{}{vertical}",
                sidelogo[i + counter],
                " ".repeat(
                    logo_maxlength - measure_text_width(&sidelogo[i + counter]) + self.offset
                ),
                " ".repeat(info_maxlength + self.format.padding_right + self.format.padding_left),
                vertical = self.format.vertical_char
            );
            counter += 1;
        }

        for i in info.iter().take(info.len() - 2) {
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
    }

    fn print_bottom_table(&self) {
        let sidelogo = self.get_logo();
        let info = self.module_order();
        let info_maxlength = Config::info_maxlength(&info);

        for line in sidelogo {
            println!("{}", line);
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
            )
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
        let matches = Config::get_args();
        if let Some(v) = matches.mode {
            match v {
                Mode::Classic => self.print_classic(),
                Mode::BottomBlock => self.print_bottom_table(),
                Mode::SideBlock => self.print_side_table(),
            }
        } else {
            match self.format.mode {
                Mode::Classic => self.print_classic(),
                Mode::BottomBlock => self.print_bottom_table(),
                Mode::SideBlock => self.print_side_table(),
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            offset: 4,
            module_order: String::from("user delimiter os host kernel uptime packages"),
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
        }
    }
}

#[cfg(test)]
mod test {
    use super::Config;
    use pretty_assertions::assert_eq;

    #[test]
    fn check_default_config() {
        let config = Config::from_config(String::from("config.toml"));
        assert_eq!(Config::default(), config);
    }
}
