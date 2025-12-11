use std::path::{Path, PathBuf};

fn main() {

    let mut path_buffer = PathBuf::new();

    path_buffer.push("solutions");
    path_buffer.push("24_file_io");
    path_buffer.push("SampleFilesFolder");
    path_buffer.push("MultiLineTextFile.txt");

    let meta_data_result = path_buffer.metadata();

    if let Ok(meta_data) = meta_data_result {
        println!("Metadata about the file : {:?}", path_buffer);
        println!("File creation time {:?}", meta_data.created().unwrap());
        println!("File size {}", meta_data.len());
        assert_eq!(meta_data.len(), 101);
        println!("File permissions {:?}", meta_data.permissions());
        assert_eq!(meta_data.permissions().readonly(), false);
    }else {
        println!("Could not get metadata");
        assert!(false);
    }




}