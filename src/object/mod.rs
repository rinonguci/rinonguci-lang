use core::fmt::Debug;
use enum_as_inner::EnumAsInner;
use environment::Environment;
use std::{cell::RefCell, fmt::Write, rc::Rc};

use crate::ast::{expression::ExpressionType, statement::StatementType, TNode};

pub mod environment;

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
    Function(Function),
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
            _ => write!(f, "Function"),
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
            Object::Function(f) => f.inspect(),
            Object::Error(e) => e.inspect(),
        }
    }

    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Integer(i) => i.object_type(),
            Object::Boolean(b) => b.object_type(),
            Object::Null(n) => n.object_type(),
            Object::Return(r) => r.object_type(),
            Object::Function(f) => f.object_type(),
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

// type Function struct {
// Parameters []*ast.Identifier
// Body *ast.BlockStatement
// Env *Environment
// }
// func (f *Function) Type() ObjectType { return FUNCTION_OBJ }
// func (f *Function) Inspect() string {
// var out bytes.Buffer
// params := []string{}
// for _, p := range f.Parameters {
// params = append(params, p.String())
// }
// out.WriteString("fn")
// out.WriteString("(")
// out.WriteString(strings.Join(params, ", "))
// out.WriteString(") {\n")
// out.WriteString(f.Body.String())
// out.WriteString("\n}")
// return out.String()
// }

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub parameters: Vec<Box<ExpressionType>>,
    pub body: Box<StatementType>,
    pub env: Rc<RefCell<Environment>>,
}

impl TObject for Function {
    fn inspect(&self) -> String {
        let mut out = String::new();
        let params: Vec<String> = self.parameters.iter().map(|p| p.string()).collect();
        out.push_str("fn");
        out.push_str("(");
        out.push_str(&params.join(", "));
        out.push_str(") {\n");
        out.push_str(&self.body.string());
        out.push_str("\n}");
        out
    }

    fn object_type(&self) -> ObjectType {
        ObjectType::NULL
    }
}

impl Write for Function {
    fn write_str(&mut self, _s: &str) -> std::fmt::Result {
        todo!()
    }
}
