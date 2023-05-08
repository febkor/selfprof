use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Read, Write},
    path::Path,
};

pub fn load_map(path: &Path) -> HashMap<String, u32> {
    let mut buf = Vec::<u8>::new();
    read_file_to_buf(path, &mut buf);

    let mut map: HashMap<String, u32> = HashMap::new();
    for (i, line) in buf.lines().enumerate() {
        map.insert(line.unwrap_or_default(), i.try_into().expect("Too big"));
    }
    map
}

pub fn update_names<T>(vec: &Vec<T>, path: &Path, func: fn(&T) -> &str) {
    let mut w = file_open_append(path);

    for entry in vec {
        let encoded = func(entry);
        writeln!(w, "{}", encoded).expect("wrote all names");
    }
}

pub fn update_snaps<T, const N: usize>(vec: &Vec<T>, path: &Path, func: fn(&T) -> [u8; N]) {
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
