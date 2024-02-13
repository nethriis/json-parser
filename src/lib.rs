mod utils;

use utils::{Lexer, Parser};
pub use utils::{JSONValue, OrderedMap};

/// A JSON parser that can parse a JSON input string to a JSONValue.
///
/// # Example
///
/// ```no_run
/// use jsonparser::JSONParser;
///
/// let input = r#"
///   {
///     "name": "John Doe",
///     "age": 30
///   }
/// "#;
///
/// let mut parser = JSONParser::new(input);
/// ```
pub struct JSONParser<'a> {
    pub parser: Parser<'a>,
}

impl<'a> JSONParser<'a> {
    /// Create a new JSONParser instance with the given input string.
    ///
    /// # Example
    ///
    /// ```
    /// use jsonparser::JSONParser;
    ///
    /// let input = r#"
    ///   {
    ///     "name": "John Doe",
    ///     "age": 30
    ///   }
    /// "#;
    ///
    /// let mut parser = JSONParser::new(input);
    /// ```
    pub fn new(input: &'a str) -> Self {
        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);

        Self { parser }
    }

    /// Parse the JSON input to a JSONValue.
    ///
    /// # Example
    ///
    /// ```
    /// use jsonparser::JSONParser;
    ///
    /// let input = r#"
    ///   {
    ///     "name": "John Doe",
    ///     "age": 30
    ///   }
    /// "#;
    ///
    /// let mut parser = JSONParser::new(input);
    /// let json = match parser.parse() {
    ///   Ok(value) => value,
    ///   Err(e) => {
    ///     eprintln!("Error: {}", e);
    ///     return;
    ///   }
    /// };
    ///
    /// println!("{:#?}", json["name"].as_str());
    /// ```
    pub fn parse(&mut self) -> Result<JSONValue, String> {
        self.parser.parse()
    }

    /// Parse the JSON input to a JSONValue.
    ///
    /// # Example
    ///
    /// ```
    /// use jsonparser::JSONParser;
    ///
    /// let input = r#"
    ///   {
    ///     "name": "John Doe",
    ///     "age": 30
    ///   }
    /// "#;
    ///
    /// let json = match JSONParser::from(input) {
    ///   Ok(value) => value,
    ///   Err(e) => {
    ///     eprintln!("Error: {}", e);
    ///     return;
    ///   }
    /// };
    ///
    /// println!("{:#?}", json["name"].as_str());
    /// ```
    pub fn from(input: &'a str) -> Result<JSONValue, String>{
        let mut parser = JSONParser::new(input);

        parser.parse()
    }
}
