# xp3-tool

A simple set of tools for packing and unpacking XP3 archives, which are commonly used in visual novel engines like Kirikiri and Kirikiroid2.

## Overview

This project contains two components:
- **xp3-packer**: Packs a directory into an XP3 archive.
- **xp3-unpacker**: Extracts the contents of an XP3 archive into a directory.

## Prerequisites

- **Rust & Cargo:**  
  Make sure you have [Rust](https://rustup.rs/) installed. Cargo, Rust’s package manager, is included with the installation.

## Building the Project

1. **Clone the Repository:**
   ```bash
   git clone <repository-url>
   cd storycraft-xp3-tool
   ```

2. **Build in Release Mode:**
   ```bash
   cargo build --release
   ```
   This will generate the executables in the `target/release/` directory:
   - On Linux/macOS: `xp3-packer` and `xp3-unpacker`
   - On Windows: `xp3-packer.exe` and `xp3-unpacker.exe`

## Usage

### Packing an XP3 Archive

To pack a directory into an XP3 archive, run:
```bash
./target/release/xp3-packer <input_dir> <output_xp3>
```
**Example:**
```bash
./target/release/xp3-packer game_files game_archive.xp3
```

### Unpacking an XP3 Archive

To extract an XP3 archive into a directory, run:
```bash
./target/release/xp3-unpacker <input_xp3> <output_dir>
```
**Example:**
```bash
./target/release/xp3-unpacker game_archive.xp3 extracted_files
```

## License

This project is licensed under the MIT License.

## Contributing

Contributions, bug reports, and feature requests are welcome. Please fork the repository and submit a pull request with your improvements.

## Acknowledgments

This tool was developed to help modders and translators work with XP3 archives found in many visual novel projects.
