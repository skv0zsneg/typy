use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    // TODO: Float(f64), Bool(bool), Str(Rc<str>), etc.
}

impl Value {
    pub fn as_int(&self) -> Result<i64, String> {
        match self {
            Value::Int(n) => Ok(*n),
            _ => Err(format!(
                "TypeError: expected int, got {:?}",
                self.type_name()
            )),
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
        }
    }
}

impl Value {
    pub fn add(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            _ => Err(format!(
                "TypeError: unsupported operand types for +: '{}' and '{}'",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    pub fn sub(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            _ => Err(format!(
                "TypeError: unsupported operand types for -: '{}' and '{}'",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    pub fn mul(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            _ => Err(format!(
                "TypeError: unsupported operand types for *: '{}' and '{}'",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    pub fn div(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err("ZeroDivisionError: division by zero".to_string())
                } else {
                    Ok(Value::Int(a / b))
                }
            }
            _ => Err(format!(
                "TypeError: unsupported operand types for /: '{}' and '{}'",
                self.type_name(),
                other.type_name()
            )),
        }
    }
}
