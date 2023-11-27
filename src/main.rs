use std::{
    ffi::CString,
    path::Path,
    thread,
    time::{self},
};

use selfprof::{active_app_name, idle_time, storage, time_now, RingBuf, Snap};

mod cli;

type Name = CString;

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

    const MAX_NAME_LOOKBACK: usize = u8::MAX as usize;
    let mut snaps: Vec<Snap> = Vec::with_capacity(snaps_per_save);
    let mut names: RingBuf<Name> = RingBuf::with_capacity(MAX_NAME_LOOKBACK);

    {
        // Load snaps to initialize names
        let snaps_prev = storage::load_snaps(&snaps_path);
        for snap in snaps_prev {
            if config.verbose {
                println!("Loaded {:?}", snap);
            }

            names.push(snap.name);
        }
    }

    loop {
        for _ in 0..snaps_per_save {
            thread::sleep(interval);
            let idle = idle_time();
            if idle > config.idle_cutoff {
                continue;
            }
            let time = time_now();
            let name = CString::new(active_app_name()).expect("No nul");
            let index: u8 = names
                .iter()
                .rev()
                .position(|x| x == &name)
                .unwrap_or(MAX_NAME_LOOKBACK)
                .try_into()
                .unwrap_or(MAX_NAME_LOOKBACK as u8);
            let name_exists = index < MAX_NAME_LOOKBACK as u8;

            let snap = Snap {
                time,
                idle,
                index,
                name: if name_exists {
                    CString::default()
                } else {
                    name.clone()
                },
            };

            if config.verbose {
                println!("{:?}", &snap);
            }

            if !name_exists {
                names.push(name);
            }

            snaps.push(snap);
        }

        if config.verbose {
            println!("{:?}", "saving...");
        }

        storage::update_snaps(&snaps, &snaps_path, |s| s.to_bytes());

        snaps.clear();
    }
}
