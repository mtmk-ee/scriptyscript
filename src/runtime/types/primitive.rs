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
            (Primitive::Nil, Primitive::Nil) => true,
            (Primitive::Integer(a), Primitive::Integer(b)) => a == b,
            (Primitive::Float(a), Primitive::Float(b)) => a == b,
            (Primitive::String(a), Primitive::String(b)) => a == b,
            (Primitive::Boolean(a), Primitive::Boolean(b)) => a == b,
            _ => false,
        }
    }
}

impl std::ops::Add for Primitive {
    type Output = Option<Primitive>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Integer(a), Primitive::Integer(b)) => Some(Primitive::Integer(a + b)),
            (Primitive::Integer(a), Primitive::Float(b)) => Some(Primitive::Float(a as f64 + b)),
            (Primitive::Float(a), Primitive::Integer(b)) => Some(Primitive::Float(a + b as f64)),
            (Primitive::Float(a), Primitive::Float(b)) => Some(Primitive::Float(a + b)),
            (Primitive::String(a), Primitive::String(b)) => Some(Primitive::String(a + b.as_str())),
            _ => None,
        }
    }
}

impl std::ops::Sub for Primitive {
    type Output = Option<Primitive>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Integer(a), Primitive::Integer(b)) => Some(Primitive::Integer(a - b)),
            (Primitive::Integer(a), Primitive::Float(b)) => Some(Primitive::Float(a as f64 - b)),
            (Primitive::Float(a), Primitive::Integer(b)) => Some(Primitive::Float(a - b as f64)),
            (Primitive::Float(a), Primitive::Float(b)) => Some(Primitive::Float(a - b)),
            _ => None,
        }
    }
}

impl std::ops::Mul for Primitive {
    type Output = Option<Primitive>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Integer(a), Primitive::Integer(b)) => Some(Primitive::Integer(a * b)),
            (Primitive::Integer(a), Primitive::Float(b)) => Some(Primitive::Float(a as f64 * b)),
            (Primitive::Float(a), Primitive::Integer(b)) => Some(Primitive::Float(a * b as f64)),
            (Primitive::Float(a), Primitive::Float(b)) => Some(Primitive::Float(a * b)),
            _ => None,
        }
    }
}

impl std::ops::Div for Primitive {
    type Output = Option<Primitive>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Integer(a), Primitive::Integer(b)) => Some(Primitive::Integer(a / b)),
            (Primitive::Integer(a), Primitive::Float(b)) => Some(Primitive::Float(a as f64 / b)),
            (Primitive::Float(a), Primitive::Integer(b)) => Some(Primitive::Float(a / b as f64)),
            (Primitive::Float(a), Primitive::Float(b)) => Some(Primitive::Float(a / b)),
            _ => None,
        }
    }
}

impl std::ops::Rem for Primitive {
    type Output = Option<Primitive>;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Integer(a), Primitive::Integer(b)) => Some(Primitive::Integer(a % b)),
            (Primitive::Integer(a), Primitive::Float(b)) => Some(Primitive::Float(a as f64 % b)),
            (Primitive::Float(a), Primitive::Integer(b)) => Some(Primitive::Float(a % b as f64)),
            (Primitive::Float(a), Primitive::Float(b)) => Some(Primitive::Float(a % b)),
            _ => None,
        }
    }
}

impl ToString for Primitive {
    fn to_string(&self) -> String {
        match self {
            Primitive::Nil => "nil".to_string(),
            Primitive::Integer(i) => i.to_string(),
            Primitive::Float(f) => f.to_string(),
            Primitive::String(s) => s.to_string(),
            Primitive::Boolean(b) => b.to_string(),
        }
    }
}
