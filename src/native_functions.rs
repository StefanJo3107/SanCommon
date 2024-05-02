use std::time::Duration;
use crate::value::{NativeFunctionData, Value};

pub type NativeFn = fn(Vec<Value>) -> Value;

pub fn get_function_from_data(fun_data: &NativeFunctionData) -> NativeFn {
    match fun_data {
        NativeFunctionData { arity: 1, name } if *name == String::from("sleep") => sleep,
        _ => panic!("Unimplemented native function with name {}", fun_data.name)
    }
}

fn sleep(args: Vec<Value>) -> Value {
    if args.len() >= 1 {
        if let Value::ValNumber(duration) = args.first().unwrap() {
            std::thread::sleep(Duration::from_secs(*duration as u64));

        }
    }

    Value::ValNil
}