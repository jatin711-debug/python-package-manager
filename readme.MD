# Python Package Manager

A CLI tool to manage Python packages and save them in a `requirements.json` file. This tool supports installing, deleting, updating, and listing Python packages. Additionally, it can install multiple packages from a given `requirements.json` file.

## Features

- Install a Python package and save its name and version in `requirements.json`
- Delete a Python package and remove its entry from `requirements.json`
- Update a Python package to a specified version and update `requirements.json`
- List all installed packages from `requirements.json`
- Install all packages listed in a `requirements.json` file

## Installation

To use this tool, you need to have Rust and Cargo installed on your system. If you don't have them installed, you can follow the instructions [here](https://www.rust-lang.org/tools/install).

Clone the repository and build the project:

```sh
git clone <repository-url>
cd python_package_manager
cargo build --release
