# HyprTools

**A collection of lightweight Linux desktop utilities written in Rust, designed primarily for Hyprland workflows.**

HyprTools is a collection of small, independent applications that replace selected shell scripts with Rust implementations.

The goal is to build practical desktop utilities with minimal dependencies, predictable behavior, and maintainable code.

## Features

- Lightweight standalone Rust applications.
- Integration with existing Linux desktop tools.
- Rofi-based user interfaces.
- Error handling and automated tests.
- Automated CI and GitHub Releases.

## Project Structure

```text
hyprtools/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── crates/
│   ├── rofi-search/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   └── brightness-control/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
├── rofi-themes/
│   ├── config-search.rasi
│   └── config-brightness.rasi
└── .github/
    └── workflows/
        └── release.yml
```

Each application is maintained as an independent crate inside a shared Cargo workspace.

## Available Tools

| Tool                 | Description                                                       | Status      |
| -------------------- | ----------------------------------------------------------------- | ----------- |
| `rofi-search`        | Search the web or open URLs directly through Rofi.                | Implemented |
| `brightness-control` | Gradually adjust display brightness using Rofi and brightnessctl. | Implemented |
| `volume-control`     | Audio volume management.                                          | Planned     |

## Requirements

- Linux
- Rust and Cargo (only required when building from source)
- Rofi
- XDG utilities where required
- Hyprland configuration for `rofi-search`
- `brightnessctl` for `brightness-control`

## Installation

### Download a Release

Prebuilt binaries are available on the [GitHub Releases](https://github.com/dex0o0/hyprtools/releases) page.

Download the appropriate binary and make it executable:

```bash
chmod +x rofi-search-linux-x86_64-gnu
chmod +x brightness-control-linux-x86_64-gnu
```

You can then move the binaries to a directory in your `PATH`, for example:

```bash
mkdir -p ~/.local/bin

mv rofi-search-linux-x86_64-gnu ~/.local/bin/rofi-search
mv brightness-control-linux-x86_64-gnu ~/.local/bin/brightness-control
```

### Build from Source

Clone the repository:

```bash
git clone https://github.com/dex0o0/hyprtools.git
cd hyprtools
```

Build the entire workspace:

```bash
cargo build --release --workspace
```

The binaries will be available in:

```text
target/release/
```

## Tools Documentation

### 1. Rofi Search

A simple web search utility that uses Rofi as its input interface.

**Features:**

- Search the web using your configured search engine.
- Open HTTP and HTTPS URLs directly.
- Automatically recognize domain names and IP addresses.
- Encode search queries, including special characters and Persian text.
- Ignore empty input and handle cancelled Rofi sessions.

#### Configuration

The application reads the search engine URL from:

```text
~/.config/hypr/UserConfigs/user-defaults.lua
```

Add a `search_engine` setting containing the `{}` placeholder:

```lua
search_engine = "https://www.google.com/search?q={}"
```

You can use other search engines as well:

```lua
search_engine = "https://duckduckgo.com/?q={}"
```

#### Rofi Theme

The repository includes a Rofi theme:

```text
rofi-themes/config-search.rasi
```

Copy it to the expected configuration location:

```bash
mkdir -p ~/.config/rofi

cp rofi-themes/config-search.rasi \
   ~/.config/rofi/config-search.rasi
```

#### Usage

Run the application:

```bash
rofi-search
```

Or, when building from source:

```bash
cargo run --release -p rofi-search
```

Type a search query or website address into the Rofi input field.

Examples:

| Input               | Behavior                             |
| ------------------- | ------------------------------------ |
| `rust programming`  | Search the configured search engine. |
| `github.com`        | Open GitHub directly.                |
| `https://crates.io` | Open crates.io directly.             |
| `192.168.1.1`       | Open the IP address directly.        |

### 2. Brightness Control

A brightness management utility that uses Rofi for input and `brightnessctl` to control display brightness.

**Features:**

- Interactive brightness input through Rofi.
- Accepts percentage values from 0 to 100.
- Reads current and maximum brightness.
- Gradually adjusts brightness instead of changing it instantly.
- Handles invalid input and command failures.

#### Dependencies

- `rofi`
- `brightnessctl`

#### Rofi Theme

The repository includes a dedicated theme:

```text
rofi-themes/config-brightness.rasi
```

Install it:

```bash
mkdir -p ~/.config/rofi

cp rofi-themes/config-brightness.rasi \
   ~/.config/rofi/config-brightness.rasi
```

#### Usage

Run the application:

```bash
brightness-control
```

Or, when building from source:

```bash
cargo run --release -p brightness-control
```

Rofi will open an input field. Enter a brightness percentage between `0` and `100`.

For example:

```text
50
```

The application will gradually adjust the display brightness to the requested level.

**Note:** This tool does not accept brightness percentages as command-line arguments. The input is collected through Rofi.

## Development

HyprTools uses a Cargo workspace, allowing individual applications to be developed and built independently.

### Build Individual Tools

```bash
cargo build --release -p rofi-search
cargo build --release -p brightness-control
```

### Build Everything

```bash
cargo build --release --workspace
```

### Run Tests

```bash
cargo test --workspace
```

### Check Formatting

```bash
cargo fmt --all -- --check
```

### Run Clippy

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

## Continuous Integration

The project uses GitHub Actions to automatically validate changes.

The workflow runs on pushes to `master` and pull requests targeting `master`.

The CI pipeline performs:

1. Rust toolchain setup.
2. Formatting checks.
3. Clippy analysis.
4. Workspace tests.

All checks must pass before the release job can run.

## Releases

HyprTools uses Git tags to trigger automated releases.

To publish a new version:

```bash
git tag v0.3.0
git push origin master
git push origin v0.3.0
```

GitHub Actions will:

1. Run formatting checks.
2. Run Clippy.
3. Execute workspace tests.
4. Build release binaries.
5. Strip the binaries.
6. Publish them as GitHub Release assets.

Current release targets:

| Binary                                | Target           |
| ------------------------------------- | ---------------- |
| `rofi-search-linux-x86_64-gnu`        | Linux x86_64 GNU |
| `brightness-control-linux-x86_64-gnu` | Linux x86_64 GNU |

These binaries are built on Ubuntu 24.04 using the default GNU toolchain.

## Roadmap

### Core

- [x] Initialize the repository.
- [x] Set up the Cargo workspace.
- [x] Implement automated formatting checks.
- [x] Configure Clippy.
- [x] Add workspace tests.
- [ ] Establish consistent project conventions.

### Rofi Search

- [x] Read search engine configuration from Hyprland settings.
- [x] Implement Rofi-based search.
- [x] Detect and open URLs directly.
- [x] Support URL encoding.
- [x] Add unit tests.
- [ ] Improve configuration parsing.
- [ ] Support additional search engine options.

### Brightness Control

- [x] Implement Rofi-based percentage input.
- [x] Read current and maximum brightness.
- [x] Implement gradual brightness transitions.
- [x] Handle invalid input and command failures.
- [x] Add unit tests.
- [ ] Support additional brightness backends.

### Additional Utilities

- [ ] Implement volume control.
- [ ] Identify more useful shell scripts to rewrite in Rust.
- [ ] Add additional Hyprland workflow utilities.

### Build and Distribution

- [x] Automate CI with GitHub Actions.
- [x] Build Linux x86_64 release binaries.
- [x] Publish binaries through GitHub Releases.
- [ ] Add AArch64 Linux builds.
- [ ] Provide an installation script.
- [ ] Improve installation and upgrade documentation.

## Design Philosophy

**Simplicity:** Each utility should solve a specific problem without unnecessary complexity.

**Reliability:** Applications should handle errors explicitly and behave predictably.

**Maintainability:** Code should remain easy to understand, test, and extend.

**Independence:** Each tool should work independently without requiring the entire collection.

**Practicality:** Features should improve real desktop workflows.

## Contributing

Contributions, bug reports, and feature suggestions are welcome.

If you have an idea for a useful Linux desktop utility or want to improve an existing tool, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
