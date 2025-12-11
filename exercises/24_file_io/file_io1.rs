use std::fs;
use std::path::Path;

const TEST_FILE_NAME: &str = "SampleTextFile.txt";
fn main() {

    create_required_files();

    let read_str_result = fs::read_to_string(TEST_FILE_NAME);

    match read_str_result {
        Ok(contents) => {
            // TODO : What would be the expected text ?
            assert_eq!(, contents);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            assert!(false);
        }
    }
}


fn create_required_files(){
    let file_path = Path::new(TEST_FILE_NAME);

    if file_path.exists() == false {
        fs::write(&file_path, "This is the file content.").unwrap();
        println!("File created.");
    }

}