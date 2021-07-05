use crate::cli::Mode;
use crate::cli::Opt;
use crate::modules::*;
use console::measure_text_width;
use console::style;
use rsys::Rsys;
use std::cmp::Ordering;
use structopt::StructOpt;
use user_error::{UserFacingError, UFE};

#[derive(Deserialize)]
#[serde(default)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub struct Config {
    module_order: String,
    offset: usize,
    logo_cmd: String,
    format: Format,
    user: User,
    delimiter: Delimiter,
}

impl Config {
    pub async fn get_args() -> Opt {
        Opt::from_args()
    }
    pub async fn from_config() -> Self {
        let matches = Config::get_args().await;
        let home_dir = match dirs::home_dir() {
            Some(v) => v,
            None => {
                UserFacingError::new("Failed to find home directory")
                    .help("If this persists, please open github issue.")
                    .print_and_exit();
                unreachable!()
            }
        };
        let path = matches.config.unwrap_or(format!(
            "{}/.config/rustfetch/config.toml",
            home_dir.to_string_lossy()
        ));
        match std::fs::read_to_string(path) {
            Ok(v) => match toml::from_str::<Config>(&v) {
                Ok(v) => v,
                Err(r) => {
                    UserFacingError::new("Failed to parse toml file.")
                        .reason(r.to_string())
                        .print_and_exit();
                    unreachable!()
                }
            },

            Err(r) => {
                println!(
                    "{}: Could not find default configuration: {}. Falling back to default configuration.",
                    style("WARNING").yellow(),
                    r.to_string()
                );
                Config::default()
            }
        }
    }

    async fn module_order(&self) -> Vec<String> {
        let mut vec = Vec::new();
        for (i, module) in self.module_order.split_whitespace().enumerate() {
            match module {
                "user" => vec.push(self.user.get_info().await),
                "delimiter" => vec.push(
                    self.delimiter
                        .get_info(measure_text_width(&vec[i - 1]))
                        .await,
                ),
                v => {
                    UserFacingError::new("Failed to parse module order string.")
                        .reason(format!("Unknown module: {}", v))
                        .print_and_exit();
                    unreachable!();
                }
            }
        }
        vec
    }

    async fn logo(os: String) -> Vec<String> {
        match os.trim() {
            "macos" => {
                vec![
                    format!("                 ,xNMM."),
                    format!("               .OMMMMo"),
                    format!("               OMMM0,"),
                    format!("     .;loddo:' loolloddol;."),
                    format!("   cKMMMMMMMMMMNWMMMMMMMMMM0:"),
                    format!(" .KMMMMMMMMMMMMMMMMMMMMMMMWd."),
                    format!(" XMMMMMMMMMMMMMMMMMMMMMMMX."),
                    format!(";MMMMMMMMMMMMMMMMMMMMMMMM:"),
                    format!(":MMMMMMMMMMMMMMMMMMMMMMMM:"),
                    format!(".MMMMMMMMMMMMMMMMMMMMMMMMX."),
                    format!(" kMMMMMMMMMMMMMMMMMMMMMMMMWd."),
                    format!(" .XMMMMMMMMMMMMMMMMMMMMMMMMMMk"),
                    format!("  .XMMMMMMMMMMMMMMMMMMMMMMMMK."),
                    format!("    kMMMMMMMMMMMMMMMMMMMMMMd"),
                    format!("     ;KMMMMMMMWXXWMMMMMMMk."),
                    format!("       .cooc,.    .,coo:."),
                ]
            }
            v => {
                UserFacingError::new(format!("Unknown OS: {}", v)).help("Please file a new issue on github to request a new OS.").print_and_exit();
                unreachable!()
            }
        }
    }

    async fn get_logo(&self) -> Vec<String> {
        if self.logo_cmd.is_empty() || self.logo_cmd == "auto" {
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
            Config::logo(os).await
        } else {
            Config::run_cmd(&self.logo_cmd)
                .await
                .lines()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
        }
    }

    async fn print_classic(&self) {
        let mut sidelogo = self.get_logo().await;
        let mut order = self.module_order().await;

        let maxlength = self.logo_maxlength().await;

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

    pub async fn run_cmd(cmd: &str) -> String {
        use std::process::Command;
        let output = if cfg!(target_os = "windows") {
            match Command::new("cmd").args(&["/C", cmd]).output() {
                Ok(v) => v,
                Err(r) => {
                    UserFacingError::new("Failed to execute command")
                        .reason(r.to_string())
                        .print_and_exit();
                    unreachable!()
                }
            }
        } else {
            match Command::new("sh").args(["-c", cmd]).output() {
                Ok(v) => v,
                Err(r) => {
                    UserFacingError::new("Failed to execute command")
                        .reason(r.to_string())
                        .print_and_exit();
                    unreachable!()
                }
            }
        };
        match String::from_utf8(output.stdout) {
            Ok(v) => v.trim().to_string(),
            Err(r) => {
                UserFacingError::new("Failed to read stdout from command.")
                    .reason(r.to_string())
                    .print_and_exit();
                unreachable!()
            }
        }
    }

    async fn logo_maxlength(&self) -> usize {
        match self
            .get_logo()
            .await
            .iter()
            .max_by_key(|&x| measure_text_width(x))
        {
            Some(v) => measure_text_width(v),
            None => {
                UserFacingError::new("Failed to find logo line with greatest length.")
                    .help("Make sure you have a logo command defined, and that it outputs something. If this persists, please open a github issue.")
                    .print_and_exit();
                unreachable!()
            }
        }
    }

    async fn info_maxlength(&self) -> usize {
        match self
            .module_order()
            .await
            .iter()
            .max_by_key(|&x| measure_text_width(x))
        {
            Some(v) => measure_text_width(v),
            None => {
                UserFacingError::new("Failed to find info line with the greatest length")
                    .help("Make sure that you have some modules defined. If this persists, please open a github issue.")
                    .print_and_exit();
                unreachable!()
            }
        }
    }

    async fn print_side_table(&self) {
        let mut sidelogo = self.get_logo().await;
        let mut info = self.module_order().await;
        match sidelogo.len().cmp(&info.len()) {
            Ordering::Greater => info.resize(sidelogo.len(), String::from("")),
            Ordering::Less => sidelogo.resize(info.len(), String::from("")),
            Ordering::Equal => (),
        }
        let logo_maxlength = self.logo_maxlength().await;
        let info_maxlength = self.info_maxlength().await;
        println!(
            "{}{}{}{}{}",
            &sidelogo[0],
            " ".repeat(logo_maxlength - measure_text_width(&sidelogo[0]) + self.offset),
            self.format.top_left_corner_char,
            self.format
                .horizontal_char
                .to_string()
                .repeat(info_maxlength + 2),
            self.format.top_right_corner_char,
        );

        for i in 0..info.len() - 2 {
            println!(
                "{}{}{vertical} {} {}{vertical}",
                sidelogo[i + 1],
                " ".repeat(logo_maxlength - measure_text_width(&sidelogo[i + 1]) + self.offset),
                info[i],
                " ".repeat(info_maxlength - measure_text_width(&info[i])),
                vertical = self.format.vertical_char
            );
        }
        let last = match sidelogo.last() {
            Some(v) => v,
            None => {
                UserFacingError::new("Failed to get last line of logo")
                    .help("If this persists, please open a github issue.")
                    .print_and_exit();
                unreachable!()
            }
        };
        println!(
            "{}{}{}{}{}",
            last,
            " ".repeat(logo_maxlength - measure_text_width(last) + self.offset),
            self.format.bottom_left_corner_char,
            self.format
                .horizontal_char
                .to_string()
                .repeat(info_maxlength + 2),
            self.format.bottom_right_corner_char,
        );
    }

    async fn print_bottom_table(&self) {
        let sidelogo = self.get_logo().await;
        let info = self.module_order().await;
        let info_maxlength = self.info_maxlength().await;

        for line in sidelogo {
            println!("{}", line);
        }
        for _ in 0..self.format.padding {
            println!();
        }
        println!(
            "{}{}{}",
            self.format.top_left_corner_char,
            self.format
                .horizontal_char
                .to_string()
                .repeat(info_maxlength + 2),
            self.format.top_right_corner_char
        );
        for line in info {
            println!(
                "{vertical} {} {}{vertical}",
                line,
                " ".repeat(info_maxlength - measure_text_width(&line)),
                vertical = self.format.vertical_char
            );
        }
        println!(
            "{}{}{}",
            self.format.bottom_left_corner_char,
            self.format
                .horizontal_char
                .to_string()
                .repeat(info_maxlength + 2),
            self.format.bottom_right_corner_char
        );
    }

    pub async fn print(&self) {
        let matches = Config::get_args().await;
        if let Some(v) = matches.mode {
            match v {
                Mode::Classic => self.print_classic().await,
                Mode::BottomTable => self.print_bottom_table().await,
                Mode::SideTable => self.print_side_table().await,
            }
        } else {
            match self.format.mode {
                Mode::Classic => self.print_classic().await,
                Mode::BottomTable => self.print_bottom_table().await,
                Mode::SideTable => self.print_side_table().await,
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            user: User::default(),
            offset: 4,
            module_order: String::from("user delimiter"),
            logo_cmd: String::from("auto"),
            format: Format::default(),
            delimiter: Delimiter::default(),
        }
    }
}
