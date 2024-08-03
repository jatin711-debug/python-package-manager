use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use tempfile::tempdir;
use python_package_manager::{PackageRegistry, load_packages_from_path, save_packages_to_path, install_packages, delete_package, update_package, list_packages, install_from_requirements};

fn get_python_interpreter() -> String {
    let version_output: Output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "where python"])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("which python3")
            .output()
            .expect("Failed to execute command")
    };

    if !version_output.status.success() {
        panic!("Failed to find Python interpreter: {}", String::from_utf8_lossy(&version_output.stderr));
    }

    let interpreter_path = String::from_utf8_lossy(&version_output.stdout)
        .lines()
        .next()
        .expect("No Python interpreter found")
        .to_string();

    // Verify the Python version is 3.10 or above
    let version_check = Command::new(&interpreter_path)
        .arg("--version")
        .output()
        .expect("Failed to check Python version");

    if !version_check.status.success() {
        panic!("Failed to check Python version: {}", String::from_utf8_lossy(&version_check.stderr));
    }

    let version_str = String::from_utf8_lossy(&version_check.stdout);
    if version_str.contains("Python 3.10") || version_str.contains("Python 3.11") {
        interpreter_path
    } else {
        panic!("Python version 3.10 or above is required. Found: {}", version_str);
    }
}

fn create_virtualenv(env_path: &Path) {
    let python_interpreter = get_python_interpreter();

    let output = Command::new(&python_interpreter)
        .args(&["-m", "venv", env_path.to_str().unwrap()])
        .output()
        .expect("Failed to create virtual environment");

    if !output.status.success() {
        panic!(
            "Failed to create virtual environment: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn run_command_in_virtualenv(env_path: &Path, args: &[&str]) -> bool {
    let bin_path = if cfg!(target_os = "windows") {
        env_path.join("Scripts").join("pip")
    } else {
        env_path.join("bin").join("pip")
    };

    let output = Command::new(bin_path)
        .args(args)
        .output()
        .expect("Failed to run command in virtual environment");

    if !output.status.success() {
        println!("Command failed: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("Command succeeded: {}", String::from_utf8_lossy(&output.stdout));
    }

    output.status.success()
}

#[test]
fn test_load_empty_packages() {
    let dir = tempdir().expect("Failed to create temp dir");
    let file_path = dir.path().join("requirements.json");
    fs::write(&file_path, "{}").expect("Failed to write to temp file");
    let packages = load_packages_from_path(&file_path);
    assert_eq!(packages, PackageRegistry::new());
}

#[test]
fn test_load_packages() {
    let dir = tempdir().expect("Failed to create temp dir");
    let file_path = dir.path().join("requirements.json");
    let mut expected_packages = PackageRegistry::new();
    expected_packages.packages.insert("pandas".to_string(), "1.0.0".to_string());
    fs::write(&file_path, r#"{"packages":{"pandas":"1.0.0"}}"#).expect("Failed to write to temp file");
    let packages = load_packages_from_path(&file_path);
    assert_eq!(packages, expected_packages);
}

#[test]
fn test_save_packages() {
    let dir = tempdir().expect("Failed to create temp dir");
    let file_path = dir.path().join("requirements.json");
    let mut packages: PackageRegistry = PackageRegistry::new();
    packages.packages.insert("pandas".to_string(), "1.0.0".to_string());
    save_packages_to_path(&packages, &file_path);
    let data = fs::read_to_string(&file_path).expect("Failed to read from temp file");
    assert_eq!(data, r#"{
  "packages": {
    "pandas": "1.0.0"
  }
}"#);
}

#[test]
fn test_install_packages() {
    let dir = tempdir().expect("Failed to create temp dir");
    let env_path = dir.path().join("env");
    create_virtualenv(&env_path);

    let file_path = dir.path().join("requirements.json");
    fs::write(&file_path, "{}").expect("Failed to write to temp file");

    let mut packages = PackageRegistry::new();
    let package_list = vec!["pandas".to_string(), "numpy".to_string(), "scipy".to_string()];
    for package in &package_list {
        if !run_command_in_virtualenv(&env_path, &["install", package]) {
            panic!("Failed to install {}", package);
        }
    }

    install_packages(&package_list, &mut packages);
    save_packages_to_path(&packages, &file_path);
    let updated_packages = load_packages_from_path(&file_path);

    for package in &package_list {
        assert_eq!(updated_packages.packages.get(package).unwrap(), "latest");
    }
}

#[test]
fn test_delete_package() {
    let dir = tempdir().expect("Failed to create temp dir");
    let env_path = dir.path().join("env");
    create_virtualenv(&env_path);

    let file_path = dir.path().join("requirements.json");
    let mut packages = PackageRegistry::new();
    packages.packages.insert("pandas".to_string(), "1.0.0".to_string());
    save_packages_to_path(&packages, &file_path);

    if run_command_in_virtualenv(&env_path, &["uninstall", "-y", "pandas"]) {
        delete_package("pandas", &mut packages);
        save_packages_to_path(&packages, &file_path);
        let updated_packages = load_packages_from_path(&file_path);
        assert!(updated_packages.packages.get("pandas").is_none());
    } else {
        panic!("Failed to uninstall pandas");
    }
}

#[test]
fn test_update_package() {
    let dir = tempdir().expect("Failed to create temp dir");
    let env_path = dir.path().join("env");
    create_virtualenv(&env_path);

    let file_path = dir.path().join("requirements.json");
    let mut packages = PackageRegistry::new();
    packages.packages.insert("pandas".to_string(), "1.0.0".to_string());
    save_packages_to_path(&packages, &file_path);

    if run_command_in_virtualenv(&env_path, &["install", "pandas==2.0.0"]) {
        update_package("pandas", "2.0.0", &mut packages);
        save_packages_to_path(&packages, &file_path);
        let updated_packages = load_packages_from_path(&file_path);
        assert_eq!(updated_packages.packages.get("pandas").unwrap(), "2.0.0");
    } else {
        panic!("Failed to update pandas");
    }
}

#[test]
fn test_list_packages() {
    let dir = tempdir().expect("Failed to create temp dir");
    let file_path = dir.path().join("requirements.json");
    let mut packages = PackageRegistry::new();
    packages.packages.insert("pandas".to_string(), "1.0.0".to_string());
    save_packages_to_path(&packages, &file_path);
    list_packages(&packages); // This just prints to stdout, so we're testing for no panic
}

#[test]
fn test_install_from_requirements() {
    let dir = tempdir().expect("Failed to create temp dir");
    let env_path = dir.path().join("env");
    create_virtualenv(&env_path);

    let file_path = dir.path().join("requirements.json");
    let mut initial_packages = PackageRegistry::new();
    initial_packages.packages.insert("pandas".to_string(), "1.0.0".to_string());
    initial_packages.packages.insert("numpy".to_string(), "1.19.5".to_string());
    save_packages_to_path(&initial_packages, &file_path);

    println!("Content of requirements.json: {}", fs::read_to_string(&file_path).unwrap());

    let mut packages = PackageRegistry::new();
    install_from_requirements(file_path.to_str().unwrap(), &mut packages);

    save_packages_to_path(&packages, &file_path);
    let updated_packages = load_packages_from_path(&file_path);

    assert_eq!(updated_packages.packages.get("pandas").unwrap(), "1.0.0");
    assert_eq!(updated_packages.packages.get("numpy").unwrap(), "1.19.5");
}
