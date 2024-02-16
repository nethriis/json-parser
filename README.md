# JSON Parser Crate

This Rust crate provides a robust and efficient solution for parsing JSON data into Rust's strongly-typed data structures. It's designed to simplify the process of converting JSON text into Rust types, such as structs, enums, and other native data types, making it easier to work with JSON data in Rust applications.

## Features

- **Strong Typing**: Leverages Rust's type system to ensure that JSON data is parsed into the correct Rust data structures.
- **Error Handling**: Provides comprehensive error handling to catch and manage parsing errors, making your application more robust.
- **Custom Deserialization**: Supports custom deserialization logic, allowing for complex data parsing scenarios.
- **Serde Support**: Offers optional integration with `serde` for serialization and deserialization, enabling compatibility with the Rust ecosystem's de facto serialization framework.

## Getting Started

To use this crate in your project, add it to your `Cargo.toml`:

```toml
[dependencies]
jsonparser = "0.1.8"
```

Then, import it in your Rust file:

```rust
use jsonparser::*;
```

## Basic Usage

### Parsing JSON to Rust

To parse a JSON string into a Rust `JSONValue`, use the `from` function provided by the crate:

```rust
use jsonparser::JSONParser;

let input = r#"
    {
        "name": "John Doe",
        "age": 30,
        "is_student": false
    }
"#;

match JSONParser::from(input) {
    Ok(json) => println!("{:#?}", json),
    Err(e) => eprintln!("Failed to parse JSON: {}", e),
}
```

### Accessing Data

Once parsed, access the data using the `.get()` method or the indexing syntax for both objects and arrays:

```rust
// Using `.get()` method
if let Some(name) = json.get("name").and_then(|v| v.as_str()) {
    println!("Name: {}", name);
}

// Using indexing syntax
let age = &json["age"];
if let Some(age) = age.as_f64() {
    println!("Age: {}", age);
}
```

### Serialization

To serialize a Rust data structure into a JSON string, use the `serialize` method:

```rust
use jsonparser::{JSONParser, JSONValue};

let json: JSONValue = JSONParser::from(r#"{ "name": "John Doe", "age": 30, "is_student": false }"#).unwrap();
let serialized = json.serialize();

assert_eq!(serialized, r#"{"name":"John Doe","age":30,"is_student":false}"#);
```

### Validation

To validate a JSONValue, use the `validate` method:

```rust
use jsonparser::{JSONParser, JSONValue, JSONSchema, StringType, NumberType, BooleanType};

let json: JSONValue = JSONParser::from(r#"{ "name": "John Doe", "age": 30, "is_student": false }"#).unwrap();
let schema = JSONSchema::new([
    ("name", StringType::new().min_length(3).max_length(50).boxed()),
    ("age", NumberType::new().minimum(18).maximum(100).boxed()),
    ("is_student", BooleanType::new().falsy().boxed()),
]);

assert!(schema.validate(&json).is_ok());
```

## Contribution

Contributions are welcome! If you have suggestions for improvements or find any issues, please open an issue or submit a pull request on [GitHub](https://github.com/nethriis/json-parser).

## License

This project is licensed under the [Apache-2.0 license](https://github.com/nethriis/json-parser/blob/main/LICENSE).
