use clap::{Arg, Command};
use reqwest::blocking::multipart;
use reqwest::Client;
use std::fs;
use std::process::Command as ProcessCommand;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("orbit-cli")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("CLI for uploading WASM files to a serverless application")
        .arg(
            Arg::new("fn_name")
                // .about("The name of the WASM file to upload")
                .required(true)
                .index(1),
        )
        .get_matches();

    // let fn_name = matches.value_source("fn_name").unwrap();

    let current_dir = std::env::current_dir()?;
    let fn_name = current_dir.file_name().unwrap().to_str().unwrap();

    // Check if the current directory is a Rust project directory
    if !fs::metadata("Cargo.toml").is_ok() {
        eprintln!("Error: This command must be run in a Rust project directory.");
        std::process::exit(1);
    }

    // Execute "rustup target add wasm32-wasip1"
    let status = ProcessCommand::new("rustup")
        .args(&["target", "add", "wasm32-wasip1"])
        .status()
        .expect("Failed to execute rustup command");
    if !status.success() {
        eprintln!("Error: Failed to add wasm32-wasip1 target.");
        std::process::exit(1);
    }

    // Execute "cargo build --release --target wasm32-wasip1"
    let status = ProcessCommand::new("cargo")
        .args(&["build", "--release", "--target", "wasm32-wasip1"])
        .status()
        .expect("Failed to execute cargo build command");
    if !status.success() {
        eprintln!("Error: Failed to build the project.");
        std::process::exit(1);
    }

    // Prepare the file path
    dbg!(&fn_name);
    let wasm_file_path = format!("./target/wasm32-wasip1/release/{}.wasm", fn_name);

    // Check if the file exists
    if !fs::metadata(&wasm_file_path).is_ok() {
        dbg!(&wasm_file_path);
        eprintln!("Error: The specified WASM file does not exist.");
        std::process::exit(1);
    }

    // Create a multipart form
    let file_part = multipart::Part::file(&wasm_file_path)?;
    let form = multipart::Form::new()
        .text("fn_name", format!("{:?}", fn_name))
        .text("wasm_file", format!("@{:?}", file_part));

    // Send the POST request
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://localhost:8080/upload")
        .multipart(form)
        .send()?;

    if res.status().is_success() {
        println!("File uploaded successfully!");
    } else {
        eprintln!("Error: Failed to upload the file. Status: {}", res.status());
    }

    Ok(())
}
