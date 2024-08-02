use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;

#[derive(Parser)]
#[command(name = "Python Package Manager")]
#[command(version = "1.0")]
#[command(about = "A CLI tool to manage Python packages and save them in a requirements.json file", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install { name: Option<String>, version: Option<String>, requirements: Option<String> },
    Delete { name: String },
    Update { name: String, version: String },
    List,
}

#[derive(Serialize, Deserialize)]
struct PackageRegistry {
    packages: HashMap<String, String>,
}

impl PackageRegistry {
    fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }
}

fn main() {
    let args = Cli::parse();
    let mut packages = load_packages();

    match args.command {
        Commands::Install { name, version, requirements } => {
            if let Some(requirements) = requirements {
                install_from_requirements(&requirements, &mut packages);
            } else if let Some(name) = name {
                install_package(&name, version.as_deref(), &mut packages);
            } else {
                println!("You must specify a package name or a requirements file.");
            }
        }
        Commands::Delete { name } => {
            delete_package(&name, &mut packages);
        }
        Commands::Update { name, version } => {
            update_package(&name, &version, &mut packages);
        }
        Commands::List => {
            list_packages(&packages);
        }
    }

    save_packages(&packages);
}

fn load_packages() -> PackageRegistry {
    if let Ok(data) = fs::read_to_string("requirements.json") {
        serde_json::from_str(&data).unwrap_or_else(|_| PackageRegistry::new())
    } else {
        PackageRegistry::new()
    }
}

fn save_packages(packages: &PackageRegistry) {
    let data = serde_json::to_string_pretty(packages).expect("Failed to serialize packages");
    fs::write("requirements.json", data).expect("Failed to write requirements.json");
}

fn run_command(command: &str) -> bool {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command])
            .status()
            .expect("Failed to execute command")
            .success()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .status()
            .expect("Failed to execute command")
            .success()
    }
}

fn install_package(name: &str, version: Option<&str>, packages: &mut PackageRegistry) {
    let version = version.unwrap_or("latest");
    let install_command = if version == "latest" {
        format!("pip install {}", name)
    } else {
        format!("pip install {}=={}", name, version)
    };
    if run_command(&install_command) {
        packages.packages.insert(name.to_string(), version.to_string());
        println!("Package {} installed successfully", name);
    } else {
        println!("Failed to install package {}", name);
    }
}

fn delete_package(name: &str, packages: &mut PackageRegistry) {
    if run_command(&format!("pip uninstall -y {}", name)) {
        packages.packages.remove(name);
        println!("Package {} deleted successfully", name);
    } else {
        println!("Failed to delete package {}", name);
    }
}

fn update_package(name: &str, version: &str, packages: &mut PackageRegistry) {
    let update_command = format!("pip install {}=={}", name, version);
    if run_command(&update_command) {
        packages.packages.insert(name.to_string(), version.to_string());
        println!("Package {} updated successfully to version {}", name, version);
    } else {
        println!("Failed to update package {}", name);
    }
}

fn install_from_requirements(requirements: &str, packages: &mut PackageRegistry) {
    if let Ok(data) = fs::read_to_string(requirements) {
        if let Ok(registry) = serde_json::from_str::<PackageRegistry>(&data) {
            for (name, version) in registry.packages {
                install_package(&name, Some(&version), packages);
            }
        } else {
            println!("Failed to parse the requirements file.");
        }
    } else {
        println!("Failed to read the requirements file.");
    }
}

fn list_packages(packages: &PackageRegistry) {
    for (name, version) in &packages.packages {
        println!("{}: {}", name, version);
    }
}
