# HyprTools

**A collection of lightweight Linux desktop utilities built with Rust, primarily designed for Hyprland.**

HyprTools replaces selected shell scripts with small, standalone Rust applications. The project uses a Cargo workspace to keep each utility independent while sharing build, test, and release workflows.

## Goals

- Replace useful Bash scripts with maintainable Rust applications.
- Improve error handling and reliability.
- Keep runtime dependencies minimal.
- Integrate with existing Linux desktop utilities.
- Build and distribute standalone binaries through GitHub Releases.

## Project Structure

HyprTools uses a Cargo Workspace to organize multiple independent applications.

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
└── .github/
    └── workflows/
        └── release.yml
```

Each application is an independent crate and can be built separately with Cargo.

## Tools

| Tool                 | Description                                                     | Status      |
| -------------------- | --------------------------------------------------------------- | ----------- |
| `rofi-search`        | Search the web or open website addresses directly through Rofi. | Implemented |
| `brightness-control` | Gradually change display brightness to a requested percentage.  | Implemented |
| `volume-control`     | Audio volume management utility.                                | Planned     |

## Requirements

- Linux
- Rust toolchain and Cargo (for building from source)
- Required runtime utilities for each application

### rofi-search

Runtime requirements:

- `rofi`
- `xdg-open`
- `notify-send` (optional; used to display errors)
- Hyprland configuration file at `~/.config/hypr/UserConfigs/user-defaults.lua`
- Rofi theme at `~/.config/rofi/config-search.rasi`

The configuration must contain a `search_engine` setting with the `{}` placeholder.

Example:

```lua
search_engine = "https://www.google.com/search?q={}"
```

Run the application:

```bash
cargo run --release -p rofi-search
```

### brightness-control

Runtime requirement:

- `brightnessctl`

Pass the target brightness as an integer percentage from 0 to 100.

Example:

```bash
brightness-control 50
```

Run directly from the workspace:

```bash
cargo run --release -p brightness-control -- 50
```

## Building

### Build an individual tool

```bash
cargo build --release -p brightness-control
```

Or:

```bash
cargo build --release -p rofi-search
```

### Build the entire workspace

```bash
cargo build --release --workspace
```

Cargo places the resulting binaries in:

```text
target/release/
```

## Testing

Run formatting checks:

```bash
cargo fmt --all -- --check
```

Run Clippy:

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

Run all tests:

```bash
cargo test --workspace
```

These checks are also performed automatically by GitHub Actions.

## Releases

Tagged releases build static Linux x86_64 binaries for all implemented tools using the `x86_64-unknown-linux-musl` target.

To create a release, push a version tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions runs the workspace checks before building and publishing the binaries.

The resulting release contains:

- `rofi-search-linux-x86_64-musl`
- `brightness-control-linux-x86_64-musl`

## Roadmap

### Core Development

- [x] Initialize the repository.
- [x] Set up the Cargo Workspace.
- [x] Add automated formatting checks.
- [x] Add Clippy checks.
- [x] Add workspace tests.
- [ ] Establish consistent project conventions.
- [ ] Document configuration options for each utility.

### Rofi Search

- [x] Read the search engine from the Hyprland configuration.
- [x] Search the web using Rofi.
- [x] Open recognized website addresses directly.
- [x] Support URL encoding.
- [x] Add unit tests for URL detection.
- [x] Add tests for configuration parsing.
- [ ] Improve configuration parsing.
- [ ] Support additional search engines.
- [ ] Improve handling of invalid URLs and configuration files.

### Brightness Control

- [x] Implement brightness percentage input.
- [x] Read current and maximum brightness.
- [x] Gradually change brightness.
- [x] Handle invalid input and command errors.
- [ ] Improve test coverage.
- [ ] Add support for additional brightness backends.

### Additional Utilities

- [ ] Implement volume control.
- [ ] Identify useful Bash scripts that can be replaced with Rust.
- [ ] Add more Hyprland workflow utilities.

### Build and Distribution

- [x] Configure static Linux builds using musl.
- [x] Support x86_64 Linux releases.
- [x] Automate builds using GitHub Actions.
- [x] Publish binaries through GitHub Releases.
- [ ] Add AArch64 Linux builds.
- [ ] Provide an installation script.
- [ ] Add installation and upgrade instructions.

### Documentation and Community

- [ ] Write documentation for each utility.
- [ ] Add usage examples.
- [ ] Document dependencies and requirements.
- [ ] Add contribution guidelines.

## Design Philosophy

**Simplicity:** Each utility should solve a specific problem without unnecessary complexity.

**Reliability:** Applications should handle errors properly instead of silently failing.

**Maintainability:** Code should be structured in a way that makes future development straightforward.

**Independence:** Each utility should be usable without requiring the entire collection.

**Practicality:** Features should solve real desktop workflow problems rather than exist just for the sake of adding functionality.

## Contributing

Contributions, bug reports, and suggestions are welcome.

If you have an idea for a useful Linux desktop utility or want to improve an existing tool, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
