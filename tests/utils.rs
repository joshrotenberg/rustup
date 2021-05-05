use rustup::utils::utils::{ensure_file_removed, remove_file};
use rustup::RustupError;
use std::fs::File;

#[test]
fn utils_remove_file() {
    let tempdir = tempfile::Builder::new().prefix("rustup").tempdir().unwrap();
    let f_path = tempdir.path().join("f");
    File::create(&f_path).unwrap();

    assert!(f_path.exists());
    assert!(remove_file("f", &f_path).is_ok());
    assert!(!f_path.exists());

    let result = remove_file("f", &f_path);
    // assert!(result.is_err());
    let err = result.unwrap_err();

    match err.downcast_ref::<RustupError>() {
        Some(RustupError::RemovingFile { name, path }) => {
            assert_eq!(*name, "f");
            assert_eq!(path.clone(), f_path);
        }
        _ => panic!(),
    }
}
