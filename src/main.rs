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
extern crate rustc;
extern crate "rustc-serialize" as rustc_serialize;

use std::path::PathBuf;

mod serialize;

docopt!(Args derive Debug, "
Usage:  tg [options] add <name> <path>
        tg [options] remove <name>
        tg [options] list
        tg (--help | --version)

Show information about tagged repositories

Subcommands:
    add                 Add a repository to the tag
    journal             Show commits in order for a given period
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

#[derive(Debug,RustcDecodable,RustcEncodable)]
struct Project {
    path: String,
    tags: Vec<String>
}

impl Project {
    fn realpath(path: String) -> String {
        realpath(PathBuf::new(&path))
            .into_os_string()
            .to_string_lossy()
            .to_string()
    }

    pub fn new(path: String, tags: Vec<String>) -> Self {
        Project { path: Project::realpath(path), tags: tags }
    }
}

fn expand_home(path: PathBuf) -> PathBuf {
    std::env::home_dir().unwrap().join(&path)
}

/// Get the absolute path to a given path
///
/// Yes, this depends on private rustc methods...
fn realpath(path: PathBuf) -> PathBuf {
    let old_path = std::old_path::Path::new(&path.to_str().unwrap());
    let real_path = rustc::util::fs::realpath(&old_path).unwrap();
    PathBuf::new(&real_path.as_str().unwrap())
}

macro_rules! verbose(
    ($verbose:expr, $($arg:expr),+) => { if $verbose { println!($($arg),+); } }
);

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    let tags = args.flag_tags.split(',').map(|t| t.to_string()).collect();
    let path = match args.flag_config {
        Some(path) => realpath(PathBuf::new(&path)),
        None => expand_home(PathBuf::new(".tg"))
    };

    let mut projects = serialize::load(&path).unwrap();
    verbose!(args.flag_verbose, "Loaded projects from {:?}", path);

    if args.cmd_add {
        if projects.contains_key(&args.arg_name) {
            panic!("Project {} already exists", args.arg_name);
        }

        let project = Project::new(args.arg_path, tags);
        verbose!(args.flag_verbose, "Adding {:?} from {:?} with tags {:?}",
            args.arg_name, project.path, project.tags);
        projects.insert(args.arg_name, project);
    } else if args.cmd_list {
        for (name, project) in projects.iter() {
            println!("- {:<10} {:?} ({:?})", name, project.path, project.tags);
        }
    } else if args.cmd_remove {
        projects.remove(&args.arg_name);
        verbose!(args.flag_verbose, "Removed project {:?}", args.arg_name);
    } else if args.flag_version {
        println!("tg 0.0.1");
    }

    serialize::save(projects, &path).unwrap();
}
