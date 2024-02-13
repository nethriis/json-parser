use core::fmt;
use std::collections::HashMap;
use std::ops::Index;

use crate::utils::lexer::{Lexer, Token, TokenKind};

#[derive(Clone)]
pub struct OrderedMap<V> {
    order: Vec<String>,
    map: HashMap<String, V>,
}

impl<V: fmt::Debug> fmt::Debug for OrderedMap<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.order.iter().map(|key| (key, self.map.get(key).unwrap()))).finish()
    }
}

impl fmt::Display for OrderedMap<JSONValue> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, key) in self.order.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }

            match self.map.get(key).expect("Key not found") {
                JSONValue::String(_) => write!(f, "\"{}\": \"{}\"", key, self.map[key])?,
                JSONValue::Null => write!(f, "\"{}\": null", key)?,
                _ => write!(f, "\"{}\": {}", key, self.map[key])?
            }
        }
        write!(f, "}}")
    }
}

pub trait JSONAccess {
    fn get<'a>(&self, value: &'a JSONValue) -> Option<&'a JSONValue>;
}

impl JSONAccess for &str {
    fn get<'a>(&self, value: &'a JSONValue) -> Option<&'a JSONValue> {
        match value {
            JSONValue::Object(obj) => obj.get(self),
            _ => None,
        }
    }
}

impl JSONAccess for usize {
    fn get<'a>(&self, value: &'a JSONValue) -> Option<&'a JSONValue> {
        match value {
            JSONValue::Array(array) => array.get(*self),
            _ => None,
        }
    }
}

impl<V> OrderedMap<V> {
    pub fn new() -> Self {
        Self {
            order: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: V) {
        let key = key.to_string();

        if !self.map.contains_key(&key) {
            self.order.push(key.clone());
        }
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        self.map.get(key)
    }
}

#[derive(Clone)]
pub enum JSONValue {
    Object(OrderedMap<JSONValue>),
    Array(Vec<JSONValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null
}

impl fmt::Debug for JSONValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JSONValue::Object(obj) => write!(f, "{:?}", obj),
            JSONValue::Array(array) => write!(f, "{:?}", array),
            JSONValue::String(value) => write!(f, "{:?}", value),
            JSONValue::Number(value) => write!(f, "{:?}", value),
            JSONValue::Boolean(value) => write!(f, "{:?}", value),
            JSONValue::Null => write!(f, "null"),
        }
    }
}

impl fmt::Display for JSONValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JSONValue::Object(obj) => write!(f, "{:?}", obj),
            JSONValue::Array(array) => write!(f, "{:?}", array),
            JSONValue::String(value) => write!(f, "{}", value),
            JSONValue::Number(value) => write!(f, "{}", value),
            JSONValue::Boolean(value) => write!(f, "{}", value),
            JSONValue::Null => write!(f, "null"),
        }
    }
}

impl JSONValue {
    pub fn get<A: JSONAccess>(&self, accessor: A) -> Option<&JSONValue> {
        accessor.get(self)
    }

    /// Returns the value as a string if it is a string.
    /// Returns None otherwise.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let value = JSONValue::String("Hello, world!".to_string());
    ///
    /// assert_eq!(value.as_str(), Some("Hello, world!"));
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JSONValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the value as a number if it is a number.
    /// Returns None otherwise.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let value = JSONValue::Number(42.0);
    ///
    /// assert_eq!(value.as_f64(), Some(42.0));
    /// ```
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JSONValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Returns the value as a boolean if it is a boolean.
    /// Returns None otherwise.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let value = JSONValue::Boolean(true);
    ///
    /// assert_eq!(value.as_bool(), Some(true));
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JSONValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the value as an array if it is an array.
    /// Returns None otherwise.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let value = JSONValue::Array(vec![JSONValue::Number(1.0), JSONValue::Number(2.0)]);
    ///
    /// assert_eq!(value.as_array(), Some(&vec![JSONValue::Number(1.0), JSONValue::Number(2.0)]));
    /// ```
    pub fn as_array(&self) -> Option<&Vec<JSONValue>> {
        match self {
            JSONValue::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Returns the value as an object if it is an object.
    /// Returns None otherwise.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let mut object = OrderedMap::new();
    ///
    /// object.insert("name", JSONValue::String("John Doe".to_string()));
    /// object.insert("age", JSONValue::Number(30.0));
    /// let value = JSONValue::Object(object);
    ///
    /// assert_eq!(value.as_object(), Some(&object));
    /// ```
    pub fn as_object(&self) -> Option<&OrderedMap<JSONValue>> {
        match self {
            JSONValue::Object(o) => Some(o),
            _ => None,
        }
    }

    /// Returns the value as an object if it is an object.
    /// Returns None otherwise.
    /// This method allows modifying the object.
    ///
    /// # Example
    /// ```no_run
    /// let mut object = OrderedMap::new();
    ///
    /// object.insert("name", JSONValue::String("John Doe".to_string()));
    /// object.insert("age", JSONValue::Number(30.0));
    /// let mut value = JSONValue::Object(object);
    ///
    /// value.as_object_mut().unwrap().insert("isStudent", JSONValue::Boolean(false));
    /// ```
    pub fn as_object_mut(&mut self) -> Option<&mut OrderedMap<JSONValue>> {
        match self {
            JSONValue::Object(o) => Some(o),
            _ => None,
        }
    }

    /// Returns true if the value is a null value.
    /// Returns false otherwise.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let value = JSONValue::Null;
    ///
    /// assert_eq!(value.is_null(), true);
    /// ```
    pub fn is_null(&self) -> bool {
        match self {
            JSONValue::Null => true,
            _ => false,
        }
    }
}

impl<'a> Index<usize> for JSONValue {
    type Output = JSONValue;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            JSONValue::Array(array) => &array[index],
            _ => panic!("Numerical indexing is supported only for JSON arrays"),
        }
    }
}

impl<'a> Index<&'a str> for JSONValue {
    type Output = JSONValue;

    fn index(&self, index: &'a str) -> &Self::Output {
        match self {
            JSONValue::Object(obj) => obj.get(index).expect("Key not found"),
            _ => panic!("Key-based indexing is supported only for JSON objects"),
        }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer, current_token: None }
    }

    pub fn parse(&mut self) -> Result<JSONValue, String> {
        self.next_token();
        self.parse_object()
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn parse_value(&mut self) -> Result<JSONValue, String> {
        match self.current_token {
            Some(ref token) => match token.kind {
                TokenKind::OpenBrace => self.parse_object(),
                TokenKind::OpenBracket => self.parse_array(),
                TokenKind::QuotedString => {
                    let value = token.text.clone().unwrap();

                    self.next_token();
                    Ok(JSONValue::String(value))
                },
                TokenKind::Number => {
                    let value = token.text.clone().unwrap().parse::<f64>().map_err(|e| e.to_string())?;

                    self.next_token();
                    Ok(JSONValue::Number(value))
                },
                TokenKind::Keyword => {
                    let value = token.text.clone().unwrap();

                    self.next_token();
                    match value.as_str() {
                        "true" => Ok(JSONValue::Boolean(true)),
                        "false" => Ok(JSONValue::Boolean(false)),
                        "null" => Ok(JSONValue::Null),
                        _ => Err(format!("Unknown keyword: {}", value))
                    }
                },
                _ => Err(format!("Unexpected token: {:?}", token))
            },
            _ => Err("Unexpected end of input".to_string())
        }
    }

    fn parse_object(&mut self) -> Result<JSONValue, String> {
        let mut object = OrderedMap::new();

        self.next_token();
        while let Some(ref token) = self.current_token {
            if token.kind == TokenKind::CloseBrace {
                self.next_token();
                return Ok(JSONValue::Object(object));
            }
            if token.kind != TokenKind::QuotedString {
                return Err(format!("Unexpected token: {:?}", token));
            }
            let key = token.text.clone().unwrap();

            self.next_token();
            match self.current_token {
                Some(ref token) if token.kind == TokenKind::Colon => {
                    self.next_token();
                },
                _ => return Err("Expected ':' after object key".to_string()),
            }
            let value = self.parse_value()?;

            object.insert(key.as_str(), value);
            match self.current_token {
                Some(ref token) if token.kind == TokenKind::Comma => {
                    self.next_token();
                },
                Some(ref token) if token.kind == TokenKind::CloseBrace => continue,
                _ => return Err("Expected ',' or '}' after object value".to_string()),
            }
        }
        Err("Unexpected end of input".to_string())
    }

    fn parse_array(&mut self) -> Result<JSONValue, String> {
        let mut array = Vec::new();

        self.next_token();
        while let Some(ref token) = self.current_token {
            if token.kind == TokenKind::CloseBracket {
                self.next_token();
                return Ok(JSONValue::Array(array));
            }
            let value = self.parse_value()?;

            array.push(value);
            match self.current_token {
                Some(ref token) if token.kind == TokenKind::Comma => {
                    self.next_token();
                },
                Some(ref token) if token.kind == TokenKind::CloseBracket => continue,
                _ => return Err("Expected ',' or ']' in array".to_string())
            }
        }
        Err("Unexpected end of input".to_string())
    }
}