use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    loop {
        println!("==== File Manager ====");
        println!("1. List files in a directory");
        println!("2. Copy a file");
        println!("3. Move a file");
        println!("4. Delete a file");
        println!("5. Create a directory");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        match choice.trim() {
            "1" => list_files(),
            "2" => copy_file(),
            "3" => move_file(),
            "4" => delete_file(),
            "5" => create_directory(),
            "6" => break,
            _ => println!("Invalid choice. Try again."),
        }
    }
}

fn list_files() {
    let mut dir = String::new();
    println!("Enter directory path:");
    io::stdin().read_line(&mut dir).expect("Failed to read input");

    let dir = dir.trim();
    if Path::new(dir).is_dir() {
        println!("Files in '{}':", dir);
        for entry in WalkDir::new(dir).min_depth(1).max_depth(1) {
            match entry {
                Ok(e) => println!("{}", e.path().display()),
                Err(err) => eprintln!("Error reading entry: {}", err),
            }
        }
    } else {
        println!("Invalid directory path.");
    }
}

fn copy_file() {
    let mut src = String::new();
    let mut dest = String::new();

    println!("Enter source file path:");
    io::stdin().read_line(&mut src).expect("Failed to read input");
    println!("Enter destination file path:");
    io::stdin().read_line(&mut dest).expect("Failed to read input");

    let src = src.trim();
    let dest = dest.trim();

    match fs::copy(src, dest) {
        Ok(_) => println!("File copied successfully."),
        Err(err) => eprintln!("Error copying file: {}", err),
    }
}

fn move_file() {
    let mut src = String::new();
    let mut dest = String::new();

    println!("Enter source file path:");
    io::stdin().read_line(&mut src).expect("Failed to read input");
    println!("Enter destination file path:");
    io::stdin().read_line(&mut dest).expect("Failed to read input");

    let src = src.trim();
    let dest = dest.trim();

    match fs::rename(src, dest) {
        Ok(_) => println!("File moved successfully."),
        Err(err) => eprintln!("Error moving file: {}", err),
    }
}

fn delete_file() {
    let mut path = String::new();

    println!("Enter file path to delete:");
    io::stdin().read_line(&mut path).expect("Failed to read input");

    let path = path.trim();

    match fs::remove_file(path) {
        Ok(_) => println!("File deleted successfully."),
        Err(err) => eprintln!("Error deleting file: {}", err),
    }
}

fn create_directory() {
    let mut path = String::new();

    println!("Enter directory path to create:");
    io::stdin().read_line(&mut path).expect("Failed to read input");

    let path = path.trim();

    match fs::create_dir_all(path) {
        Ok(_) => println!("Directory created successfully."),
        Err(err) => eprintln!("Error creating directory: {}", err),
    }
}
