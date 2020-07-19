# Configuer

(name inspired by `dialoguer` ;D)

Configuer is a tiny lib which will help you organize your configuration data

`Configuer::new` creates new instance of Configuer. You must provide `T` parameter which specify data model.

### Example

```rust
use configuer::Configuer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
struct MyData {
    user_name: String,
}

fn main() {
    let mut config = Configuer::with_file("myIniFileName").on_create(|| {
        println!("I see you open this app very first time, please pass your name: ...");

        MyData {
            user_name: "Default user name".into(),
        }
    });

    println!("{:?}", config.data);
    config.save();
}
```
