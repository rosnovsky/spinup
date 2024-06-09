# CLI for setting up a new Linux computer

This is a command line tool to set up a new Linux computer. It is designed to check existing installed applications and configurations, to install any missing dependencies, and to configure the system to be ready for use.

> **Note:** This tool is still in development and is not yet ready for use. Right now, it only supports Fedora Linux and macOS (somewhat).

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- Nightly toolchain (https://rust-lang.github.io/rustup/concepts/toolchains.html#toolchain-specification)
- A `config.json` "secret" gist in your GitHub account following the [schema](./src/schema.json).

## Usage

To use the CLI, run the following command to build and run the project:

```bash
cargo run
```

or run the following command to use a pre-built binary:

```bash
./spinup
```
