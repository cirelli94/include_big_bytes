use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./program.exe").unwrap();
    let mut buffer = vec![];
    file.read_to_end(&mut buffer)?;
    let mut position = 0;

    for (i, val) in buffer.iter().enumerate() {
        if *val == b'F'
            && buffer[i + 1] == b'O'
            && buffer[i + 2] == b'O'
            && buffer[i + 3] == b'T'
            && buffer[i + 4] == b'E'
            && buffer[i + 5] == b'R'
        {
            position = i;
            break;
        }
    }

    let footer = buffer.split_at(position).1;
    let footer = String::from_utf8(footer.to_vec()).unwrap();

    let indexes: Vec<&str> = footer.trim().split(';').collect();
    let start = indexes[1].parse::<usize>().unwrap();
    let end = start + indexes[2].parse::<usize>().unwrap();

    let mut file_dest = File::create("./data").unwrap();

    let buff_dest = &buffer[start..end];
    file_dest.write_all(buff_dest)?;

    Ok(())
}
