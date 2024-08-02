use clap::Parser;
use python_package_manager::{Cli, Commands, load_packages, save_packages, install_package, delete_package, update_package, list_packages, install_from_requirements};

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
