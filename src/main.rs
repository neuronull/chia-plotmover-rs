mod cfg;
use cfg::Cfg;
use log::{debug, error, info};
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::error::Error;
use std::sync::mpsc::channel;
use std::{
    ffi::OsString,
    fs::{self, DirEntry},
};
use sysinfo::{DiskExt, System, SystemExt};
#[macro_use]
extern crate lazy_static;

lazy_static! {
    // parse config
    static ref CFG : Cfg  = Cfg::new().unwrap();
    static ref SSDS : &'static Vec<String> = &CFG.dirs.ssds;
    static ref HDDS : &'static Vec<String> = &CFG.dirs.hdds;
    static ref ONLY_REPLACE : bool = CFG.options.only_replace.unwrap_or(false);
}

// check if any hdd has space for the new plot
fn hdd_has_space(hdd: &str, sys: &System, plot_sz: u64) -> bool {
    for disk in sys.disks() {
        let sys_path = disk.mount_point().to_str().unwrap();
        debug!("{} , {} , {}", sys_path, hdd, sys_path == hdd);
        if sys_path == hdd {
            if disk.available_space() >= plot_sz {
                return true;
            } else {
                return false;
            }
        }
    }
    panic!("hdd {} not found in system disks!", hdd);
}

fn get_free_space(plot_sz: u64) -> Result<&'static str, Box<dyn Error>> {
    let mut hdd_idx: i8 = -1;
    let sys = System::new_all();

    // if any drive has space, use it
    if *ONLY_REPLACE == false {
        for (i, path) in HDDS.iter().enumerate() {
            if hdd_has_space(&path, &sys, plot_sz) {
                info!("hdd {:?} has space, using that", path);
                return Ok(&HDDS[i]);
            }
        }
    }

    // otherwise, remove a plot
    for (i, path) in HDDS.iter().enumerate() {
        let legacy_path = format!("{}{}", path, "/legacy_plots");

        let legacy_plots = fs::read_dir(&legacy_path)?;
        if let Some(remove_plot) = legacy_plots.into_iter().last() {
            let path_buf = remove_plot?.path();
            let res = fs::remove_file(&path_buf);
            if res.is_err() {
                error!("unable to remove {:?} err= {}", &path_buf, res.unwrap_err());
                continue;
            }
            info!("removed plot {:?}", path_buf.to_str().unwrap());
            hdd_idx = i as i8;
            break;
        }
    }
    if hdd_idx < 0 {
        panic!("no plots were available to remove! are we done re-plotting ?!?!?!")
    }
    Ok(&HDDS[hdd_idx as usize])
}

fn move_file(source: &DirEntry) {
    let source_file = String::from(source.file_name().to_str().unwrap());
    let source_path = String::from(source.path().to_str().unwrap());
    let source_sz = fs::metadata(&source_path).unwrap().len();

    let free_path = match get_free_space(source_sz) {
        Ok(path) => path,
        Err(e) => {
            error!("No free space found, aborting the move! err: {}", e);
            return;
        }
    };
    let dest_path = format!("{}{}{}", free_path, "/pool_plots/", &source_file);

    info!("copy plot {:?} to {:?} ... ", &source_path, dest_path);

    match fs::copy(&source_path, &dest_path) {
        Ok(_) => {
            info!(" ... completed");
            let res_rm = fs::remove_file(&source_path);
            if res_rm.is_err() {
                error!(
                    "unable to remove {}! err: {}",
                    source_path,
                    res_rm.unwrap_err()
                );
            }
        }
        Err(e) => {
            error!(
                "unable to copy {} to {}! err: {}",
                &source_path, &dest_path, e
            );
        }
    }
}

fn check_path(path: &str) {
    info!("checking for plot files in path {}", path);
    let files = fs::read_dir(path).unwrap();
    files
        .filter_map(Result::ok)
        .filter(|f| f.path().extension().unwrap_or(&OsString::from("foo")) == "plot")
        .for_each(|f| move_file(&f));
}

fn check_all() {
    for path in SSDS.iter() {
        check_path(path);
    }
}

fn main() {
    // init logging
    log4rs::init_file("logcfg.yml", Default::default()).unwrap();

    check_all();

    info!("monitoring these dirs for new plots {:?}", &*SSDS);

    // setup the channel and watch the dirs for new plots
    let (tx, rx) = channel();
    let mut watcher = raw_watcher(tx).unwrap();

    for path in SSDS.iter() {
        watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
    }

    loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(_op),
                cookie: _,
            }) => {
                check_path(path.parent().unwrap().to_str().unwrap());
            }
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
