use std::fs;
use std::path::Path;

const TEST_FILE_NAME: &str = "SampleTextFile.txt";
fn main() {
    create_required_files();

    let read_str_result = fs::read_to_string(TEST_FILE_NAME);

    match read_str_result {
        Ok(contents) => {
            assert_eq!("This is the file content.", contents);
        }
        Err(err) => {
            eprintln!("File read error. {}", err);
        }
    }

    file_cleanup();
}

fn create_required_files() {
    let file_path = Path::new(TEST_FILE_NAME);

    if !file_path.exists() {
        let file_write_result = fs::write(file_path, "This is the file content.");
        if file_write_result.is_ok() {
            println!("Successfully wrote to file.");
        } else {
            panic!("Error writing to file.");
        }
    } else {
        println!("File already exist.");
    }
}

fn file_cleanup() {
    let file_path = Path::new(TEST_FILE_NAME);

    if file_path.exists() {
        let remove_status = fs::remove_file(file_path);
        if remove_status.is_ok() {
            println!("Successfully removed file.");
        } else {
            panic!("Error deleting file.");
        }
    } else {
        println!("No cleanup necassary since file not exist.");
    }
}
