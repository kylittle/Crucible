pub mod img_loader;
pub mod obj_loader;

/// Checks the env variable ASSET_DIR to find where assets are stored. Otherwise searches
/// for 6 directories up for a folder called assets and the file itself.
fn build_asset_path(asset_filename: &str) -> Option<String> {
    let folder = std::env::var("ASSET_DIR");

    match folder {
        Ok(path) => {
            // Found a path append the filename
            return Some(path + asset_filename);
        }
        Err(_) => {
            // Found no env variable lets search a bit
            if std::fs::exists("assets/".to_owned() + asset_filename).is_ok() {
                return Some("assets/".to_owned() + asset_filename);
            }
            if std::fs::exists("../assets/".to_owned() + asset_filename).is_ok() {
                return Some("../assets/".to_owned() + asset_filename);
            }
            if std::fs::exists("../../assets/".to_owned() + asset_filename).is_ok() {
                return Some("../../assets/".to_owned() + asset_filename);
            }
            if std::fs::exists("../../../assets/".to_owned() + asset_filename).is_ok() {
                return Some("../../../assets/".to_owned() + asset_filename);
            }
            if std::fs::exists("../../../../assets/".to_owned() + asset_filename).is_ok() {
                return Some("../../../../assets/".to_owned() + asset_filename);
            }
            if std::fs::exists("../../../../../assets/".to_owned() + asset_filename).is_ok() {
                return Some("../../../../../assets/".to_owned() + asset_filename);
            }
            if std::fs::exists("../../../../../../assets/".to_owned() + asset_filename).is_ok() {
                return Some("../../../../../../assets/".to_owned() + asset_filename);
            }
        }
    }

    None
}
