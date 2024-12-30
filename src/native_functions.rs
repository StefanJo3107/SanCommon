use rand::Rng;
use crate::hid_actuator::HidActuator;
use crate::keycodes::ASCII_2_HID;
use crate::value::{NativeFunctionData, Number, Value};

pub type NativeFn<T> = fn(&mut NativeFns<T>, Vec<Value>) -> Value;

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
            NativeFunctionData { arity: 2, name } if *name == String::from("random_float") => Self::random_float,
            NativeFunctionData { arity: 1, name } if *name == String::from("inject_keys") => Self::inject_keys,
            NativeFunctionData { arity: 1, name } if *name == String::from("hold_keys") => Self::hold_keys,
            NativeFunctionData { arity: 3, name } if *name == String::from("inject_sequence") => Self::inject_sequence,
            NativeFunctionData { arity: 0, name } if *name == String::from("release_keys") => Self::release_keys,
            NativeFunctionData { arity: 1, name } if *name == String::from("string_to_keys") => Self::string_to_keys,
            NativeFunctionData { arity: 3, name } if *name == String::from("type_string") => Self::type_string,
            NativeFunctionData { arity: 2, name } if *name == String::from("mouse_move") => Self::mouse_move,
            NativeFunctionData { arity: 1, name } if *name == String::from("mouse_click") => Self::mouse_click,
            NativeFunctionData { arity: 1, name } if *name == String::from("mouse_hold") => Self::mouse_hold,
            NativeFunctionData { arity: 0, name } if *name == String::from("mouse_up") => Self::mouse_up,

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
                    self.hid_actuator.sleep(60);
                    self.hid_actuator.clear_keys();
                    self.hid_actuator.sleep(10);
                }
            }
        }

        Value::ValNil
    }

    fn hold_keys(&mut self, args: Vec<Value>) -> Value {
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
            if let Value::ValNumber(jitter) = args[0] {
                if let Value::ValNumber(delay) = args[1] {
                    if let Value::ValKeySequence(seq) = &args[2] {
                        for combo in seq {
                            self.hid_actuator.key_down(combo);
                            let mut rng = rand::thread_rng();
                            if jitter > 0f64 {
                                self.hid_actuator.sleep(delay as usize + rng.gen_range(0..jitter as usize));
                            } else {
                                self.hid_actuator.sleep(delay as usize);
                            }
                            self.hid_actuator.clear_keys();
                            self.hid_actuator.sleep(10);
                        }

                        self.hid_actuator.clear_keys();
                        self.hid_actuator.sleep(10);
                    }
                }
            }
        }

        Value::ValNil
    }

    fn release_keys(&mut self, _: Vec<Value>) -> Value {
        self.hid_actuator.clear_keys();
        self.hid_actuator.sleep(10);
        Value::ValNil
    }

    fn mouse_move(&mut self, args: Vec<Value>) -> Value {
        if args.len() >= 2 {
            if let Value::ValNumber(y) = args[0] {
                if let Value::ValNumber(x) = args[1] {
                    self.hid_actuator.move_cursor(x as i8, y as i8);
                    self.hid_actuator.sleep(60);
                    self.hid_actuator.move_cursor(0, 0);
                    self.hid_actuator.sleep(60);
                }
            }
        }

        Value::ValNil
    }

    fn mouse_click(&mut self, args: Vec<Value>) -> Value {
        if args.len() >= 1 {
            if let Value::ValMouseButton(code) = args[0] {
                self.hid_actuator.mouse_down(code);
                self.hid_actuator.sleep(10);
                self.hid_actuator.mouse_up();
                self.hid_actuator.sleep(10);
            }
        }

        Value::ValNil
    }

    fn mouse_hold(&mut self, args: Vec<Value>) -> Value {
        if args.len() >= 1 {
            if let Value::ValMouseButton(code) = args[0] {
                self.hid_actuator.mouse_down(code);
            }
        }

        Value::ValNil
    }

    fn mouse_up(&mut self, _: Vec<Value>) -> Value {
        self.hid_actuator.mouse_up();
        self.hid_actuator.sleep(10);
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
            if let Value::ValNumber(jitter) = args[0] {
                if let Value::ValNumber(delay) = args[1] {
                    if let Value::ValString(keys) = &args[2] {
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
                            if jitter > 0f64 {
                                self.hid_actuator.sleep(delay as usize + rng.gen_range(0..jitter as usize));
                            } else {
                                self.hid_actuator.sleep(delay as usize);
                            }
                            self.hid_actuator.clear_keys();
                            self.hid_actuator.sleep(10);
                        }

                        self.hid_actuator.clear_keys();
                    }
                }
            }
        }

        Value::ValNil
    }
}