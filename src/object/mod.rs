use core::fmt::Debug;
use enum_as_inner::EnumAsInner;

trait TObject {
    fn inspect(&self) -> String;
    fn object_type(&self) -> ObjectType;
}

#[derive(PartialEq, Debug)]
pub enum ObjectType {
    INTEGER,
    BOOLEAN,
    NULL,
}

#[derive(EnumAsInner, PartialEq, Clone)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Null(Null),
    Return(ReturnValue),
    Error(Error),
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{:?}", i),
            Object::Boolean(b) => write!(f, "{:?}", b),
            Object::Null(n) => write!(f, "{:?}", n),
            Object::Return(r) => write!(f, "{:?}", r),
            Object::Error(e) => write!(f, "{:?}", e),
        }
    }
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(i) => i.inspect(),
            Object::Boolean(b) => b.inspect(),
            Object::Null(n) => n.inspect(),
            Object::Return(r) => r.inspect(),
            Object::Error(e) => e.inspect(),
        }
    }

    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Integer(i) => i.object_type(),
            Object::Boolean(b) => b.object_type(),
            Object::Null(n) => n.object_type(),
            Object::Return(r) => r.object_type(),
            Object::Error(e) => e.object_type(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Integer {
    pub value: i64,
}

impl TObject for Integer {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }

    fn object_type(&self) -> ObjectType {
        ObjectType::INTEGER
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl TObject for Boolean {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }

    fn object_type(&self) -> ObjectType {
        ObjectType::BOOLEAN
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Null;

impl TObject for Null {
    fn inspect(&self) -> String {
        "null".into()
    }

    fn object_type(&self) -> ObjectType {
        ObjectType::NULL
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnValue {
    pub value: Box<Object>,
}

impl TObject for ReturnValue {
    fn inspect(&self) -> String {
        match self {
            ReturnValue { value } => value.inspect(),
        }
    }

    fn object_type(&self) -> ObjectType {
        ObjectType::NULL
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Error {
    pub message: String,
}

impl TObject for Error {
    fn inspect(&self) -> String {
        format!("ERROR: {}", self.message)
    }

    fn object_type(&self) -> ObjectType {
        ObjectType::NULL
    }
}
