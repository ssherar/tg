#![feature(core)]
#![feature(io)]
#![feature(fs)]
#![feature(old_path)]
#![feature(path)]
#![feature(plugin)]
#![feature(rustc_private)]
#![feature(os)]
#![plugin(docopt_macros)]

extern crate docopt;
extern crate libc;
#[macro_use] extern crate log;
extern crate rustc;
extern crate "rustc-serialize" as rustc_serialize;
extern crate simple_logger;

use std::path::PathBuf;

pub use ::data::{Tags,Project,Tg};

mod data;
mod serialize;
mod utils;

docopt!(Args derive Debug, "
Usage:  tg [options] add <name> <path>
        tg [options] list
        tg [options] remove <name>
        tg [options] status
        tg (--help | --version)

Show information about tagged repositories

Subcommands:
    add                 Add a repository to the tag
    list                Show repository paths
    remove              Remove a repository from the tag
    status              Show repository statuses

Options:
    -t, --tags TAGS     Filter by tags (comma seperated)
    -c, --config PATH   Path to configuration file (default: ~/.tg)
    -v, --verbose       Show debug output
    --version           Show the version number
    -h, --help          Show this message
", flag_config: Option<String>);

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let tags: Tags = utils::split_tags(args.flag_tags);
    let path = match args.flag_config {
        Some(path) => utils::realpath(PathBuf::new(&path)),
        None       => utils::expand_home(PathBuf::new(".tg"))
    };

    if args.flag_verbose {
        simple_logger::init();
    }

    let mut tg = serialize::load(&path).unwrap();

    if args.cmd_add {
        tg.add(args.arg_name, Project::new(args.arg_path, tags)).unwrap();
    } else if args.cmd_list {
        for (name, project) in tg.projects.iter() {
            println!("- {:<10} {:?} ({:?})", name, project.path, project.tags);
        }
    } else if args.cmd_remove {
        tg.remove(args.arg_name).unwrap();
    } else if args.cmd_status {
        for (name, project) in tg.projects.iter() {
            println!("- {:<10} {}", name, project.status().unwrap());
        }
    } else if args.flag_version {
        println!("tg 0.0.1");
    }

    serialize::save(tg, &path).unwrap();
}
