use std::path::PathBuf;
use walkdir::WalkDir;

pub fn find_files_with_extensions(dir: &PathBuf, extensions: &[String]) -> Vec<std::path::PathBuf> {
    let mut matching_files = Vec::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if extensions.iter().any(|e| e == ext) {
                matching_files.push(path.to_path_buf());
            }
        }
    }

    matching_files
}
