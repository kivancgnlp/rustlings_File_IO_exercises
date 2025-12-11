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
        Err(_) => {
            panic!("Error reading file.");

        }
    }
}


fn create_required_files(){
    let file_path = Path::new(TEST_FILE_NAME);

    if !file_path.exists() {
        fs::write(file_path, "This is the file content.").unwrap();
        println!("File created.");
    }

}