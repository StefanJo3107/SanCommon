use std::time::Duration;
use rand::{random, Rng};
use crate::hid_actuator::HidActuator;
use crate::keycodes::ASCII_2_HID;
use crate::value::{NativeFunctionData, Number, Value};

pub type NativeFn<T: HidActuator> = fn(&mut NativeFns<T>, Vec<Value>) -> Value;

pub struct NativeFns<T: HidActuator> {
    hid_actuator: T,
}

impl<T: HidActuator> NativeFns<T> {
    pub fn new(hid_actuator: T) -> Self {
        NativeFns {
            hid_actuator
        }
    }

    pub fn get_function_from_data(&self, fun_data: &NativeFunctionData) -> NativeFn<T> {
        match fun_data {
            NativeFunctionData { arity: 1, name } if *name == String::from("sleep") => Self::sleep,
            NativeFunctionData { arity: 2, name } if *name == String::from("random_int") => Self::random_int,
            NativeFunctionData { arity: 2, name } if *name == String::from("random_float") => Self::random_int,
            NativeFunctionData { arity: 1, name } if *name == String::from("inject_keys") => Self::inject_keys,
            NativeFunctionData { arity: 3, name } if *name == String::from("inject_sequence") => Self::inject_sequence,
            NativeFunctionData { arity: 0, name } if *name == String::from("release_keys") => Self::release_keys,
            NativeFunctionData { arity: 1, name } if *name == String::from("string_to_keys") => Self::string_to_keys,
            NativeFunctionData { arity: 3, name } if *name == String::from("type_string") => Self::type_string,
            _ => panic!("Unimplemented native function with name {}", fun_data.name)
        }
    }

    fn sleep(&mut self, args: Vec<Value>) -> Value {
        if args.len() >= 1 {
            if let Value::ValNumber(duration) = args.first().unwrap() {
                self.hid_actuator.sleep(*duration as usize);
            }
        }

        Value::ValNil
    }

    fn random_int(&mut self, args: Vec<Value>) -> Value {
        if args.len() >= 2 {
            if let Value::ValNumber(low) = args.first().unwrap() {
                if let Value::ValNumber(high) = args[1] {
                    let mut rng = rand::thread_rng();
                    return Value::ValNumber(rng.gen_range(*low as u64..high as u64) as Number);
                }
            }
        }

        Value::ValNil
    }

    fn random_float(&mut self, args: Vec<Value>) -> Value {
        if args.len() >= 2 {
            if let Value::ValNumber(low) = args.first().unwrap() {
                if let Value::ValNumber(high) = args[1] {
                    let mut rng = rand::thread_rng();
                    return Value::ValNumber(rng.gen_range(*low as f64..high as f64));
                }
            }
        }

        Value::ValNil
    }

    fn inject_keys(&mut self, args: Vec<Value>) -> Value {
        if args.len() >= 1 {
            if let Value::ValKey(keys) = args.first().unwrap() {
                if keys.len() <= 6 {
                    self.hid_actuator.key_down(keys);
                }
            }
        }

        Value::ValNil
    }

    fn inject_sequence(&mut self, args: Vec<Value>) -> Value {
        if args.len() >= 3 {
            if let Value::ValKeySequence(seq) = args.first().unwrap() {
                if let Value::ValNumber(delay) = args[1] {
                    if let Value::ValNumber(jitter) = args[2] {
                        for combo in seq {
                            self.hid_actuator.key_down(combo);
                            let mut rng = rand::thread_rng();
                            self.hid_actuator.sleep(delay as usize + rng.gen_range(0..jitter as usize));
                        }

                        self.hid_actuator.clear_keys();
                    }
                }
            }
        }

        Value::ValNil
    }

    fn release_keys(&mut self, args: Vec<Value>) -> Value {
        self.hid_actuator.clear_keys();
        Value::ValNil
    }

    fn string_to_keys(&mut self, args: Vec<Value>) -> Value {
        if args.len() >= 1 {
            if let Value::ValString(keys) = args.first().unwrap() {
                let mut seq: Vec<Vec<u8>> = vec![];

                for c in keys.chars() {
                    let hid = ASCII_2_HID[c as usize];
                    if hid[1] == 0 {
                        seq.push(vec![hid[0]]);
                    } else {
                        seq.push(hid.to_vec());
                    }
                }

                return Value::ValKeySequence(seq);
            }
        }

        Value::ValNil
    }

    fn type_string(&mut self, args: Vec<Value>) -> Value {
        if args.len() >= 3 {
            if let Value::ValString(keys) = args.first().unwrap() {
                if let Value::ValNumber(delay) = args[1] {
                    if let Value::ValNumber(jitter) = args[2] {
                        let mut seq: Vec<Vec<u8>> = vec![];

                        for c in keys.chars() {
                            let hid = ASCII_2_HID[c as usize];
                            if hid[1] == 0 {
                                seq.push(vec![hid[0]]);
                            } else {
                                seq.push(hid.to_vec());
                            }
                        }

                        for combo in seq {
                            self.hid_actuator.key_down(&combo);
                            let mut rng = rand::thread_rng();
                            self.hid_actuator.sleep(delay as usize + rng.gen_range(0..jitter as usize));
                        }

                        self.hid_actuator.clear_keys();
                    }
                }
            }
        }

        Value::ValNil
    }
}