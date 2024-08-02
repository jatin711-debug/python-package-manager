use std::fs;
use python_package_manager::{PackageRegistry, load_packages, save_packages, install_package, delete_package, update_package, list_packages};

#[test]
fn test_load_empty_packages() {
    fs::write("requirements.json", "").unwrap();
    let packages = load_packages();
    assert_eq!(packages, PackageRegistry::new());
}

#[test]
fn test_load_packages() {
    let mut expected_packages = PackageRegistry::new();
    expected_packages.packages.insert("test_package".to_string(), "1.0.0".to_string());
    fs::write("requirements.json", r#"{"packages":{"test_package":"1.0.0"}}"#).unwrap();
    let packages = load_packages();
    assert_eq!(packages, expected_packages);
}

#[test]
fn test_save_packages() {
    let mut packages = PackageRegistry::new();
    packages.packages.insert("test_package".to_string(), "1.0.0".to_string());
    save_packages(&packages);
    let data = fs::read_to_string("requirements.json").unwrap();
    assert_eq!(data, r#"{
  "packages": {
    "test_package": "1.0.0"
  }
}"#);
}

#[test]
fn test_install_package() {
    let mut packages = PackageRegistry::new();
    install_package("test_package", Some("1.0.0"), &mut packages);
    assert_eq!(packages.packages.get("test_package").unwrap(), "1.0.0");
}

#[test]
fn test_delete_package() {
    let mut packages = PackageRegistry::new();
    packages.packages.insert("test_package".to_string(), "1.0.0".to_string());
    delete_package("test_package", &mut packages);
    assert!(packages.packages.get("test_package").is_none());
}

#[test]
fn test_update_package() {
    let mut packages = PackageRegistry::new();
    packages.packages.insert("test_package".to_string(), "1.0.0".to_string());
    update_package("test_package", "2.0.0", &mut packages);
    assert_eq!(packages.packages.get("test_package").unwrap(), "2.0.0");
}

#[test]
fn test_list_packages() {
    let mut packages = PackageRegistry::new();
    packages.packages.insert("test_package".to_string(), "1.0.0".to_string());
    list_packages(&packages); // This just prints to stdout, so we're testing for no panic
}
