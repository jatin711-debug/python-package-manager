use clap::Parser;
use python_package_manager::{Cli, Commands, load_packages, save_packages, install_packages, delete_package, update_package, list_packages, install_from_requirements};

fn main() {
    let args = Cli::parse();
    let mut package_registry = load_packages();

    match args.command {
        Commands::Install { packages } => {
            if packages.len() == 1 && packages[0].starts_with("-r=") {
                let requirements_path = &packages[0][3..];
                install_from_requirements(requirements_path, &mut package_registry);
            } else {
                install_packages(&packages, &mut package_registry);
            }
        }
        Commands::Delete { name } => {
            delete_package(&name, &mut package_registry);
        }
        Commands::Update { name, version } => {
            update_package(&name, &version, &mut package_registry);
        }
        Commands::List => {
            list_packages(&package_registry);
        }
    }

    save_packages(&package_registry);
}
