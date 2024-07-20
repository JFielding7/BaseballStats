use std::fmt::{Display};
use std::fs::File;
use std::io::{BufReader, Read, Result, Seek, SeekFrom};

fn get_line_length(file: &File) -> u64 {
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1];

    let mut i = 1;
    while reader.read(&mut buffer).is_ok() {
        let char = buffer[0];
        if char == b'\n' {
            return i;
        }
        i += 1;
    }
    i
}

pub(crate) fn get_id(file: &str, key: &String, id_len: usize) -> Result<i32> {
    let bytes = key.as_bytes();
    let mut player_file = File::open(file)?;
    let file_len = player_file.metadata().unwrap().len();
    let line_len: u64 = get_line_length(&player_file);
    if key.len() > (line_len as usize) - id_len - 2 {
        return Ok(-1);
    }

    let mut start = 0;
    let mut end = file_len;
    let mut buffer: Box<[u8]> = vec![0; line_len as usize].into_boxed_slice();
    while start < end {
        let mid = (start + end >> 1) / line_len * line_len;
        player_file.seek(SeekFrom::Start(mid))?;
        player_file.read_exact(&mut buffer)?;

        let mut cmp: i8 = 0;
        for i in 0..buffer.len() {
            if buffer[i] == b' ' {
                if i != bytes.len() {
                    cmp = 1;
                }
                break;
            }
            if i == bytes.len() {
                cmp = -1;
                break;
            }
            cmp = (bytes[i] as i8) - (buffer[i] as i8);
            if cmp != 0 {
                break;
            }
        }

        if cmp == 0 {
            player_file.seek(SeekFrom::Start(mid + line_len - (id_len as u64) - 1))?;
            let mut id_buffer: Box<[u8]>= vec![0; id_len].into_boxed_slice();
            player_file.read_exact(&mut id_buffer)?;
            return Ok(String::from_utf8_lossy(&id_buffer).parse::<i32>().unwrap());
        }
        else if cmp > 0 {
            start = mid + line_len;
        }
        else {
            end = mid;
        }
    }
    Ok(-1)
}