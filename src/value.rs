use serde::{Deserialize, Serialize};
use crate::chunk::Chunk;

pub type Number = f64;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum Value {
    ValBool(bool),
    ValNumber(Number),
    ValNil,
    ValString(String),
    ValKey(u8),
    ValFunction(FunctionData),
    ValNativeFn(NativeFunctionData)
}

#[derive(Copy, Clone, PartialEq)]
pub enum FunctionType {
    Function,
    Script
}
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct FunctionData {
    pub arity: usize,
    pub chunk: Chunk,
    pub name: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct NativeFunctionData{
    pub arity: usize,
    pub name: String
}

impl FunctionData {
    pub fn new() -> FunctionData {
        FunctionData {
            arity: 0,
            chunk: Chunk::new(),
            name: String::from(""),
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray { values: vec![] }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn get(&self, index: usize) -> &Value {
        &self.values[index]
    }

    pub fn write_constant(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn print_value(value: &Value) {
        match value {
            Value::ValBool(boolean) => print!("\x1B[3m{}\x1B[0m", boolean),
            Value::ValNumber(number) => print!("\x1B[3m{}\x1B[0m", number),
            Value::ValNil => print!("\x1B[3m{}\x1B[0m", "nil"),
            Value::ValString(string) => print!("\x1B[3m{}\x1B[0m", string),
            Value::ValFunction(data) => print!("\x1B[3m{}\x1B[0m", if data.name!="" {&data.name} else{"<script>"}),
            Value::ValKey(key ) => print!("\x1B[3m{}\x1B[0m", key),
            Value::ValNativeFn(_) => print!("\x1B[3m{}\x1B[0m", "<native fn>")
        }
    }
}
