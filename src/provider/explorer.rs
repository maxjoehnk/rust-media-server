use super::{SharedProviders, ProviderFolder};

pub struct Explorer {
    pub path: Vec<String>,
    providers: SharedProviders
}

impl Explorer {
    pub fn new(providers: SharedProviders) -> Explorer {
        Explorer {
            path: vec![],
            providers
        }
    }

    pub fn navigate_absolute(&mut self, path: String) {
        let mut absolute = vec![];
        let mut current = path.as_str();
        while current.len() > 0 {
            let layer = match current.find('/') {
                Some(index) => {
                    let layer = &path[..index];
                    current = &path[index + 1..];
                    layer
                },
                None => {
                    let copy = current.clone();
                    current = "";
                    copy
                }
            };
            absolute.push(layer.to_owned());
        }
        self.path = absolute;
    }

    pub fn navigate(&mut self, path: String) {
        self.path.push(path);
    }

    pub fn go_up(&mut self) {
        self.path.pop();
    }

    pub fn path(&self) -> String {
        self.path
            .iter()
            .fold(String::new(), |mut a, b| {
                a.push_str(format!("{}/", b).as_str());
                a
            })
    }

    fn get_root(&self) -> ProviderFolder {
        let folders = self.providers
            .iter()
            .map(|provider| provider.lock().unwrap().root())
            .collect();
        ProviderFolder {
            label: String::from("Root"),
            folders,
            items: vec![]
        }
    }

    pub fn items(&self) -> Result<ProviderFolder, ()> {
        let root = self.get_root();
        match self.path.len() {
            0 => Ok(root),
            _ => {
                let mut folder = Some(root);
                let path = self.path.clone();
                for item in path {
                    folder = folder.ok_or(())?
                        .folders
                        .iter()
                        .cloned()
                        .find(|folder| folder.label == item);
                }
                folder.ok_or(())
            }
        }
    }
}