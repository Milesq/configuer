use serde::{Deserialize, Serialize};
use std::{
    default::Default,
    fs::{File, OpenOptions},
    path::Path,
};

pub trait Model<'de>: Serialize + Deserialize<'de> + Clone + Default {}

impl<'de, T: Serialize + Deserialize<'de> + Clone + Default> Model<'de> for T {}

#[derive(Clone)]
pub struct Configuer<'de, T: Model<'de>> {
    pub(crate) file_name: String,
    pub data: T,
}

impl<'de, T: Model<'de>> Configuer<'de, T> {
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
        let writer = OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.file_name())
            .unwrap();

        bincode::serialize_into(writer, &self.data).unwrap();
        self
    }

    pub fn load(self) -> Self {
        let reader = File::open(self.file_name()).map(|r| {
            bincode::deserialize_from::<_, T>(r).expect("Config file is damaged");
        });

        self
    }

    #[cfg(feature = "dirs")]
    fn file_name(&self) -> String {
        let mut file_path = dirs::home_dir().unwrap();
        file_path.push(self.file_name.clone());

        file_path.to_str().unwrap().to_string()
    }

    #[cfg(not(feature = "dirs"))]
    fn file_name(&self) -> String {
        self.file_name.clone()
    }
}
