use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Int(i64),
    Bool(bool),
    None,
}

impl Object {
    pub fn type_name(&self) -> &'static str {
        match self {
            Object::Int(_) => "int",
            Object::Bool(_) => "bool",
            Object::None => "NoneType",
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Int(n) => write!(f, "{}", n),
            Object::Bool(b) => write!(f, "{}", if *b { "True" } else { "False" }),
            Object::None => write!(f, "None"),
        }
    }
}

impl Object {
    pub fn add(&self, other: &Object) -> Result<Object, String> {
        match (self, other) {
            (Object::Int(a), Object::Int(b)) => Ok(Object::Int(a + b)),
            _ => Err(get_bin_op_type_error_msg(
                "+",
                self.type_name(),
                other.type_name(),
            )),
        }
    }

    pub fn sub(&self, other: &Object) -> Result<Object, String> {
        match (self, other) {
            (Object::Int(a), Object::Int(b)) => Ok(Object::Int(a - b)),
            _ => Err(get_bin_op_type_error_msg(
                "-",
                self.type_name(),
                other.type_name(),
            )),
        }
    }

    pub fn mul(&self, other: &Object) -> Result<Object, String> {
        match (self, other) {
            (Object::Int(a), Object::Int(b)) => Ok(Object::Int(a * b)),
            _ => Err(get_bin_op_type_error_msg(
                "*",
                self.type_name(),
                other.type_name(),
            )),
        }
    }

    pub fn div(&self, other: &Object) -> Result<Object, String> {
        match (self, other) {
            (Object::Int(a), Object::Int(b)) => {
                if *b == 0 {
                    Err("ZeroDivisionError: division by zero".to_string())
                } else {
                    Ok(Object::Int(a / b))
                }
            }
            _ => Err(get_bin_op_type_error_msg(
                "/",
                self.type_name(),
                other.type_name(),
            )),
        }
    }

    pub fn eq(&self, other: &Object) -> Result<bool, String> {
        match (self, other) {
            (Object::Int(a), Object::Int(b)) => Ok(a == b),
            (Object::Bool(a), Object::Bool(b)) => Ok(a == b),
            _ => Err(get_bin_op_type_error_msg(
                "==",
                self.type_name(),
                other.type_name(),
            )),
        }
    }

    pub fn lt(&self, other: &Object) -> Result<bool, String> {
        match (self, other) {
            (Object::Int(a), Object::Int(b)) => Ok(a < b),
            _ => Err(get_bin_op_type_error_msg(
                "<",
                self.type_name(),
                other.type_name(),
            )),
        }
    }

    pub fn gt(&self, other: &Object) -> Result<bool, String> {
        match (self, other) {
            (Object::Int(a), Object::Int(b)) => Ok(a > b),
            _ => Err(get_bin_op_type_error_msg(
                ">",
                self.type_name(),
                other.type_name(),
            )),
        }
    }

    pub fn le(&self, other: &Object) -> Result<bool, String> {
        match (self, other) {
            (Object::Int(a), Object::Int(b)) => Ok(a <= b),
            _ => Err(get_bin_op_type_error_msg(
                "<=",
                self.type_name(),
                other.type_name(),
            )),
        }
    }

    pub fn ge(&self, other: &Object) -> Result<bool, String> {
        match (self, other) {
            (Object::Int(a), Object::Int(b)) => Ok(a >= b),
            _ => Err(get_bin_op_type_error_msg(
                ">=",
                self.type_name(),
                other.type_name(),
            )),
        }
    }

    pub fn ne(&self, other: &Object) -> Result<bool, String> {
        match (self, other) {
            (Object::Int(a), Object::Int(b)) => Ok(a != b),
            (Object::Bool(a), Object::Bool(b)) => Ok(a != b),
            _ => Err(get_bin_op_type_error_msg(
                "!=",
                self.type_name(),
                other.type_name(),
            )),
        }
    }
}

fn get_bin_op_type_error_msg(
    first_type_name: &str,
    second_type_name: &str,
    operator: &str,
) -> String {
    format!(
        "TypeError: '{}' not supported between instances of '{}' and '{}'",
        operator, first_type_name, second_type_name
    )
}
