use std::{
    collections::VecDeque,
    ffi::CString,
    mem::size_of,
    path::Path,
    thread,
    time::{self},
};

use selfprof::{active_app_name, idle_time, storage, time_now, Snap};

mod cli;

type Name = String;
type NameId = u32;

fn main() {
    let config = cli::parse();

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
    let snaps = storage::load_snaps(&snaps_path);

    let mut snaps: Vec<Snap> = Vec::with_capacity(snaps_per_save);
    const MAX_NAME_LOOKBACK: usize = size_of::<u8>();
    let mut names: VecDeque<Name> = VecDeque::with_capacity(MAX_NAME_LOOKBACK);

    loop {
        for _ in 1..=snaps_per_save {
            thread::sleep(interval);
            let idle = idle_time();
            if idle > config.idle_cutoff {
                continue;
            }
            let name = active_app_name();

            // TODO: find rindex (reverse!)
            let index = names.iter().position(|x| x == &name).unwrap_or(0);

            let snap = Snap {
                time: time_now(),
                idle: idle,
                index: index.try_into().unwrap_or(0),
                // TODO: only write if index == 0
                name: CString::new(name).expect("No nul"),
            };

            if config.verbose {
                println!("{:?}", &snap);
            }

            snaps.push(snap);
        }

        if config.verbose {
            println!("{:?}", "saving...");
        }

        storage::update_snaps(&snaps, &snaps_path, |s| s.to_bytes());

        names.clear();
        snaps.clear();
    }
}
