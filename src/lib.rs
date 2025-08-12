#![allow(unused)] // Make it stop!
pub mod action;
#[cfg(test)]
mod tests;

use action::{Action, ActionType, Properties};
use regex::Regex;
use std::{ffi::OsStr, fmt::Display, fs, io::Write, path::Path};

#[derive(Debug, Clone)]
pub struct VDM {
    pub actions: Vec<Action>,
    pub name: String,
}

impl Default for VDM {
    fn default() -> Self {
        Self::new()
    }
}

impl VDM {
    pub fn new() -> Self {
        VDM {
            actions: vec![],
            name: String::new(),
        }
    }

    pub fn open(file_path: impl AsRef<Path>) -> Result<VDM, &'static str> {
        let file_path = file_path.as_ref();

        if file_path.extension().is_some_and(|ext| ext == "vdm") {
            let file = match fs::read_to_string(file_path) {
                Ok(f) => f,
                Err(_) => {
                    return Err("Error Opening File.");
                }
            };

            let mut vdm = VDM::from(file);
            vdm.name = file_path
                .file_name()
                .and_then(OsStr::to_str)
                .and_then(|s| s.strip_suffix(".vdm"))
                .unwrap()
                .to_owned();

            return Ok(vdm);
        }

        Err("Invalid file type")
    }

    pub fn export(&self, file_path: &str) {
        let mut vdm_file = fs::File::create(file_path).unwrap();
        vdm_file.write_all(self.to_string().as_bytes()).unwrap();
    }

    pub fn add(&mut self, action: Action) {
        self.actions.push(action)
    }

    pub fn create_action(&mut self, factory: ActionType) -> &mut Action {
        let new_action = Action::new(factory);
        self.add(new_action);
        self.last_mut()
    }

    pub fn remove_first(&mut self) {
        self.actions.remove(0);
    }

    pub fn remove(&mut self, i: usize) {
        self.actions.remove(i);
    }

    pub fn remove_last(&mut self) {
        let i = self.len() - 1;
        self.actions.remove(i);
    }

    pub fn clear(&mut self) {
        self.actions = vec![];
    }

    pub fn len(&self) -> usize {
        self.actions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    pub fn first(&self) -> &Action {
        &self.actions[0]
    }

    pub fn nth(&self, i: usize) -> &Action {
        &self.actions[i]
    }

    pub fn last(&self) -> &Action {
        let i = self.len() - 1;
        &self.actions[i]
    }

    pub fn first_mut(&mut self) -> &mut Action {
        &mut self.actions[0]
    }

    pub fn nth_mut(&mut self, i: usize) -> &mut Action {
        &mut self.actions[i]
    }

    pub fn last_mut(&mut self) -> &mut Action {
        let i = self.len() - 1;
        &mut self.actions[i]
    }

    pub fn set_first(&mut self, new_action: Action) {
        self.actions[0] = new_action;
    }

    pub fn set_nth(&mut self, i: usize, new_action: Action) {
        self.actions[i] = new_action;
    }

    pub fn set_last(&mut self, new_action: Action) {
        let i = self.len() - 1;
        self.actions[i] = new_action;
    }

    pub fn set_first_props(&mut self, new_props: Properties) {
        self.actions[0] = self.actions[0].set_props(new_props);
    }

    pub fn set_nth_props(&mut self, i: usize, new_props: Properties) {
        self.actions[i] = self.actions[i].set_props(new_props);
    }

    pub fn set_last_props(&mut self, new_props: Properties) {
        let i = self.len() - 1;
        self.actions[i] = self.actions[i].set_props(new_props);
    }
}

impl<T: AsRef<str>> From<T> for VDM {
    fn from(file_text: T) -> Self {
        let file_text = file_text.as_ref();

        let mut vdm_actions = vec![];

        let re = Regex::new(r"demoactions\r\n\{\r\n((.|\r\n)*)\r\n\}").unwrap();
        let Some(actions) = re.captures(file_text) else {
            return VDM::new();
        };

        let actions_split = actions[1].split("}");

        for event in actions_split {
            let main_body = event.split("{").collect::<Vec<_>>();
            if main_body.len() > 1 {
                vdm_actions.push(Action::from(main_body[1].to_string()));
            }
        }

        VDM {
            actions: vdm_actions,
            name: String::new(),
        }
    }
}

impl Display for VDM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "demoactions\r\n{{\r\n")?;

        for (i, action) in self.actions.iter().enumerate() {
            write!(f, "\t\"{}\"\r\n\t{{\r\n{action}\t}}\r\n", i + 1);
        }

        write!(f, "}}\r\n")
    }
}

impl From<VDM> for String {
    fn from(vdm: VDM) -> Self {
        vdm.to_string()
    }
}
