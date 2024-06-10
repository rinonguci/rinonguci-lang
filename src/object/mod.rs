use enum_as_inner::EnumAsInner;

trait TObject {
    fn inspect(&self) -> String;
    fn object_type(&self) -> ObjectType;
}

#[derive(PartialEq)]
pub enum ObjectType {
    INTEGER,
    BOOLEAN,
    NULL,
}

#[derive(Debug, EnumAsInner, PartialEq)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Null(Null),
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(i) => i.inspect(),
            Object::Boolean(b) => b.inspect(),
            Object::Null(n) => n.inspect(),
        }
    }

    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Integer(i) => i.object_type(),
            Object::Boolean(b) => b.object_type(),
            Object::Null(n) => n.object_type(),
        }
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Null;

impl TObject for Null {
    fn inspect(&self) -> String {
        "null".into()
    }

    fn object_type(&self) -> ObjectType {
        ObjectType::NULL
    }
}
