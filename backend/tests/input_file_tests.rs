use defispring::api::processor::retrieve_valid_files;
use std::fs;

/// Tests that only valid files are utilized
#[test]
fn file_names() {
    let path = "./tests/test_empty_input_files".to_string();
    let files = retrieve_valid_files(path.clone());
    let paths: Vec<String> = files.iter().map(|f| f.full_path.clone()).collect();
    let dir_entries: Vec<String> = fs::read_dir(&path)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    let has_raw_upper = dir_entries.iter().any(|name| name == "raw_1.ZIP");
    let has_raw_upper_all = dir_entries.iter().any(|name| name == "RAW_1.ZIP");

    // Valid files
    assert!(paths.contains(&(path.clone() + &"/raw_1.zip")));
    if has_raw_upper {
        assert!(paths.contains(&(path.clone() + &"/raw_1.ZIP")));
    }
    if has_raw_upper_all {
        assert!(paths.contains(&(path.clone() + &"/RAW_1.ZIP")));
    }
    assert!(paths.contains(&(path.clone() + &"/raw_100.zip")));

    // Invalid files
    assert!(!paths.contains(&(path.clone() + &"/raw_0.zip")));
    assert!(!paths.contains(&(path.clone() + &"/raw_1.json")));
    assert!(!paths.contains(&(path.clone() + &"/raw_1.JSON")));
    assert!(!paths.contains(&(path.clone() + &"/raw_1.json.zip")));
    assert!(!paths.contains(&(path.clone() + &"/raw-1.zip")));
    assert!(!paths.contains(&(path.clone() + &"/xraw_1.zip")));
}
