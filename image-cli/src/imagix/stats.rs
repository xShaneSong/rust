use super::error::ImagixError;
use super::resize::get_image_files;
use std::path::PathBuf;

pub fn get_stats(src_folder: PathBuf) -> Result<(usize, f64), ImagixError> {
    let image_files = get_image_files(src_folder.to_path_buf())?;
    let size = image_files
        .iter()
        .map(move |f| match std::fs::metadata(f) {
            Ok(metadata) => metadata.len(),
            Err(e) => {
                eprintln!("Failed to get metadata for file {}: {}", f, e);
                0
            }
        })
        .sum::<u64>();
    Ok((image_files.len(), size / 1000000 as f64))
}
