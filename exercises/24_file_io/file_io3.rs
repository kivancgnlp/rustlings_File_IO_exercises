use std::fs;
use std::path::PathBuf;

fn main() {
    create_required_files();
    let mut path_buffer = PathBuf::new();

    path_buffer.push("SampleFilesFolder");
    path_buffer.push("MultiLineTextFile.txt");

    // TODO : How to get metadata using path_buffer ?
    let meta_data_result = path_buffer.

    match meta_data_result {
        Ok(meta_data) => {
            println!("Metadata about the file : {:?}", path_buffer);
            println!("File creation time {:?}", meta_data.created());
            println!("File size {}", meta_data.len());
            assert_eq!(meta_data.len(), 117);
            println!("File permissions {:?}", meta_data.permissions());
            assert!(!meta_data.permissions().readonly());
        }
        Err(error) => {
            eprintln!("Could not get metadata. Error: {:?}", error);
        }
    }


    file_cleanup();
}

fn create_required_files() {
    let file_path = PathBuf::from("SampleFilesFolder/MultiLineTextFile.txt");

    let dir_path = file_path.parent().unwrap();

    if !dir_path.exists() {
        let dir_create_result = fs::create_dir(dir_path);
        if dir_create_result.is_ok() {
            println!("{:?} created", dir_path);
        }
    }

    if !file_path.exists() {
        let text = "This is the first line of the text.
        This is the second line.
        And this is the third and the last line.";
        let file_write_result = fs::write(&file_path, text);

        if file_write_result.is_ok() {
            println!("Multi line file created successfully!");
        }else {
            eprintln!("Error creating file : {} , error : {:?}", file_path.display(), file_write_result.err());
        }
    }
}

fn file_cleanup() {
    let mut path_buffer = PathBuf::new();

    path_buffer.push("SampleFilesFolder");
    path_buffer.push("MultiLineTextFile.txt");

    if path_buffer.exists() {
        let remove_status = fs::remove_file(&path_buffer);
        if remove_status.is_ok() {
            println!("Test file deleted.");
        }else {
            panic!("Error deleting file.");
        }
    }

    path_buffer.pop();

    if path_buffer.exists() {
        let remove_status = fs::remove_dir(&path_buffer);
        if remove_status.is_ok() {
            println!("Test directory deleted.");
        }else {
            panic!("Error deleting directory.");
        }
    }
}
