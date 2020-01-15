mod generate_options;
mod ron_format;

pub use generate_options::GenerateOptions;

use crate::opts::AmethystVersion;
use crate::PngData;
use ron_format::{RonWrapper, SpritesheetData};
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn generate_rons_for_pngs(
    pngs_data: Vec<PngData>,
    generate_options: GenerateOptions,
) -> Result<(), String> {
    if generate_options.verbose {
        let tile_size_s: String = generate_options.tile_size.into();
        eprintln!("Tile Size: {}", tile_size_s);
        eprintln!(
            "Pretty RON formatting?: {}",
            if generate_options.pretty {
                "TRUE"
            } else {
                "FALSE"
            }
        );
        eprintln!(
            "Generate for amethyst version: {}",
            generate_options.amethyst_version
        );
    }

    for png_data in pngs_data {
        let ron_file_path = {
            let dir = png_data.path.parent().unwrap_or(Path::new("."));
            let name = png_data
                .path
                .file_stem()
                .ok_or_else(|| {
                    String::from("PngData's path should have file name")
                })?
                .to_str()
                .ok_or_else(|| String::from("Couldn't convert &OsStr to &str"))?
                .to_string()
                + ".ron";
            dir.join(name)
        };

        if generate_options.verbose {
            eprintln!(
                "PNG    {:?}\nto RON {:?}",
                &png_data.path, &ron_file_path
            );
        }

        let mut spritesheet_data = SpritesheetData::from(png_data);
        spritesheet_data.gen_sprites_with_tile_size(generate_options.tile_size);

        let ron_s = {
            let ser_err_fn =
                |e| format!("Couldn't serialize spritesheet data: {}", e);

            match generate_options.amethyst_version {
                AmethystVersion::_0_13 => {
                    if generate_options.pretty {
                        let pretty_config = ron::ser::PrettyConfig::default();
                        ron::ser::to_string_pretty(
                            &spritesheet_data,
                            pretty_config,
                        )
                        .map_err(ser_err_fn)?
                    } else {
                        ron::ser::to_string(&spritesheet_data)
                            .map_err(ser_err_fn)?
                    }
                }
                AmethystVersion::Master => {
                    let wrapper = RonWrapper(spritesheet_data);
                    String::from("List")
                        + if generate_options.pretty {
                            let pretty_config =
                                ron::ser::PrettyConfig::default();
                            ron::ser::to_string_pretty(&wrapper, pretty_config)
                                .map_err(ser_err_fn)?
                        } else {
                            ron::ser::to_string(&wrapper).map_err(ser_err_fn)?
                        }
                        .as_str()
                }
            }
        };

        // TODO: add command-line option to set where to save generated
        //       RON file, relative to its PNG file
        let mut ron_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&ron_file_path)
            .map_err(|e| {
                format!(
                    "Couldn't open RON file for writing: {:?}\n{}",
                    &ron_file_path, e
                )
            })?;
        ron_file.write_all(ron_s.as_bytes()).map_err(|e| {
            format!("Couldn't write to RON file: {:?}\n{}", &ron_file_path, e)
        })?;
        ron_file.flush().map_err(|e| {
            format!("Couldn't flush file: {:?}\n{}", &ron_file_path, e)
        })?;
    }

    Ok(())
}
