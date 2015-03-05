/// Data types used to store and modify projects

use std::collections::HashMap;
use std::default::Default;
use std::process::Command;

use ::utils::realpath_string;

pub type Tags = Vec<String>;

#[derive(Debug,RustcDecodable,RustcEncodable)]
pub struct Project {
    // This uses string instead of Path, as `Path`/`PathBuf` encode strangely
    pub path: String,
    pub tags: Tags
}

impl Project {
    pub fn new(path: String, tags: Vec<String>) -> Self {
        Project { path: realpath_string(path), tags: tags }
    }

    fn git(&self) -> &mut Command {
        Command::new("git").arg("-C").arg(self.path.as_slice())
    }

    pub fn status(&self) -> Result<String, String> {
        let command = self.git().arg("status").arg("--porcelain");
        let output = command.output().unwrap();
        Ok(String::from_utf8_lossy(output.stdout.as_slice()))
    }
}

#[derive(Debug,RustcDecodable,RustcEncodable)]
pub struct Tg {
    pub projects: HashMap<String, Project>
}

impl Tg {
    pub fn add(&mut self, name: String, project: Project) -> Result<(), String> {
        info!("Adding {:?} from {:?} with tags {:?}",
            name, project.path, project.tags);

        if self.projects.contains_key(&name) {
            return Err(format!("Project {} already exists", name));
        }

        self.projects.insert(name, project);
        return Ok(());
    }

    pub fn remove(&mut self, name: String) -> Result<(), String> {
        if !self.projects.contains_key(&name) {
            return Err(format!("Project {:?} does not exist", name));
        }

        info!("Removing project {:?}", name);
        self.projects.remove(&name);
        return Ok(());
    }
}

impl Default for Tg {
    fn default() -> Self {
        Tg { projects: HashMap::new() }
    }
}
