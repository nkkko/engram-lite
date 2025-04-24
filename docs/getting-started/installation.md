# Installation Guide

This guide will help you install EngramAI Lite on your system.

## Prerequisites

Before installing EngramAI Lite, ensure you have the following requirements:

- **Rust**: Version 1.70.0 or higher (installation via [rustup](https://rustup.rs/) recommended)
- **Build Tools**: C/C++ compiler and development tools
  - Linux: `build-essential`, `clang`, `libclang-dev`
  - macOS: Xcode Command Line Tools
  - Windows: Visual Studio Build Tools with C++ development workload
- **Git**: For cloning the repository

## Installing from Source

### Step 1: Clone the Repository

```bash
git clone https://github.com/nkkko/engram-lite.git
cd engram-lite
```

### Step 2: Build the Project

```bash
# Build in release mode
cargo build --release
```

This will create the binary in `./target/release/engramlt`.

### Step 3: Add to PATH (Optional)

For easier access, you can add the binary to your system PATH:

=== "Linux/macOS"

    ```bash
    # Create a symlink to your binary in a directory that's already in your PATH
    sudo ln -s $(pwd)/target/release/engramlt /usr/local/bin/engramlt
    ```

=== "Windows"

    ```powershell
    # Add the binary location to your PATH environment variable
    $binPath = Join-Path (Get-Location) "target\release"
    [Environment]::SetEnvironmentVariable("PATH", $env:PATH + ";" + $binPath, [EnvironmentVariableTarget]::User)
    ```

## Configuration

EngramAI Lite stores data in a RocksDB database by default in the `./engram_db` directory relative to where the command is run. You can specify a different location:

```bash
engramlt --db-path /path/to/custom/database
```

## Verifying Installation

To verify that EngramAI Lite is installed correctly:

```bash
engramlt --help
```

You should see the help menu displaying available commands and options.

## Troubleshooting

### Compilation Errors

If you encounter errors during compilation related to RocksDB:

- Make sure you have the necessary C/C++ development tools installed
- On Linux, you might need additional libraries: `sudo apt install libsnappy-dev librocksdb-dev`

### Database Access Issues

If you experience permission errors when accessing the database:

- Ensure the directory exists and has appropriate permissions
- Try running with a custom path in a location where you have write access:
  ```bash
  engramlt --db-path ~/engram_data
  ```

## Next Steps

Once EngramAI Lite is installed, proceed to the [Quickstart Guide](quickstart.md) to learn how to use the system.