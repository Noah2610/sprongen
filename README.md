# `sprongen`
`Spr`itesheet  
`RON`  
`Gen`erator

## Description
Generates RON spritesheet config files for your spritesheet PNG images.  
The spritesheet configs are generated in the format used by the [`amethyst`][amethyst] engine.  
Generated RON files are placed next to the corresponding PNG image file. __(TODO: make this configurable)__

## Installation
### From [crates.io]
```
cargo install sprongen
```

### From source
Clone this repo ...
```
git clone https://github.com/Noah2610/sprongen
```
Install with `cargo` ...
```
cargo install --path sprongen/ --force
```

## Usage
```
sprongen --help
```

```
sprongen 0.0.2
Generate RON files for the given spritesheet PNG images

USAGE:
    sprongen [FLAGS] [OPTIONS] <FILES>...

FLAGS:
    -h, --help
            Prints help information

    -p, --pretty
            Pretty format the generated RON files.

            Without this, generated RON files will have no new-lines/spacing.
    -V, --version
            Prints version information

    -v, --verbose
            Enable verbose logging.

            Prints information about used options, what PNG files are read, and what RON files are being generated. Is
            printed to stderr.

OPTIONS:
    -A, --amethyst-version <amethyst-version>
            For which amethyst version to generate the RON files.

            Since after amethyst v0.13, amethyst reads spritesheet RON config files differently. See
            https://github.com/amethyst/amethyst/issues/1997 `sprongen` can generate the v0.13, and the master RON
            format. <amethyst-version> must be one of: "0.13", "master" [default: 0.13]
    -s, --tile-size <tile-size>
            Use the given tile size.

            <tile-size> format is `<width>x<height>`, where <width> and <height> are positive integers. [default: 32x32]

ARGS:
    <FILES>...
```

## Examples
Let's say we have a directory `spritesheets/` in which we have a bunch of  
PNG spritesheet images, for which we want to create RON config files.  
Our file structure may look something like this ...
```
spritesheets/
├── player.png
└── tiles.png
```

Some meta details about our example spritesheets:
- `player.png`  
  Sprites in this image all have the size `32x64` pixels.
- `tiles.png`  
  Sprites/tiles in this image all have the size `16x16` pixels.

Now, we want to generate RON config files, which we will use in our [`amethyst`][amethyst] game.  
These config files need to define each sprite's position and size in our spritesheets.

To generate the RON files, keeping our details listed above in mind, use `sprongen` like so ...
```
sprongen -vp --tile-size 32x64 spritesheets/player.png
sprongen -vp --tile-size 16x16 spritesheets/tiles.png
```

<details>
<summary>Explanation of used command-line options</summary>

- `-s` or `--tile-size`  
  Sets the target tile size, as we defined above.  
  For this example, this is the only necessary option.
- `-v` or `--verbose`  
  _Verbose logging_, outputs information about what's happening.
- `-p` or `--pretty`  
  _Pretty formatting_, generated RON files have pretty formatting,  
  containing new-lines and spacing for human-readability.
</details>

After this, our new file structure would look something like this ...
```
spritesheets/
├── player.png
├── player.ron
├── tiles.png
└── tiles.ron
```

## License
Distributed under the terms of the [MIT license][license].

[amethyst]:  https://github.com/amethyst/amethyst
[crates.io]: https://crates.io/crates/sprongen
[license]:   ./LICENSE
