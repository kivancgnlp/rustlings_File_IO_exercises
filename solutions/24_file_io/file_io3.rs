use std::fs;
use std::path::PathBuf;

fn main() {
    create_required_files();
    let mut path_buffer = PathBuf::new();

    path_buffer.push("SampleFilesFolder");
    path_buffer.push("MultiLineTextFile.txt");

    let meta_data_result = path_buffer.metadata();

    if let Ok(meta_data) = meta_data_result {
        println!("Metadata about the file : {:?}", path_buffer);
        println!("File creation time {:?}", meta_data.created().unwrap());
        println!("File size {}", meta_data.len());
        assert_eq!(meta_data.len(), 117);
        println!("File permissions {:?}", meta_data.permissions());
        assert!(!meta_data.permissions().readonly());
    } else {
        panic!("Could not get metadata");
    }
}

fn create_required_files() {
    let file_path = PathBuf::from("SampleFilesFolder/MultiLineTextFile.txt");

    let dir_path = file_path.parent().unwrap();

    if !dir_path.exists() {
        fs::create_dir(dir_path).unwrap();
        println!("Created directory {:?}", dir_path);
    }

    if !file_path.exists() {
        let text = "This is the first line of the text.
        This is the second line.
        And this is the third and the last line.";
        fs::write(file_path, text).unwrap();
        println!("File created.");
    }
}
