mod config;
mod scanner;
mod worker;

use csv::{Reader, Writer};
use scanner::find_files_with_extensions;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "scrubbrush", about = "Video file batch conversion tool")]
struct Opt {
    #[structopt(parse(from_os_str))]
    config_path: Option<PathBuf>,
}

fn save_remaining_files(csv_file: &PathBuf, files: &[PathBuf]) -> Result<(), csv::Error> {
    let mut writer = Writer::from_path(csv_file)?;
    for file in files {
        writer.write_record([file.to_str().unwrap()])?;
    }
    writer.flush()?;
    Ok(())
}

fn load_remaining_files(csv_file: &PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut reader = Reader::from_path(csv_file).expect("Failed to open CSV file");
    for result in reader.records().flatten() {
        if let Some(path) = result.get(0) {
            files.push(PathBuf::from(path));
        }
    }
    files
}

fn main() {
    let args = Opt::from_args();

    let config_path = match args.config_path {
        Some(conf_path) => conf_path,
        None => PathBuf::from("./sbconfig.toml"),
    };

    let config = config::Config::parse_config(config_path).unwrap_or_else(|| {
        eprintln!("Failed to parse config");
        exit(1);
    });

    let csv_file = match config.csv_file {
        Some(f) => f,
        None => PathBuf::from("./sbstatus.csv"),
    };

    let mut unprocessed_files = if Path::new(&csv_file).exists() {
        println!("Loading from CSV");
        load_remaining_files(&csv_file)
    } else {
        println!("Scanning new files");
        find_files_with_extensions(&config.source_dir, &config.formats)
    };

    println!("Found {} files to process", unprocessed_files.len());

    let preset = config.preset.clone().unwrap_or("Fast 1080p30".to_string());
    let replace = config.overwrite;
    let hb_path = config.hb_path.clone().unwrap_or("HandBrakeCLI".to_string());
    let output_dir = config.output_dir;
    let encode_dir = config.encode_dir;
    let conv_to = config.conv_to;

    while let Some(file) = unprocessed_files.pop() {
        println!("Processing file: {:?}", file.file_name().unwrap());

        worker::worker_conv(
            file,
            preset.clone(),
            hb_path.clone(),
            output_dir.clone(),
            encode_dir.clone(),
            conv_to.clone(),
            replace,
        );

        // Save the remaining files to CSV
        if let Err(e) = save_remaining_files(&csv_file, &unprocessed_files) {
            eprintln!("Failed to save remaining files: {:?}", e);
        }
    }

    if unprocessed_files.is_empty() && Path::new(&csv_file).exists() {
        match fs::remove_file(csv_file) {
            Ok(_) => println!("No remaining files. CSV file removed."),
            Err(e) => eprintln!("Failed to remove CSV file: {:?}", e),
        }
    }
}
