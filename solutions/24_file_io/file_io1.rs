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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_directory() {
        let working_directory_result = std::path::Path::new(".").canonicalize();

        match working_directory_result {
            Ok(working_directory) => {
                println!("The working directory is {:?}", working_directory);
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    }
}