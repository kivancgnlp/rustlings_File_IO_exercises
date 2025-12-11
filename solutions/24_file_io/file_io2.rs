
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() {

    let input_file = fs::File::open("solutions/24_file_io/SampleFilesFolder/MultiLineTextFile.txt");

    if input_file.is_err() {
        eprintln!("Input file open error : {}", input_file.as_ref().unwrap_err());
        assert!(false);
    }

    let buffered_input_file = BufReader::new(input_file.unwrap());

    let output_file = fs::File::create("MultiLineOutputFile.txt");

    if output_file.is_err() {
        eprintln!("Output file open error : {}", output_file.as_ref().unwrap_err());
        assert!(false);
    }
    let mut buffered_file_writer = BufWriter::new(output_file.ok().unwrap());

    let mut line_number = 1;
    let mut lines = buffered_input_file.lines();
    while let Some(line) = lines.next() {
        if let Ok(line) = line {
            let write_result = buffered_file_writer.write(format!("Line {} : {}\n", line_number, line).as_bytes());
            if write_result.is_err() {
                eprintln!("Write result error: {}", write_result.unwrap_err());
                break;
            }
            line_number += 1;
        }else {
            eprintln!("Write line error : {}", line_number);
            assert!(false);
        }

    }

    println!("{} : lines processed", line_number - 1);
}