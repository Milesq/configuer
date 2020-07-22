use serde::{Deserialize, Serialize};
use std::{
    default::Default,
    fs::{File, OpenOptions},
    path::Path,
};

pub trait Model<'de>: Serialize + Deserialize<'de> + Clone + Default {}

impl<'de, T: Serialize + Deserialize<'de> + Clone + Default> Model<'de> for T {}

#[derive(Clone)]
pub struct Configuer<T: for<'de> Model<'de>> {
    file_name: String,
    pub data: T,
}

impl<T: for<'de> Model<'de>> Configuer<T> {
    pub fn with_file(file_name: &str) -> Self {
        let data = if let Ok(r) = File::open(Self::file_path(file_name.to_string())) {
            bincode::deserialize_from::<_, T>(r).expect("Config file is damaged")
        } else {
            Default::default()
        };

        Self {
            file_name: String::from(file_name),
            data,
        }
    }

    pub fn on_create(mut self, cb: impl FnOnce() -> T) -> Self {
        if !Path::new(&Self::file_path(self.file_name.clone())).exists() {
            self.data = cb();
        }
        self.save();

        self
    }

    pub fn save(&mut self) -> &Self {
        let writer = OpenOptions::new()
            .write(true)
            .create(true)
            .open(Self::file_path(self.file_name.clone()))
            .unwrap();

        bincode::serialize_into(writer, &self.data).unwrap();
        self
    }

    #[cfg(feature = "dirs")]
    fn file_path(file_name: String) -> String {
        let mut file_path = dirs::home_dir().unwrap();
        file_path.push(file_name.clone());

        file_path.to_str().unwrap().to_string()
    }

    #[cfg(not(feature = "dirs"))]
    fn file_path(file_name: String) -> String {
        file_name.clone()
    }
}
