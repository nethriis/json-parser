use jsonparser::{ArrayType, BooleanType, JSONParser, JSONSchema, JSONValue, NullType, NumberType, ObjectType, StringType};

fn main() {
    let input = r#"
        {
            "name": "     John Doe     ",
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
                "state": "IL",
                "zip": 62701
            },
            "spouse": null
        }
    "#;

    let json = match JSONParser::from(input) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    println!("{:?}", json);

    let schema = JSONSchema::new([
        ("name", StringType::new().includes("John").trim().boxed()),
        ("age", NumberType::new().gt(18.0).boxed()),
        ("cars", ArrayType::new().every(ObjectType::new().boxed()).boxed()),
        ("isStudent", BooleanType::new().falsy().boxed()),
        ("address", ObjectType::new()
            .property("street", StringType::new().boxed())
            .property("city", StringType::new().boxed())
            .property("state", StringType::new().boxed())
            .property("zip", NumberType::new().gt(62700.0).boxed())
        .boxed()),
        ("spouse", NullType::new().boxed())
    ]);

    match schema.validate(&json) {
        Ok(value) => println!("{:?}", value),
        Err(e) => eprintln!("Invalid JSON: {}", e)
    }
}
