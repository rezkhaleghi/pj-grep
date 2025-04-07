# pjgrep - A Simple Pattern Search Tool for Files and Directories

[`pjgrep`](command:_github.copilot.openSymbolFromReferences?%5B%22%22%2C%5B%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Frez%2FDesktop%2Fpj-grep%2FREADME.md%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A0%2C%22character%22%3A2%7D%7D%5D%2C%22d86cd89c-dc10-45b2-a4a7-48b9a80f2b9e%22%5D "Go to definition") is a fast and flexible pattern searching tool designed to search for specific patterns within files and directories. It supports searching for patterns in filenames and file contents and allows you to filter by file extensions. This tool is lightweight, highly customizable, and works cross-platform.

## Features

- Search for patterns in **file names** and **file content**.
- Support for filtering files by extension (e.g., `.txt`, `.md`, `.rs`).
- Provides **counts** for how many times a pattern appears in the content.
- Color-coded output for better readability.
- Cross-platform support (Linux, macOS, Windows).
- Fast search through directories and subdirectories.

## Installation

To use [`pjgrep`](command:_github.copilot.openSymbolFromReferences?%5B%22%22%2C%5B%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Frez%2FDesktop%2Fpj-grep%2FREADME.md%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A0%2C%22character%22%3A2%7D%7D%5D%2C%22d86cd89c-dc10-45b2-a4a7-48b9a80f2b9e%22%5D "Go to definition"), you need to have [Rust](https://www.rust-lang.org/) installed. You can install it using the following steps:

1. **Install Rust** (if not already installed):

   - Follow the instructions on the official [Rust installation page](https://www.rust-lang.org/tools/install).
   - On most systems, you can install it via the following command:
     ```sh
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

2. **Clone the Repository**:
   Clone this repository to your local machine:

   ```sh
   git clone https://github.com/rezkhaleghi/pj-grep.git
   cd pj-grep
   ```

3. **Build the Project**:
   Build the pjgrep application with the following command:

   ```sh
   cargo build --release
   ```

4. **Install the Binary**:
   Optionally, you can copy the binary to a directory in your `$PATH` (e.g., `/usr/local/bin` or `~/bin`) for easy access:

   ```sh
   sudo cp target/release/pjgrep /usr/local/bin/pjgrep
   ```

## Usage

```sh
pjgrep <pattern> [directory] [--format=<ext1,ext2,...>]
```

- `<pattern>`: The pattern to search for.
- `[directory]`: The directory to search in (optional, defaults to the current directory).
- `[--format=<txt,json,ts,env,...>]`: A comma-separated list of file extensions to filter by (optional).

### Examples

#### Search for a pattern in the current directory

```sh
pjgrep "example"
```

#### Search for a pattern in a specific directory

```sh
pjgrep "example" /path/to/directory
```

#### Search for a pattern in files with specific extensions

```sh
pjgrep "example" /path/to/directory --format=rs,txt,md
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## Author

Created and maintained by "PocketJack (Rez Khaleghi)"

- GitHub: https://github.com/rezkhaleghi
- Email: rezaxkhaleghi@gmail.com

## Support

If you enjoy this app and would like to support my work:

- Patreon : https://patreon.com/PocketJack
  Your support helps me continue developing free and open-source stuff.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgements

- [Rust Programming Language](https://www.rust-lang.org/)

# pj-grep
