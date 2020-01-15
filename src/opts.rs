use crate::meta;
use crate::size::Size;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Clone, StructOpt, Debug)]
#[structopt(
    name = meta::NAME,
    version = meta::VERSION,
    about = meta::DESCRIPTION,
)]
pub struct Opts {
    /// Enable verbose logging.
    ///
    /// Prints information about used options,
    /// what PNG files are read,
    /// and what RON files are being generated.
    /// Is printed to stderr.
    #[structopt(short, long)]
    pub verbose: bool,

    /// Pretty format the generated RON files.
    ///
    /// Without this, generated RON files will have no new-lines/spacing.
    #[structopt(short, long)]
    pub pretty: bool,

    /// Use the given tile size.
    ///
    /// <tile-size> format is `<width>x<height>`,
    /// where <width> and <height> are positive integers.
    #[structopt(short = "s", long, default_value = "32x32")]
    pub tile_size: Size,

    /// For which amethyst version to generate the RON files.
    ///
    /// Since after amethyst v0.13, amethyst reads spritesheet RON config files differently.
    /// See https://github.com/amethyst/amethyst/issues/1997
    /// `sprongen` can generate the v0.13, and the master RON format.
    /// <amethyst-version> must be one of:
    ///   "0.13", "master"
    #[structopt(
        short = "A",
        long,
        default_value = "0.13",
        parse(try_from_str)
    )]
    pub amethyst_version: AmethystVersion,

    #[structopt(name = "FILES", multiple = true, required = true)]
    pub files: Vec<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum AmethystVersion {
    _0_13,
    Master,
}

impl FromStr for AmethystVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "0.13" => Ok(AmethystVersion::_0_13),
            "master" => Ok(AmethystVersion::Master),
            _ => Err(String::from(
                "--amethyst-version must be one of: '0.13' or 'master'",
            )),
        }
    }
}

impl fmt::Display for AmethystVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AmethystVersion::_0_13 => "0.13",
                AmethystVersion::Master => "master",
            }
        )
    }
}
