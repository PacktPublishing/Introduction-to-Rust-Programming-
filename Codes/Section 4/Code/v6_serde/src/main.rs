use serde_derive::*;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    name: String,
    age: Option<i32>,
    dob: Option<String>,
    children: Vec<Person>,
}

fn main() -> anyhow::Result<()> {
    let mut v = Vec::new();
    for a in std::env::args().skip(1) {
        let mut s = String::new();
        std::fs::File::open(a)?.read_to_string(&mut s)?;
        let p: Vec<Person> = serde_json::from_str(&s)?;
        v.extend(p.into_iter());
    }

    println!("{}", toml::to_string(&v)?);

    Ok(())
}
