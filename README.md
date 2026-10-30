# HyprTools

**A collection of lightweight Linux desktop utilities built with Rust, primarily designed for Hyprland.**

HyprTools is an open-source project that aims to replace existing shell scripts with reliable, efficient, and maintainable Rust applications.

The main goal is to build a collection of standalone utilities that simplify everyday desktop tasks, improve workflow automation, and integrate naturally with the Hyprland ecosystem.

Rather than maintaining a large collection of independent shell scripts, HyprTools brings these utilities together in a single Rust workspace.

## Goals

- Replace existing Bash scripts with Rust applications where it makes sense.
- Improve reliability, error handling, and maintainability.
- Minimize unnecessary runtime dependencies.
- Provide lightweight tools that integrate with existing Linux utilities.
- Support multiple Linux architectures through precompiled releases.
- Automate building and releasing binaries using GitHub Actions.
- Keep each utility independent and easy to install.

## Project Structure

HyprTools uses a Cargo Workspace to organize multiple independent applications.

```text
hyprtools/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── crates/
│   └── rofi-search/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
├── scripts/
└── .github/
    └── workflows/
```

Each utility lives in its own crate and can be built independently.

## Tools

| Tool | Description | Status |
|---|---|---|
| `rofi-search` | Search the web or open websites directly through Rofi. | In development |
| `volume-control` | Audio volume management utility. | Planned |
| `brightness-control` | Display brightness management utility. | Planned |

More utilities will be added as development progresses.

## Building

### Requirements

- Rust toolchain
- Cargo
- Linux
- Required system utilities for each application

### Build an individual tool

```bash
cargo build --release -p rofi-search
```

### Build the entire workspace

```bash
cargo build --release --workspace
```

The compiled binaries will be available in:

```text
target/release/
```

## Roadmap

The following tasks represent the planned development direction of HyprTools.

### Core Development

- [x] Initialize the repository.
- [x] Set up the Cargo Workspace.
- [x] Create the initial `rofi-search` crate.
- [ ] Improve error handling across all utilities.
- [ ] Add unit and integration tests.
- [ ] Establish consistent project conventions.
- [ ] Document configuration options for each utility.

### Rofi Search

- [x] Read the search engine from the Hyprland configuration.
- [x] Search the web using Rofi.
- [x] Open recognized website addresses directly.
- [x] Support URL encoding.
- [x] Launch URLs without blocking the main application.
- [ ] Add unit tests for URL detection.
- [ ] Improve configuration parsing.
- [ ] Support additional search engines.
- [ ] Add better handling for invalid URLs and configuration files.

### Additional Utilities

- [ ] Implement volume control.
- [ ] Implement brightness control.
- [ ] Identify useful Bash scripts that can be replaced with Rust.
- [ ] Add more Hyprland workflow utilities.

### Build and Distribution

- [ ] Configure static Linux builds using musl.
- [ ] Support x86_64 Linux releases.
- [ ] Add AArch64 Linux builds.
- [ ] Automate builds using GitHub Actions.
- [ ] Publish binaries through GitHub Releases.
- [ ] Provide an installation script.
- [ ] Add installation and upgrade instructions.

### Documentation and Community

- [ ] Write documentation for each utility.
- [ ] Add usage examples.
- [ ] Document dependencies and requirements.
- [ ] Add contribution guidelines.

## Design Philosophy

HyprTools follows a few simple principles:

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
