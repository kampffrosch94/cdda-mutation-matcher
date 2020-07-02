use std::collections::HashSet;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Mutation {
    id: String,
    name: Name,
    #[serde(default)]
    cancels: Vec<String>,
    #[serde(default)]
    category: Vec<String>,
    #[serde(default)]
    threshreq: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Name {
    str: String,
}

fn main() -> anyhow::Result<()> {
    let s: String = std::fs::read_to_string("mutations.json")?;
    let mut data: Vec<Mutation> = serde_json::from_str(&s)?;
    data.sort_by_key(|it| it.name.str.clone());

    println!("Mutation cancels:");
    for d in data.iter() {
        let mut cancellers = Vec::new();
        for x in data.iter().filter(|it| !it.category.is_empty()) {
            if x.cancels.contains(&d.id) {
                if !x.threshreq.is_empty() {
                    cancellers.push(format!("{} (threshold)", x.name.str.clone()));
                } else {
                    cancellers.push(x.name.str.clone())
                }
            }
        }
        if !cancellers.is_empty() {
            println!("{} cancelled by {:?}", d.name.str, cancellers);
        }
    }
    println!("\n\n\nMutation categories");
    let mut categories = HashSet::new();
    for d in data.iter().filter(|it| !it.category.is_empty()) {
        if d.threshreq.is_empty() {
            println!("{} in {:?}", d.name.str, d.category);
        } else {
            println!("{} in {:?} (threshold required)", d.name.str, d.category);
        }
        for c in d.category.iter() {
            categories.insert(c.clone());
        }
    }

    println!("\n\n\nMutation by category");
    let mut categories: Vec<_> = categories.iter().collect();
    categories.sort();

    for c in categories {
        let mut mutations = Vec::new();
        let mut threshold_mutations = Vec::new();
        for d in data.iter() {
            if d.category.contains(c) {
                if !d.threshreq.is_empty() {
                    threshold_mutations.push(d.name.str.clone());
                } else {
                    mutations.push(d.name.str.clone());
                }
            }
        }
        println!("\n{}", c);
        mutations.sort();
        mutations.iter().for_each(|it| println!("{}", it));
        println!("\n{} post threshold", c);
        threshold_mutations.sort();
        threshold_mutations.iter().for_each(|it| println!("{}", it));
    }
    Ok(())
}
