use jsonparser::JSONParser;

fn main() {
    let input = r#"
        {
            "name": "John Doe",
            "age": 30,
            "cars": [
                {
                    "model": "Ford",
                    "year": 2018
                },
                {
                    "model": "BMW",
                    "year": 2019
                }
            ],
            "isStudent": false,
            "address": {
                "street": "123 Main St",
                "city": "Springfield",
                "state": "IL"
            },
            "spouse": null
        }
    "#;

    let mut parser = JSONParser::new(input);

    let json = match parser.parse() {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    println!("{:#?}", json["name"].as_str());
}
