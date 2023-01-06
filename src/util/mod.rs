use std::path::PathBuf;

pub(crate) mod fakemap_hack;

/// Finds the path of the user's microcontroller data folder.
///
/// It should be at `%appdata%/Stormworks/data/microprocessors/`.
///
/// # Errors
///
/// Will return an [`Err`] if the path cannot be found.
pub fn find_microcontroller_folder() -> Result<PathBuf, &'static str> {
    if let Some(data_dir) = dirs::data_dir() {
        let mcs = data_dir.join(
            "Stormworks/data/microprocessors/".replace('/', &std::path::MAIN_SEPARATOR.to_string()),
        );
        if mcs.exists() {
            Ok(mcs)
        } else {
            Err("Could not find folder at %appdata%/Stormworks/data/microprocessors/, please specify full path to microprocessors folder.")
        }
    } else {
        Err("Could not find %appdata%, please specify full path to microprocessors folder.")
    }
}
