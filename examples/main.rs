use configuer::Configuer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
struct MyData {
    user_name: String,
}

fn main() {
    let config = Configuer::with_file("myIniFileName")
        .on_create(|| {
            println!("I see you open this app very first time, please pass your name: ...");

            MyData {
                user_name: "Default user name".into(),
            }
        })
        .load();

    println!("{}", config.data.user_name);
}
