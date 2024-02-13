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
jsonparser = "0.1.3"
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

## Contribution

Contributions are welcome! If you have suggestions for improvements or find any issues, please open an issue or submit a pull request on [GitHub](https://github.com/nethriis/json-parser).

## License

This project is licensed under the [Apache-2.0](https://github.com/nethriis/json-parser/blob/main/LICENSE) file for details.
