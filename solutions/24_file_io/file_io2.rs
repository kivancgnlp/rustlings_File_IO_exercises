use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

const TEST_INPUT_FILE_NAME: &str = "MultiLineTextFile.txt";
const TEST_OUTPUT_FILE_NAME: &str = "MultiLineOutputFile.txt";

fn main() {
    create_required_files();

    let input_file = match fs::File::open(TEST_INPUT_FILE_NAME) {
        Ok(f) => f,
        Err(e) => panic!("Input file open error : {}", e),
    };

    let buffered_input_file = BufReader::new(input_file);

    let output_file = match fs::File::create(TEST_OUTPUT_FILE_NAME) {
        Ok(f) => f,
        Err(e) => panic!("Output file open error : {}", e),
    };

    let mut buffered_file_writer = BufWriter::new(output_file);

    let mut line_number = 1;

    for line in buffered_input_file.lines() {
        if let Ok(line) = line {
            let write_result =
                buffered_file_writer.write(format!("Line {} : {}\n", line_number, line).as_bytes());
            if write_result.is_err() {
                eprintln!("Line write error: {}", write_result.unwrap_err());
                break;
            }
            line_number += 1;
        } else {
            panic!("Line read error");
        }
    }

    println!("{} : lines processed", line_number - 1);
    file_cleanup();
}

fn create_required_files() {
    let file_path = Path::new(TEST_INPUT_FILE_NAME);

    if !file_path.exists() {
        let text = "This is the first line of the text.
        This is the second line.
        And this is the third and the last line.";
        let file_write_result = fs::write(file_path, text);

        if file_write_result.is_ok() {
            println!("Multi line file created successfully!");
        } else {
            eprintln!(
                "Error creating file : {} , error : {:?}",
                file_path.display(),
                file_write_result.err()
            );
        }
    }
}

fn file_cleanup() {
    let file_names = vec![TEST_INPUT_FILE_NAME, TEST_OUTPUT_FILE_NAME];

    for file_name in file_names {
        let file_path = Path::new(file_name);

        if file_path.exists() {
            let remove_status = fs::remove_file(file_path);
            if remove_status.is_ok() {
                println!("Successfully deleted file {}", file_name);
            } else {
                eprintln!(
                    "Error deleting file {}, err : {:?}",
                    file_name,
                    remove_status.err()
                );
            }
        } else {
            println!("No cleanup necassary since {} not exist.", file_name);
        }
    }
}
