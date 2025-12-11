use std::fs;

fn main() {

    let read_str_result = fs::read_to_string("solutions/24_file_io/SampleFilesFolder/SampleTextFile.txt");

    match read_str_result {
        Ok(contents) => {
            assert_eq!("This is the file content.", contents);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            assert!(false);
        }
    }
}
