# VTT to SRT Converter

## Overview
This program converts WebVTT (`.vtt`) subtitle files into SubRip (`.srt`) format. It sorts and deduplicates the subtitle entries based on timestamps and content. This utility is particularly useful for merging multiple VTT files into a single SRT file with clean and ordered subtitles.

## Installation

### Prerequisites
- Rust Programming Language: Ensure Rust is installed on your system. You can download it and find installation instructions at [Rust's official website](https://www.rust-lang.org/tools/install).

### Clone the Repository
First, clone the repository to your local machine using Git:

```bash
git clone https://github.com/jwu/vtt2srt
cd vtt-to-srt-converter
```

### Build the Project
Compile the project using Cargo, Rust's package manager and build system:

```bash
cargo build --release
```

The executable will be located in `./target/release/`.

### Usage
Run the program by specifying the path to the folder containing the VTT files:

```bash
./target/release/vtt2srt /path/to/your/vtt_files_folder
```
Replace `/path/to/your/vtt_files_folder` with the actual path to your folder containing .vtt files.

### Features
- Conversion: Converts all VTT files in the specified folder to SRT format.
- Sorting: Sorts subtitle entries by their start time.
- Deduplication: Removes duplicate entries having the same timestamp and content.

### Contributing
Contributions are welcome! If you have suggestions or improvements, please fork the repository and submit a pull request.

### Support
If you encounter any issues or have questions about using the program, please file an issue in the GitHub repository.

### License
This project is licensed under the MIT License - see the LICENSE file for details.
