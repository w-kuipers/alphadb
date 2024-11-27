pub fn verify_version_number(version_number: &String) -> bool {
    let version_number = version_number.replace(".", "");
    version_number.parse::<i32>().expect("Could not convert version to integer");
    return true;
}

pub fn get_version_number_int(version_number: &String) -> u32 {
    let version_number = version_number.replace(".", "");
    version_number.parse::<u32>().expect("Could not convert version to integer")
}

/// Get the latest version in a version source
///
/// versions: Vector of versions from version source
pub fn get_latest_version(versions: &Vec<serde_json::Value>) -> String {
    let mut latest_version = String::from("0.0.0");
    for version in versions {
        let version = version["_id"].as_str().expect("No verssion number was specified");

        if get_version_number_int(&String::from(version)) > get_version_number_int(&latest_version) {
            latest_version = version.to_string();
        }
    }

    latest_version
}
