mod configuer;

pub use configuer::*;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Default)]
    struct DataModel {
        points: i32,
    }

    #[test]
    #[should_panic(expected = "file created")]
    fn default_value_works() {
        Configuer::with_file("random").on_create(|| {
            panic!("file created");
        });
    }

    #[test]
    fn saving_and_reading_works_between_sessions() {
        {
            let mut config = Configuer::<DataModel>::with_file("my");
            config.data.points = 3;
            config.save();
        }

        let config = Configuer::<DataModel>::with_file("my");

        assert_eq!(config.data.points, 3);
    }
}
