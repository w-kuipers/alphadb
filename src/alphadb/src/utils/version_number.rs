pub fn verify_version_number(version_number: &String) -> bool {
    let version_number = version_number.replace(".", "");
    version_number
        .parse::<i32>()
        .expect("Could not convert version to integer");
    return true;
}

pub fn get_version_number_int(version_number: &String) -> u32 {
    let version_number = version_number.replace(".", "");
    version_number
        .parse::<u32>()
        .expect("Could not convert version to integer")
}
