use serde_derive::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize)]
struct Content {
    elements: Elements,
}

impl Content {
    pub fn as_array(&self) -> [&Vec<String>; 4] {
        [
            &self.elements.people,
            &self.elements.verbs,
            &self.elements.adverbs,
            &self.elements.objects,
        ]
    }
}

#[derive(Deserialize)]
struct Elements {
    people: Vec<String>,
    verbs: Vec<String>,
    adverbs: Vec<String>,
    objects: Vec<String>,
}
mod random_gen {
    use rand::Rng;
    use std::error::Error;
    use std::process::exit;

    pub fn rand_element(e: &Vec<String>) -> &String {
        let secret_number = rand::thread_rng().gen_range(0..=e.len() - 1);
        &e[secret_number]
    }

    pub fn error_handler<F, T, E>(f: F) -> T
    where
        F: Fn() -> Result<T, E>,
        E: Error,
    {
        match f() {
            Ok(t) => t,
            Err(e) => {
                println!("An error occurred: {}", e);
                exit(1);
            }
        }
    }
}
fn main() {
    // The path to the file to read
    let path = "./data/elements.toml";

    // Read the file contents into a string, returns `io::Result<usize>`

    let raw_contents = random_gen::error_handler(|| fs::read_to_string(path));

    let contents: Content = random_gen::error_handler(|| toml::from_str(&raw_contents));

    let mut text = String::new();

    for el in contents.as_array() {
        text.push_str(&random_gen::rand_element(el));
        text.push_str(" ");
    }

    println!("{}", text);
}
