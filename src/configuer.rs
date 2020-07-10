use serde::{Deserialize, Serialize};
use std::{default::Default, path::Path};

pub trait Model: Serialize + Deserialize<'static> + Clone + Default {}

impl<T: Serialize + Deserialize<'static> + Clone + Default> Model for T {}

#[derive(Clone)]
pub struct Configuer<T: Model> {
    pub(crate) file_name: String,
    pub data: T,
}

impl<T: Model> Configuer<T> {
    pub fn with_file(file_name: &str) -> Self {
        Self {
            file_name: String::from(file_name),
            data: Default::default(),
        }
    }

    pub fn on_create(mut self, cb: impl FnOnce() -> T) -> Self {
        if !Path::new(&self.file_name()).exists() {
            self.data = cb();
        }

        self
    }

    pub fn save(&mut self) -> &Self {
        self
    }

    pub fn load(self) -> Self {
        if !Path::new(&self.file_name()).exists() {}

        self
    }

    fn file_name(&self) -> String {
        if cfg!(feature = "dirs") {
            let mut file_path = dirs::home_dir().unwrap();
            file_path.push(self.file_name.clone());

            file_path.to_str().unwrap().to_string()
        } else {
            self.file_name.clone()
        }
    }
}
