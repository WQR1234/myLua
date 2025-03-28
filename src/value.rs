use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::vm::ExeState;

pub struct Table {
    pub array: Vec<Value>,
    pub map: HashMap<Value, Value>,
}

#[derive(Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Function(fn (&mut ExeState)->i32),
    Nil,

    Table(Rc<RefCell<Table>>),
    
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(float) => write!(f, "{}", float),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Function(_) => write!(f, "<function>"),
            Value::Nil => write!(f, "nil"),
            Value::Table(t) => {
                let t = t.borrow();
                write!(f, "<table>: {} {}", t.array.len(), t.map.len())
            },
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Integer(i1), Value::Integer(i2)) => *i1 == *i2,
            (Value::Float(f1), Value::Float(f2)) => *f1 == *f2,
            (Value::String(s1), Value::String(s2)) => *s1 == *s2,
            (Value::Bool(b1), Value::Bool(b2)) => *b1 == *b2,
            (Value::Function(f1), Value::Function(f2)) => std::ptr::eq(f1, f2),
            (Value::Nil, Value::Nil) => true,
            (Value::Table(t1), Value::Table(t2)) => Rc::ptr_eq(t1, t2),
            _ => false,
        }
    }
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
            Value::Table(t) => write!(f, "<table> : {:?}", Rc::as_ptr(t)),
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
            Value::Table(t) => Rc::as_ptr(t).hash(state),
        }
    }
}

