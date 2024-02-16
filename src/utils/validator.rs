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
    max_length: Option<usize>
}

impl StringType {
    /// Create a new StringType instance.
    pub fn new() -> Self {
        Self {
            min_length: None,
            max_length: None
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
    empty: Option<bool>,
    all: Option<Box<dyn Validator>>
}

impl ArrayType {
    /// Create a new ArrayType instance.
    pub fn new() -> Self {
        Self {
            min_length: None,
            max_length: None,
            empty: None,
            all: None
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

    /// Set whether the array can be empty.
    pub fn empty(mut self) -> Self {
        self.empty = Some(true);
        self
    }

    /// Set a rule for all items in the array.
    pub fn all(mut self, rule: Box<dyn Validator>) -> Self {
        self.all = Some(rule);
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

                if let Some(empty) = self.empty {
                    if empty && arr.is_empty() {
                        return Err("Array is empty".to_string());
                    }
                }

                if let Some(rule) = &self.all {
                    for item in arr {
                        rule.validate(item)?;
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
