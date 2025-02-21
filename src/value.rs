use crate::vm::ExeState;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Function(fn (&mut ExeState)->i32),
    Nil,
    
}

impl Eq for Value { }

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(n) => write!(f, "{:?}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Function(_) => write!(f, "<function>"),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Nil => (),
            Value::Bool(b) => b.hash(state),
            Value::Integer(i) => i.hash(state),
            Value::String(s) => s.hash(state),
            Value::Float(f) => unsafe {
                std::mem::transmute::<f64, i64>(*f).hash(state);
            },
            Value::Function(f) => (*f as *const usize).hash(state),
        }
    }
}

