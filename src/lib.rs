mod action;

use action::{Action, Properties};
use std::{fs, io::Write};
use regex::{Regex};

#[derive(Debug, Clone)]
pub struct VDM {
    pub actions: Vec<Action>
}

impl VDM {
    fn new() -> Self {
        VDM {
            actions: vec![]
        }
    }

    fn open(file_path: &str) -> Result<VDM, &'static str> {
        if file_path.ends_with(".vdm") {
            let file;
            match fs::read_to_string(&file_path) {
                Ok(f) => {
                    file = f;
                },
                Err(_) => {
                    return Err("Error Opening File.");
                }
            };

            let vdm = VDM::from(file);
            return Ok(vdm);
        }

        Err("Invalid file type")
    }

    fn export(&self, file_path: &str) {
        let mut vdm_file = fs::File::create(file_path).unwrap();
        vdm_file.write_all(self.to_string().as_bytes()).unwrap();
    }

    fn to_string(&self) -> String {
        let mut vdm_str = "".to_string();
        for (i, action) in self.actions.iter().enumerate() {
            vdm_str.push_str(&format!("\t\"{}\"\r\n\t{{\r\n{}\t}}\r\n", i + 1, String::from(action.to_owned())));
        }

        return format!("demoactions\r\n{{\r\n{}}}\r\n", vdm_str);
    }

    fn add(&mut self, action: Action) {
        self.actions.push(action)
    }

    fn create_action(&mut self, factory: &str) -> Action {
        let new_action = Action::new(factory);
        self.add(new_action);
        return self.last()
    }

    fn remove_first(&mut self) {
        self.actions.remove(0);
    }

    fn remove(&mut self, i:usize) {
        self.actions.remove(i);
    }

    fn remove_last(&mut self) {
        let i = self.len() - 1;
        self.actions.remove(i);
    }

    fn len(&self) -> usize {
        return self.actions.len()
    }

    fn first(&self) -> Action {
        self.actions[0].clone()
    }

    fn nth(&self, i: usize) -> Action {
        self.actions[i].clone()
    }

    fn last(&self) -> Action {
        self.actions[self.len() - 1].clone()
    }

    fn set_first(&mut self, new_action: Action) {
        self.actions[0] = new_action;
    }

    fn set_nth(&mut self, i: usize, new_action: Action) {
        self.actions[i] = new_action;
    }

    fn set_last(&mut self, new_action: Action) {
        let i = self.len() - 1;
        self.actions[i] = new_action;
    }

    fn set_first_props(&mut self, new_props: Properties) {
        self.actions[0] = self.actions[0].set_props(new_props);
    }

    fn set_nth_props(&mut self, i: usize, new_props: Properties) {
        self.actions[i] = self.actions[i].set_props(new_props);
    }

    fn set_last_props(&mut self, new_props: Properties) {
        let i = self.len() - 1;
        self.actions[i] = self.actions[i].set_props(new_props);
    }
}

impl From<String> for VDM {
    fn from(file_text: String) -> Self {
        let mut vdm_actions = vec![];

        let re = Regex::new(r"demoactions\r\n\{\r\n((.|\r\n)*)\r\n\}").unwrap();
        let actions = re.captures(&file_text).unwrap();

        let actions_split = &actions[1].split("}");

        for event in actions_split.to_owned() {
            let main_body = event.split("{").collect::<Vec<_>>();
            if main_body.len() > 1 {
                vdm_actions.push(Action::from(main_body[1].to_string()));
            }
        }

        VDM {
            actions: vdm_actions
        }
    }
}

impl From<VDM> for String {
    fn from(vdm: VDM) -> Self {
        let mut vdm_str = "".to_string();
        for (i, action) in vdm.actions.iter().enumerate() {
            vdm_str.push_str(&format!("\t\"{}\"\r\n\t{{\r\n{}\t}}\r\n", i + 1, String::from(action.to_owned())));
        }

        return format!("demoactions\r\n{{\r\n{}}}\r\n", vdm_str);
    }
}
