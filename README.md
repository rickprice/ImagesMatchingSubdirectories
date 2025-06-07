# Images Matching Time Periods

A Rust command-line tool for finding image files within specified subdirectories of a given directory.

## Description

This tool recursively searches for image files in specified subdirectories and provides flexible output options including random sampling and compact formatting.

## Features

- **Recursive Search**: Searches through all subdirectories within the specified folders
- **Image Format Support**: Supports common image formats (jpg, jpeg, png, gif, bmp, tiff, webp)
- **Random Sampling**: Optionally limit the number of displayed results with random selection
- **Flexible Output**: Choose between detailed listing or space-separated paths
- **Error Handling**: Provides warnings for non-existent or invalid subdirectories

## Installation

```bash
cargo build --release
```

## Usage

```bash
cargo run -- <directory> <subdirectory1> [subdirectory2] ... [OPTIONS]
```

### Arguments

- `<directory>`: The main directory to search in
- `<subdirectory1> [subdirectory2] ...`: Names of subdirectories within the main directory to search

### Options

- `-l, --limit <NUMBER>`: Maximum number of images to display (random selection)
- `-n, --names-only`: Print only image paths separated by spaces (suppresses other output)
- `-h, --help`: Print help information

## Examples

### Basic Usage

Find all images in specific subdirectories:
```bash
cargo run -- /home/user/photos 2023 2024
```

### Limit Results

Show only 5 randomly selected images:
```bash
cargo run -- /home/user/photos vacation work --limit 5
```

### Compact Output

Get space-separated paths (useful for scripting):
```bash
cargo run -- /home/user/photos family events --names-only
```

### Combined Options

Get 3 random image paths in compact format:
```bash
cargo run -- /home/user/photos 2023 2024 --limit 3 --names-only
```

## Output Formats

### Standard Output
```
Found 15 image(s):
  /home/user/photos/2023/vacation1.jpg
  /home/user/photos/2023/sunset.png
  /home/user/photos/2024/birthday.gif
  ...
```

### Names-Only Output
```
/home/user/photos/2023/vacation1.jpg /home/user/photos/2023/sunset.png /home/user/photos/2024/birthday.gif
```

## Supported Image Formats

- JPEG (jpg, jpeg)
- PNG (png)
- GIF (gif)
- BMP (bmp)
- TIFF (tiff)
- WebP (webp)

## Error Handling

- The tool will exit with an error if the main directory doesn't exist
- It will show warnings for subdirectories that don't exist or aren't directories
- It will continue processing other valid subdirectories even if some are invalid
