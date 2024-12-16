use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};

pub fn read_line(file_path: &str, line: usize) -> io::Result<String> {
    let request = line - 1;
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut buffer = Vec::new();
    let mut lines = Vec::new();

    let file_size = reader.seek(SeekFrom::End(0))?;
    let mut position = file_size;

    while position > 0 {
        position -= 1;
        reader.seek(SeekFrom::Start(position))?;
        let mut byte = [0; 1];
        reader.read_exact(&mut byte)?;

        if byte[0] == b'\n' && !buffer.is_empty() {
            buffer.reverse();
            lines.push(String::from_utf8(buffer.clone()).unwrap_or_default());
            buffer.clear();

            if lines.len() > request {
                break;
            }
        } else {
            buffer.push(byte[0]);
        }
    }

    if !buffer.is_empty() {
        buffer.reverse();
        lines.push(String::from_utf8(buffer.clone()).unwrap_or_default());

        buffer.clear();
    }

    if request < lines.len() {
        println!("{:?}", lines);
        Ok(lines[request].clone())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Requested line number is out of bounds",
        ))
    }
}
