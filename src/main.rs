use std::{
    collections::HashMap,
    path::Path,
    thread,
    time::{self},
};

use selfprof::{active_app_name, idle_time, storage, time_now, Snap};

mod main_cli;

type Name = String;
type NameId = u32;

fn main() {
    let config = main_cli::parse();

    let interval_snap: u32 = u32::max(config.interval_snap, 1);
    let interval_save: u32 = u32::max(config.interval_save, 1);

    let snaps_per_save: usize = (interval_save / interval_snap)
        .try_into()
        .expect("snaps per save is of reasonable size");

    assert!(interval_save >= interval_snap);
    assert!(snaps_per_save >= 1);
    let interval = time::Duration::from_secs(interval_snap.into());

    let dir_path = &Path::new(&config.out_dir);
    std::fs::create_dir_all(dir_path).expect("create output directory");
    let snaps_path = dir_path.join("selfprof.dat");
    let names_path = dir_path.join("selfprof.txt");

    let mut snaps: Vec<Snap> = Vec::with_capacity(snaps_per_save);
    let mut names: Vec<Name> = Vec::with_capacity(snaps_per_save);
    let mut name_ids: HashMap<Name, NameId> = storage::load_map(&names_path);

    loop {
        for _ in 1..=snaps_per_save {
            thread::sleep(interval);
            let idle = idle_time();
            let name = active_app_name();

            let name_id = match name_ids.get(&name) {
                Some(id) => *id,
                None => {
                    names.push(name.clone());
                    let id = name_ids.len().try_into().unwrap_or(std::u32::MAX);
                    name_ids.insert(name.clone(), id);
                    id
                }
            };

            let snap = Snap {
                time: time_now(),
                name: name_id,
                idle,
                pad: 0,
            };

            if config.verbose {
                println!("{:?}", &name);
                println!("{:?}", &snap);
            }

            snaps.push(snap);
        }

        if config.verbose {
            println!("{:?}", "saving...");
        }

        storage::update_names(&names, &names_path, |s| s);
        storage::update_snaps(&snaps, &snaps_path, |s| s.to_bytes());

        names.clear();
        snaps.clear();
    }
}
