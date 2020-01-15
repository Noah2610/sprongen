use std::fmt;
use std::str::FromStr;

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
