use std::fs;

fn main() {

    let read_str_result = fs::read_to_string("exercises/24_file_io/SampleFilesFolder/SampleTextFile.txt");

    match read_str_result {
        Ok(contents) => {
            // TODO : What should the read string would be ?
            let expected_string = 
            assert_eq!(expected_string, contents);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            assert!(false);
        }
    }
}
