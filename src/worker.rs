use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command};
use std::fs;

fn replace_extension(file_path: &str, new_extension: &str) -> String {
    let path = Path::new(file_path);

    if let Some(stem) = path.file_stem() {
        if let Some(parent) = path.parent() {
            // Reconstruct the path with the new extension
            return parent
                .join(format!("{}.{}", stem.to_string_lossy(), new_extension))
                .to_string_lossy()
                .to_string();
        } else {
            // If there's no parent, just replace the extension
            return format!("{}.{}", stem.to_string_lossy(), new_extension);
        }
    }

    // If the file_path is invalid, return it unchanged
    file_path.to_string()
}

pub fn worker_conv(
    video_input: PathBuf,
    preset: String,
    hb_path: String,
    output_dir: PathBuf,
    encode_dir: Option<PathBuf>,
    conv_to: String,
    replace: bool,
) -> bool {
    // path parsing
    let file_name = video_input
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let conv_file_name = replace_extension(&file_name, &conv_to);

    // copy input to encoding dir
    let input_path = match encode_dir {
        Some(edir) => {
            println!("copy {} to {:?}", &file_name, &edir);
            if !Path::new(&edir).exists() {
                fs::create_dir(&edir).unwrap();
            }
            let target = edir.join(file_name.clone());
            match fs::copy(&video_input, &target) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("failed to copy {} to {:?}: {}", &file_name, &edir, e);
                    return false;
                }
            }
            target
        }
        None => video_input.clone(),
    };

    let input_path = match input_path.to_str() {
        Some(s) => s,
        None => {
            eprintln!("Failed get video path: {:?}", file_name);
            return false;
        }
    };

    let output_path = match output_dir.to_str() {
        Some(s) => format!("{}/{}", s, conv_file_name),
        None => {
            eprintln!("Failed get output path: {:?}", file_name);
            return false;
        }
    };

    println!("encoding {} -> {}", &input_path, &output_path);
    let output = Command::new(hb_path)
        .args(["-i", input_path, "-Z", &preset, "-o", &output_path])
        .output()
        .unwrap_or_else(|_| panic!("Failed to convert {}", input_path));

    if !output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        return false;
    }

    if replace {
        let copy_to =
            PathBuf::from(PathBuf::from(&video_input).parent().unwrap()).join(conv_file_name);

        println!("overwriting {:?}", copy_to);
        match fs::copy(&output_path, &copy_to) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("failed to copy {} to {:?}: {}", &output_path, &copy_to, e);
                return false;
            }
        }
        match fs::remove_file(&video_input) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to remove {:?}: {}", &video_input, e);
                return false;
            }
        }
        match fs::remove_file(&output_path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to remove {}: {}", output_path, e);
                return false;
            }
        }
    }

    true
}
