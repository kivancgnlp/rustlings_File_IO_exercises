use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

const TEST_FILE_NAME: &str = "MultiLineTextFile.txt";

fn main() {
    create_required_files();
    let input_file = fs::File::open(TEST_FILE_NAME);

    if input_file.is_err() {
        panic!("Input file open error");
    }

    let buffered_input_file = BufReader::new(input_file.unwrap());

    let output_file = fs::File::create("MultiLineOutputFile.txt");

    if output_file.is_err() {
        eprintln!(
            "Output file open error : {}",
            output_file.as_ref().unwrap_err()
        );
        panic!("Output file open error");
    }
    let mut buffered_file_writer = BufWriter::new(output_file.ok().unwrap());

    let mut line_number = 1;

    for line in buffered_input_file.lines() {
        if let Ok(line) = line {
            let write_result =
                buffered_file_writer.write(format!("Line {} : {}\n", line_number, line).as_bytes());
            if write_result.is_err() {
                eprintln!("Write result error: {}", write_result.unwrap_err());
                break;
            }
            line_number += 1;
        } else {
            panic!("Write line error");
        }
    }

    println!("{} : lines processed", line_number - 1);
}

fn create_required_files() {
    let file_path = Path::new(TEST_FILE_NAME);

    if !file_path.exists() {
        let text = "This is the first line of the text.
        This is the second line.
        And this is the third and the last line.";
        fs::write(file_path, text).unwrap();
        println!("File created.");
    }
}
