use std::collections::HashMap;

use crate::JSONValue;

pub struct JSONSchema<'a> {
	rules: HashMap<&'a str, Box<dyn Validator>>
}

impl<'a> JSONSchema<'a> {
    /// Create a new JSONSchema instance with the given rules.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jsonparser::{JSONSchema, StringType, NumberType};
    ///
    /// let schema = JSONSchema::new([
    ///   ("name", StringType::new().min_length(6).boxed()),
    ///   ("age", NumberType::new().gt(18.0).boxed())
    /// ]);
    /// ```
    pub fn new<T: Into<HashMap<&'a str, Box<dyn Validator>>>>(rules: T) -> Self {
        Self {
            rules: rules.into()
        }
    }

    /// Validate the given JSONValue against the schema.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jsonparser::{JSONValue, JSONSchema, StringType, NumberType};
    ///
    /// let schema = JSONSchema::new([
    ///   ("name", StringType::new().min_length(6).boxed()),
    ///   ("age", NumberType::new().gt(18.0).boxed())
    /// ]);
    ///
    /// let json = JSONValue::Object({ /* ... */ });
    ///
    /// match schema.validate(&json) {
    ///   Ok(_) => println!("Valid JSON"),
    ///   Err(e) => eprintln!("Invalid JSON: {}", e)
    /// }
    /// ```
    pub fn validate(&self, value: &JSONValue) -> Result<(), String> {
        match value {
            JSONValue::Object(obj) => {
                for (key, rule) in &self.rules {
                    match obj.get(*key) {
                        Some(value) => rule.validate(value)?,
                        None => return Err(format!("Key '{}' not found", *key))
                    }
                }
                Ok(())
            },
            _ => Err("Expected an object for validation".to_string()),
        }
    }
}

pub trait Validator {
    fn validate(&self, value: &JSONValue) -> Result<(), String>;
}

pub struct StringType {
    min_length: Option<usize>,
    max_length: Option<usize>,
    length: Option<usize>,
    starts_with: Option<String>,
    ends_with: Option<String>,
    includes: Option<String>
}

impl StringType {
    /// Create a new StringType instance.
    pub fn new() -> Self {
        Self {
            min_length: None,
            max_length: None,
            length: None,
            starts_with: None,
            ends_with: None,
            includes: None
        }
    }

    /// Set the minimum length of the string.
    pub fn min_length(mut self, min: usize) -> Self {
        self.min_length = Some(min);
        self
    }

    /// Set the maximum length of the string.
    pub fn max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    /// Set the exact length of the string.
    pub fn length(mut self, length: usize) -> Self {
        self.length = Some(length);
        self
    }

    /// Set the expected starting of the string.
    pub fn starts_with(mut self, value: &str) -> Self {
        self.starts_with = Some(value.to_string());
        self
    }

    /// Set the expected ending of the string.
    pub fn ends_with(mut self, value: &str) -> Self {
        self.ends_with = Some(value.to_string());
        self
    }

    /// Set the string to include a specific substring.
    pub fn includes(mut self, value: &str) -> Self {
        self.includes = Some(value.to_string());
        self
    }

    /// Convert the StringType to a Box<dyn Validator>.
    pub fn boxed(self) -> Box<dyn Validator> {
        Box::new(self)
    }
}

impl Validator for StringType {
    fn validate(&self, value: &JSONValue) -> Result<(), String> {
        match value {
            JSONValue::String(s) => {
                if let Some(min) = self.min_length {
                    if s.len() < min {
                        return Err(format!("String is too short (min: {})", min));
                    }
                }

                if let Some(max) = self.max_length {
                    if s.len() > max {
                        return Err(format!("String is too long (max: {})", max));
                    }
                }

                if let Some(length) = self.length {
                    if s.len() != length {
                        return Err(format!("String is not the correct length (length: {})", length));
                    }
                }

                if let Some(starts_with) = &self.starts_with {
                    if !s.starts_with(starts_with) {
                        return Err(format!("String does not start with '{}'", starts_with));
                    }
                }

                if let Some(ends_with) = &self.ends_with {
                    if !s.ends_with(ends_with) {
                        return Err(format!("String does not end with '{}'", ends_with));
                    }
                }

                if let Some(includes) = &self.includes {
                    if !s.contains(includes) {
                        return Err(format!("String does not include '{}'", includes));
                    }
                }

                Ok(())
            },
            _ => Err("Type mismatch, expected String".to_string()),
        }
    }
}

pub struct NumberType {
    min: Option<f64>,
    max: Option<f64>
}

impl NumberType {
    /// Create a new NumberType instance.
    pub fn new() -> Self {
        Self {
            min: None,
            max: None
        }
    }

    /// Set the minimum value of the number.
    pub fn gt(mut self, value: f64) -> Self {
        self.min = Some(value);
        self
    }

    /// Set the maximum value of the number.
    pub fn lt(mut self, value: f64) -> Self {
        self.max = Some(value);
        self
    }

    /// Convert the NumberType to a Box<dyn Validator>.
    pub fn boxed(self) -> Box<dyn Validator> {
        Box::new(self)
    }
}

impl Validator for NumberType {
    fn validate(&self, value: &JSONValue) -> Result<(), String> {
        match value {
            JSONValue::Number(n) => {
                if let Some(min) = self.min {
                    if n < &min {
                        return Err(format!("Number is too small (min: {})", min));
                    }
                }

                if let Some(max) = self.max {
                    if n > &max {
                        return Err(format!("Number is too large (max: {})", max));
                    }
                }

                Ok(())
            },
            _ => Err("Type mismatch, expected Number".to_string()),
        }
    }
}

pub struct ArrayType {
    min_length: Option<usize>,
    max_length: Option<usize>,
    length: Option<usize>,
    empty: Option<bool>,
    every: Option<Box<dyn Validator>>,
    some: Option<Box<dyn Validator>>,
    at: Option<(usize, Box<dyn Validator>)>
}

impl ArrayType {
    /// Create a new ArrayType instance.
    pub fn new() -> Self {
        Self {
            min_length: None,
            max_length: None,
            length: None,
            empty: None,
            every: None,
            some: None,
            at: None
        }
    }

    /// Set the minimum length of the array.
    pub fn min_length(mut self, min: usize) -> Self {
        self.min_length = Some(min);
        self
    }

    /// Set the maximum length of the array.
    pub fn max_length(mut self, max: usize) -> Self {
        self.max_length = Some(max);
        self
    }

    /// Set the exact length of the array.
    pub fn length(mut self, length: usize) -> Self {
        self.length = Some(length);
        self
    }

    /// Set whether the array can be empty.
    pub fn empty(mut self) -> Self {
        self.empty = Some(true);
        self
    }

    /// Set a rule for every items in the array.
    pub fn every(mut self, rule: Box<dyn Validator>) -> Self {
        self.every = Some(rule);
        self
    }

    /// Set a rule for at least one item in the array.
    pub fn some(mut self, rule: Box<dyn Validator>) -> Self {
        self.some = Some(rule);
        self
    }

    /// Set a rule for a specific item in the array.
    pub fn at(mut self, index: usize, rule: Box<dyn Validator>) -> Self {
        self.at = Some((index, rule));
        self
    }

    /// Convert the ArrayType to a Box<dyn Validator>.
    pub fn boxed(self) -> Box<dyn Validator> {
        Box::new(self)
    }
}

impl Validator for ArrayType {
    fn validate(&self, value: &JSONValue) -> Result<(), String> {
        match value {
            JSONValue::Array(arr) => {
                if let Some(min) = self.min_length {
                    if arr.len() < min {
                        return Err(format!("Array is too short (min: {})", min));
                    }
                }

                if let Some(max) = self.max_length {
                    if arr.len() > max {
                        return Err(format!("Array is too long (max: {})", max));
                    }
                }

                if let Some(length) = self.length {
                    if arr.len() != length {
                        return Err(format!("Array is not the correct length (length: {})", length));
                    }
                }

                if let Some(empty) = self.empty {
                    if empty && arr.is_empty() {
                        return Err("Array is empty".to_string());
                    }
                }

                if let Some(rule) = &self.every {
                    for item in arr {
                        rule.validate(item)?;
                    }
                }

                if let Some(rule) = &self.some {
                    for item in arr {
                        if rule.validate(item).is_ok() {
                            return Ok(());
                        }
                    }
                    return Err("No items in the array match the rule".to_string());
                }

                if let Some((index, rule)) = &self.at {
                    if let Some(item) = arr.get(*index) {
                        rule.validate(item)?;
                    } else {
                        return Err(format!("Index {} not found", index));
                    }
                }

                Ok(())
            },
            _ => Err("Type mismatch, expected Array".to_string()),
        }
    }
}

pub struct BooleanType {
    value: Option<bool>
}

impl BooleanType {
    /// Create a new BooleanType instance.
    pub fn new() -> Self {
        Self {
            value: None
        }
    }

    /// Set the expected value to true.
    pub fn truthy(mut self) -> Self {
        self.value = Some(true);
        self
    }

    /// Set the expected value to false.
    pub fn falsy(mut self) -> Self {
        self.value = Some(false);
        self
    }

    /// Convert the BooleanType to a Box<dyn Validator>.
    pub fn boxed(self) -> Box<dyn Validator> {
        Box::new(self)
    }
}

impl Validator for BooleanType {
    fn validate(&self, value: &JSONValue) -> Result<(), String> {
        match value {
            JSONValue::Boolean(b) => {
                if let Some(expected) = self.value {
                    if b != &expected {
                        return Err(format!("Expected {}", expected));
                    }
                }

                Ok(())
            },
            _ => Err("Type mismatch, expected Boolean".to_string()),
        }
    }
}

pub struct ObjectType {
    rules: HashMap<String, Box<dyn Validator>>
}

impl ObjectType {
    /// Create a new ObjectType instance.
    pub fn new() -> Self {
        Self {
            rules: HashMap::new()
        }
    }

    /// Add a rule for a property in the object.
    pub fn property(mut self, key: &str, rule: Box<dyn Validator>) -> Self {
        self.rules.insert(key.to_string(), rule);
        self
    }

    /// Convert the ObjectType to a Box<dyn Validator>.
    pub fn boxed(self) -> Box<dyn Validator> {
        Box::new(self)
    }
}

impl Validator for ObjectType {
    fn validate(&self, value: &JSONValue) -> Result<(), String> {
        match value {
            JSONValue::Object(obj) => {
                for (key, rule) in &self.rules {
                    match obj.get(key) {
                        Some(value) => rule.validate(value)?,
                        None => return Err(format!("Key '{}' not found", key))
                    }
                }
                Ok(())
            },
            _ => Err("Type mismatch, expected Object".to_string()),
        }
    }
}

pub struct NullType;

impl NullType {
    /// Create a new NullType instance.
    pub fn new() -> Self {
        Self
    }

    /// Convert the NullType to a Box<dyn Validator>.
    pub fn boxed(self) -> Box<dyn Validator> {
        Box::new(self)
    }
}

impl Validator for NullType {
    fn validate(&self, value: &JSONValue) -> Result<(), String> {
        match value {
            JSONValue::Null => Ok(()),
            _ => Err("Type mismatch, expected Null".to_string()),
        }
    }
}
