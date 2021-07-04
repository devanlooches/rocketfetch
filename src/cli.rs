use structopt::StructOpt;
use user_error::UserFacingError;

#[derive(StructOpt)]
#[structopt(
    about = "A WIP command line system information tool (neofetch) rewritten in rust for performance with toml file configuration."
)]
pub struct Opt {
    #[structopt(
        short = "c",
        long,
        name = "FILE",
        help = "Sets custom configuration file."
    )]
    pub config: Option<String>,
    #[structopt(
        long,
        help = "Set the printing mode. Can be one of `side-table`, `bottom-table`, or `classic`"
    )]
    pub mode: Option<Mode>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    Classic,
    SideTable,
    BottomTable,
}

impl std::str::FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "classic" => Ok(Mode::Classic),
            "side-table" | "sidetable" => Ok(Mode::SideTable),
            "bottom-table" | "bottomtable" => Ok(Mode::BottomTable),
            v => Err(format!(
                "\n{}",
                UserFacingError::new("Unable to parse mode string")
                    .reason(format!("Unknown Mode: {}", v))
                    .help("Expected one of `side-table`, `bottom-table`, or `classic`")
                    .to_string()
            )),
        }
    }
}
