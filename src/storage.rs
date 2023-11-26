use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

use crate::Snap;

pub fn load_snaps(path: &Path) -> Vec<Snap> {
    let mut buf = Vec::<u8>::new();
    read_file_to_buf(path, &mut buf);

    let mut res: Vec<Snap> = Vec::new();
    let mut i = 0;
    while i < buf.len() {
        let snap = Snap::from_bytes(&buf[i..]);
        i += snap.count_bytes();
        res.push(snap);
    }

    res
}

pub fn update_snaps<T>(vec: &Vec<T>, path: &Path, func: fn(&T) -> Vec<u8>) {
    let mut w = file_open_append(path);

    for entry in vec {
        let encoded = func(entry);
        w.write_all(&encoded).expect("wrote all snaps");
    }
}

fn file_open_append(path: &Path) -> BufWriter<File> {
    let f = File::options()
        .create(true)
        .append(true)
        .open(path)
        .expect(&format!("could not open file path {:?}", &path.to_str()));

    BufWriter::new(f)
}

fn read_file_to_buf(path: &Path, buf: &mut Vec<u8>) {
    let f = File::options()
        .create(true)
        .write(true) // for create to work
        .read(true)
        .open(path)
        .expect(&format!("could not open file path {:?}", &path.to_str()));
    let mut buf_reader = BufReader::new(f);
    buf_reader.read_to_end(buf).expect("Error reading file");
}
