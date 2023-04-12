//! Module containing the [`Primitive`] type.
//!
//! This type is used to allow for specialized support for certain types.
//! See the [`Primitive`] documentation for more information.

/// Represents a single primitive value.
///
/// A primitive value is a simple type which has specialized support
/// by the interpreter.
///
/// The `Primitive` type implements traits for certain operators.
#[derive(Debug, Clone)]
pub enum Primitive {
    /// Represents the absence of a value.
    Nil,
    /// An integer value.
    ///
    /// Currently this is defined as `i64`, but this may change in the future.
    Integer(i64),
    /// A floating point value.
    ///
    /// Currently this is defined as `f64`, but this may change in the future.
    Float(f64),
    /// A string value.
    String(String),
    /// A boolean value.
    Boolean(bool),
}

impl Eq for Primitive {}
impl PartialEq for Primitive {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::Integer(a), Self::Integer(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Boolean(a), Self::Boolean(b)) => a == b,
            _ => false,
        }
    }
}

impl std::ops::Add for Primitive {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => Some(Self::Integer(a + b)),
            (Self::Integer(a), Self::Float(b)) => Some(Self::Float(a as f64 + b)),
            (Self::Float(a), Self::Integer(b)) => Some(Self::Float(a + b as f64)),
            (Self::Float(a), Self::Float(b)) => Some(Self::Float(a + b)),
            (Self::String(a), Self::String(b)) => Some(Self::String(a + b.as_str())),
            _ => None,
        }
    }
}

impl std::ops::Sub for Primitive {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => Some(Self::Integer(a - b)),
            (Self::Integer(a), Self::Float(b)) => Some(Self::Float(a as f64 - b)),
            (Self::Float(a), Self::Integer(b)) => Some(Self::Float(a - b as f64)),
            (Self::Float(a), Self::Float(b)) => Some(Self::Float(a - b)),
            _ => None,
        }
    }
}

impl std::ops::Mul for Primitive {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => Some(Self::Integer(a * b)),
            (Self::Integer(a), Self::Float(b)) => Some(Self::Float(a as f64 * b)),
            (Self::Float(a), Self::Integer(b)) => Some(Self::Float(a * b as f64)),
            (Self::Float(a), Self::Float(b)) => Some(Self::Float(a * b)),
            _ => None,
        }
    }
}

impl std::ops::Div for Primitive {
    type Output = Option<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => Some(Self::Integer(a / b)),
            (Self::Integer(a), Self::Float(b)) => Some(Self::Float(a as f64 / b)),
            (Self::Float(a), Self::Integer(b)) => Some(Self::Float(a / b as f64)),
            (Self::Float(a), Self::Float(b)) => Some(Self::Float(a / b)),
            _ => None,
        }
    }
}

impl std::ops::Rem for Primitive {
    type Output = Option<Self>;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => Some(Self::Integer(a % b)),
            (Self::Integer(a), Self::Float(b)) => Some(Self::Float(a as f64 % b)),
            (Self::Float(a), Self::Integer(b)) => Some(Self::Float(a % b as f64)),
            (Self::Float(a), Self::Float(b)) => Some(Self::Float(a % b)),
            _ => None,
        }
    }
}

impl ToString for Primitive {
    fn to_string(&self) -> String {
        match self {
            Self::Nil => "nil".to_string(),
            Self::Integer(i) => i.to_string(),
            Self::Float(f) => f.to_string(),
            Self::String(s) => s.to_string(),
            Self::Boolean(b) => b.to_string(),
        }
    }
}
