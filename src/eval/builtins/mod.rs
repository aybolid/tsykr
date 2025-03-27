use super::{BuiltinFn, Environment, GlobalEnvironment, Value};

mod len;
mod print;

pub fn register_builtins(env: &mut GlobalEnvironment) {
    let to_register: Vec<(&str, BuiltinFn)> = vec![
        ("print", print::print),
        ("println", print::println),
        ("len", len::length),
    ];

    for (name, func) in to_register {
        env.set(name.to_string(), Value::new_builtin(func));
    }
}
