extern crate glob;

use std::fs;
use std::io;
use std::path::Path;

fn remove_dir(dryrun: bool, path: &Path) -> bool {
    let mut all_empty = true;
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return false,
    };
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_err) => {
                all_empty = false;
                eprintln!("{}: {}", path.display(), _err);
                continue;
            }
        };
        let path1 = entry.path();
        if !path1.is_dir() {
            all_empty = false;
            // eprintln!("{}: File exists", path1.display());
            continue;
        }
        match path1.file_name() {
            None => {
                all_empty = false;
                eprintln!("{}: file_name(): empty", path1.display());
            }
            Some(name1) => {
                if name1 != "." && name1 != ".." {
                    if !remove_dir(dryrun, &path1) {
                        all_empty = false;
                    }
                }
            }
        }
    }
    if all_empty {
        println!("rmdir \"{}\"", path.display());
        if !dryrun {
            if let Err(err) = fs::remove_dir(path) {
                eprintln!("{}: {}", path.display(), err);
                return false;
            }
        }
    }
    return all_empty;
}

fn remove_any(dryrun: bool, path: &Path) -> io::Result<()> {
    let meta = fs::metadata(path)?;
    if meta.is_dir() {
        remove_dir(dryrun, path);
    }
    return Ok(());
}

fn usage() {
    eprintln!("Usage: rmdirsonly [-h] [-n] {{directories}}");
    eprintln!(" -h help");
    eprintln!(" -n dry run");
}

fn rmdirsonly(args: std::env::Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut dryrun = false;
    let mut noarg = true;
    for arg in args.skip(1) {
        noarg = false;
        if arg == "-n" {
            dryrun = true;
            continue;
        }
        if arg == "-h" {
            usage();
            return Ok(());
        }
        let mut globbed = false;
        for filename in glob::glob(&arg)? {
            if let Some(filename) = filename?.to_str() {
                remove_any(dryrun, Path::new(&filename))?;
                globbed = true;
            }
        }
        if !globbed {
            remove_any(dryrun, Path::new(&arg))?;
        }
    }
    if noarg {
        usage()
    }
    return Ok(());
}

fn main() {
    if let Err(err) = rmdirsonly(std::env::args()) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
