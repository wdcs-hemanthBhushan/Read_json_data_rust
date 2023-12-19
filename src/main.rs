use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Paragraph {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}
fn main() {
    let json = r#"{
    "article":"what ever",
    "author" :"3erg3rg",
    "paragraph":[
        {"name":"check 111"}
    ]
   }"#;

    let parsed: Article = read_json_type(json);
    println!("the parsed data {}", parsed.paragraph[0].name); 
}

fn read_json_type(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    parsed
}
