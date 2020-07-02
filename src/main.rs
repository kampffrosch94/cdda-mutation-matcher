use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Mutation {
    id: String,
    name: Name,
    #[serde(default)]
    cancels: Vec<String>,
    #[serde(default)]
    category: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Name {
    str: String,
}

fn main() -> anyhow::Result<()>{
    let s: String = std::fs::read_to_string("mutations.json")?;
    let mut data: Vec<Mutation> = serde_json::from_str(&s)?;
    data.sort_by_key(|it| it.name.str.clone());
    for d in data.iter() {
        let mut cancellers = vec![];
        for x in data.iter().filter(|it| !it.category.is_empty()) {
            if x.cancels.contains(&d.id) {
                cancellers.push(x.name.str.clone())
            }
        }
        if !cancellers.is_empty() {
            println!("{} cancelled by {:?}", d.name.str, cancellers);
        }
    }
    Ok(())
}
