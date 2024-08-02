use std::fs;
use tempfile::tempdir;
use python_package_manager::{PackageRegistry, load_packages_from_path, save_packages_to_path, install_package, delete_package, update_package, list_packages};

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
    let mut packages = PackageRegistry::new();
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
fn test_install_package() {
    let dir = tempdir().expect("Failed to create temp dir");
    let file_path: std::path::PathBuf = dir.path().join("requirements.json");
    fs::write(&file_path, "{}").expect("Failed to write to temp file");
    let mut packages = PackageRegistry::new();
    install_package("pandas", Some("latest"), &mut packages);
    save_packages_to_path(&packages, &file_path);
    let updated_packages = load_packages_from_path(&file_path);
    assert_eq!(updated_packages.packages.get("pandas").unwrap(), "latest");
}

#[test]
fn test_delete_package() {
    let dir = tempdir().expect("Failed to create temp dir");
    let file_path = dir.path().join("requirements.json");
    let mut packages = PackageRegistry::new();
    packages.packages.insert("pandas".to_string(), "1.0.0".to_string());
    save_packages_to_path(&packages, &file_path);
    delete_package("pandas", &mut packages);
    save_packages_to_path(&packages, &file_path);
    let updated_packages = load_packages_from_path(&file_path);
    assert!(updated_packages.packages.get("pandas").is_none());
}

#[test]
fn test_update_package() {
    let dir = tempdir().expect("Failed to create temp dir");
    let file_path = dir.path().join("requirements.json");
    let mut packages = PackageRegistry::new();
    packages.packages.insert("pandas".to_string(), "1.0.0".to_string());
    save_packages_to_path(&packages, &file_path);
    update_package("pandas", "2.0.0", &mut packages);
    save_packages_to_path(&packages, &file_path);
    let updated_packages = load_packages_from_path(&file_path);
    assert_eq!(updated_packages.packages.get("pandas").unwrap(), "2.0.0");
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
