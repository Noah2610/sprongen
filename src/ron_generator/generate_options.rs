use crate::opts::AmethystVersion;
use crate::Size;

pub struct GenerateOptions {
    pub verbose:          bool,
    pub tile_size:        Size,
    pub pretty:           bool,
    pub amethyst_version: AmethystVersion,
}
