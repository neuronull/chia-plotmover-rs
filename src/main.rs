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

fn get_free_space(hdds: &mut Vec<String>, plot_sz: u64) -> Result<&str, Box<dyn Error>> {
    let mut remove_dirs = vec![];

    let mut hdd_idx: i8 = -1;
    let sys = System::new_all();

    // if any drive has space, use it
    for (i, path) in hdds.iter().enumerate() {
        if hdd_has_space(&path, &sys, plot_sz) {
            info!("hdd {:?} has space, using that", path);
            return Ok(&hdds[i]);
        }
    }

    // otherwise, remove a plot
    for (i, path) in hdds.iter().enumerate() {
        let legacy_path = format!("{}{}", path, "/legacy_plots");

        let legacy_plots = fs::read_dir(&legacy_path)?;
        let remove_plot = legacy_plots.into_iter().last();
        if remove_plot.is_none() {
            remove_dirs.push(hdds[i].clone());
        } else {
            let path_buf = remove_plot.unwrap()?.path();
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
    for path in &remove_dirs {
        info!(
            "no plots to remove in path {}, removing it from the list",
            path
        );
        hdds.retain(|path_| path != path_);
        hdd_idx = hdd_idx - 1;
    }
    if hdd_idx == -1 {
        panic!("no plots were available to remove! are we done re-plotting ?!?!?!")
    }
    Ok(&hdds[hdd_idx as usize])
}

fn move_file(source: &DirEntry, hdds: &mut Vec<String>) {
    let source_file = String::from(source.file_name().to_str().unwrap());
    let source_path = String::from(source.path().to_str().unwrap());
    let source_sz = fs::metadata(&source_path).unwrap().len();

    let free_path = match get_free_space(hdds, source_sz) {
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

fn check_path(hdds: &mut Vec<String>, path: &str) {
    info!("checking for plot files in path {}", path);
    let files = fs::read_dir(path).unwrap();
    files
        .filter_map(Result::ok)
        .filter(|f| f.path().extension().unwrap_or(&OsString::from("foo")) == "plot")
        .for_each(|f| move_file(&f, hdds));
}

fn check_all(ssds: &Vec<String>, hdds: &mut Vec<String>) {
    for path in ssds {
        check_path(hdds, path);
    }
}

fn main() {
    // init logging
    log4rs::init_file("logcfg.yml", Default::default()).unwrap();

    // parse config
    let cfg = Cfg::new().unwrap();
    let ssds = cfg.dirs.ssds;
    let mut hdds = cfg.dirs.hdds;

    check_all(&ssds, &mut hdds);

    info!("monitoring these dirs for new plots {:?}", &ssds);

    // setup the channel and watch the dirs for new plots
    let (tx, rx) = channel();
    let mut watcher = raw_watcher(tx).unwrap();

    for path in &ssds {
        watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
    }

    loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(_op),
                cookie: _,
            }) => {
                check_path(&mut hdds, path.parent().unwrap().to_str().unwrap());
            }
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
