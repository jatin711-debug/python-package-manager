use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::path::Path;

#[derive(Parser)]
#[command(name = "Python Package Manager")]
#[command(version = "1.0")]
#[command(about = "A CLI tool to manage Python packages and save them in a requirements.json file", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Install { packages: Vec<String> },
    Delete { name: String },
    Update { name: String, version: String },
    List,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PackageRegistry {
    pub packages: HashMap<String, String>,
}

impl PackageRegistry {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }
}

pub fn load_packages() -> PackageRegistry {
    load_packages_from_path(Path::new("requirements.json"))
}

pub fn load_packages_from_path(path: &Path) -> PackageRegistry {
    if let Ok(data) = fs::read_to_string(path) {
        serde_json::from_str(&data).unwrap_or_else(|_| PackageRegistry::new())
    } else {
        PackageRegistry::new()
    }
}

pub fn save_packages(packages: &PackageRegistry) {
    save_packages_to_path(packages, Path::new("requirements.json"))
}

pub fn save_packages_to_path(packages: &PackageRegistry, path: &Path) {
    let data = serde_json::to_string_pretty(packages).expect("Failed to serialize packages");
    fs::write(path, data).expect("Failed to write requirements.json");
}

#[cfg(test)]
pub fn run_command(_command: &str) -> bool {
    true // Mock implementation always returns success
}

#[cfg(not(test))]
pub fn run_command(command: &str) -> bool {
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

pub fn install_packages(package_names: &[String], packages: &mut PackageRegistry) {
    for name in package_names {
        let install_command = format!("pip install {}", name);
        if run_command(&install_command) {
            packages.packages.insert(name.clone(), "latest".to_string());
            println!("Package {} installed successfully", name);
        } else {
            println!("Failed to install package {}", name);
        }
    }
}

pub fn delete_package(name: &str, packages: &mut PackageRegistry) {
    if run_command(&format!("pip uninstall -y {}", name)) {
        packages.packages.remove(name);
        println!("Package {} deleted successfully", name);
    } else {
        println!("Failed to delete package {}", name);
    }
}

pub fn update_package(name: &str, version: &str, packages: &mut PackageRegistry) {
    let update_command = format!("pip install {}=={}", name, version);
    if run_command(&update_command) {
        packages.packages.insert(name.to_string(), version.to_string());
        println!("Package {} updated successfully to version {}", name, version);
    } else {
        println!("Failed to update package {}", name);
    }
}

pub fn install_from_requirements(requirements: &str, packages: &mut PackageRegistry) {
    if let Ok(data) = fs::read_to_string(requirements) {
        if let Ok(registry) = serde_json::from_str::<PackageRegistry>(&data) {
            for (name, version) in registry.packages {
                let install_command = format!("pip install {}=={}", name, version);
                if run_command(&install_command) {
                    packages.packages.insert(name.clone(), version.clone());
                    println!("Package {} installed successfully", name);
                } else {
                    println!("Failed to install package {}", name);
                }
            }
        } else {
            println!("Failed to parse the requirements file.");
        }
    } else {
        println!("Failed to read the requirements file.");
    }
}

pub fn list_packages(packages: &PackageRegistry) {
    for (name, version) in &packages.packages {
        println!("{}: {}", name, version);
    }
}
