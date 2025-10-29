//! - Standard library for the Frut language

use frut_lib::{semantic::SemanticAnalyzer, types::Type, value::{RuntimeEnvironment, Value}};
use frut_lib::types::Type as Ty;

/// Predeclare std built-ins into the semantic analyzer
pub fn predeclare_std_builtins(analyzer: &mut SemanticAnalyzer) -> Result<(), frut_lib::ErrorReport> {
    analyzer.predeclare_function("print".to_string(), vec![Type::String], Type::Void)?;
    analyzer.predeclare_function("println".to_string(), vec![Type::String], Type::Void)?;
    analyzer.predeclare_function("input".to_string(), vec![Type::String], Type::String)?;
    Ok(())
}

/// Register native implementations for std built-ins into the runtime
pub fn register_std_builtins(env: &mut RuntimeEnvironment) {
    env.define_function(
        "print".to_string(),
        Value::native_function("print", 1, |args| {
            if let Value::String(s) = &args[0] { print!("{}", s); }
            Ok(Value::Void)
        }),
    );

    env.define_function(
        "println".to_string(),
        Value::native_function("println", 1, |args| {
            if let Value::String(s) = &args[0] { println!("{}", s); }
            Ok(Value::Void)
        }),
    );

    env.define_function(
        "input".to_string(),
        Value::native_function("input", 1, |args| {
            use std::io::{self, Write};
            print!("{}", args[0]);
            let _ = io::stdout().flush();
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => Ok(Value::from_string(input.trim_end().to_string())),
                Err(e) => Err(format!("input error: {}", e)),
            }
        }),
    );
}

/// Register native implementations for imported modules.
pub fn register_native_modules(env: &mut RuntimeEnvironment, modules: &[(String, Option<Vec<String>>)]) {
    for (m, names_opt) in modules {
        match m.as_str() {
            "std/io" => {
                let want_all = names_opt.is_none();
                let names: Vec<String> = names_opt.clone().unwrap_or_else(|| vec!["print".into(), "println".into(), "input".into()]);
                let mut add = |name: &str| {
                    match name {
                        "print" => env.define_function(
                            "print".to_string(),
                            Value::native_function("print", 1, |args| {
                                if let Value::String(s) = &args[0] { print!("{}", s); }
                                Ok(Value::Void)
                            })
                        ),
                        "println" => env.define_function(
                            "println".to_string(),
                            Value::native_function("println", 1, |args| {
                                if let Value::String(s) = &args[0] { println!("{}", s); }
                                Ok(Value::Void)
                            })
                        ),
                        "input" => env.define_function(
                            "input".to_string(),
                            Value::native_function("input", 1, |args| {
                                use std::io::{self, Write};
                                print!("{}", args[0]);
                                let _ = io::stdout().flush();
                                let mut input = String::new();
                                match io::stdin().read_line(&mut input) {
                                    Ok(_) => Ok(Value::from_string(input.trim_end().to_string())),
                                    Err(e) => Err(format!("input error: {}", e)),
                                }
                            })
                        ),
                        _ => {}
                    }
                };
                if want_all { add("print"); add("println"); add("input"); }
                else { for n in names { add(&n); } }
            }
            "std/math" => {
                let want_all = names_opt.is_none();
                let names: Vec<String> = names_opt.clone().unwrap_or_else(|| vec![
                    "abs_i".into(), "abs_d".into(),
                    "min_i".into(), "min_d".into(),
                    "max_i".into(), "max_d".into(),
                ]);
                let mut add = |name: &str| {
                    match name {
                        "abs_i" => env.define_function(
                            "abs_i".to_string(),
                            Value::native_function("abs_i", 1, |args| {
                                if let Value::Int(n) = args[0] { Ok(Value::Int(n.abs())) } else { Err("abs_i expects int".into()) }
                            })
                        ),
                        "abs_d" => env.define_function(
                            "abs_d".to_string(),
                            Value::native_function("abs_d", 1, |args| {
                                if let Value::Double(n) = args[0] { Ok(Value::Double(n.abs())) } else { Err("abs_d expects double".into()) }
                            })
                        ),
                        "min_i" => env.define_function(
                            "min_i".to_string(),
                            Value::native_function("min_i", 2, |args| {
                                match (&args[0], &args[1]) { (Value::Int(a), Value::Int(b)) => Ok(Value::Int((*a).min(*b))), _ => Err("min_i expects (int,int)".into()) }
                            })
                        ),
                        "min_d" => env.define_function(
                            "min_d".to_string(),
                            Value::native_function("min_d", 2, |args| {
                                match (&args[0], &args[1]) { (Value::Double(a), Value::Double(b)) => Ok(Value::Double(a.min(*b))), _ => Err("min_d expects (double,double)".into()) }
                            })
                        ),
                        "max_i" => env.define_function(
                            "max_i".to_string(),
                            Value::native_function("max_i", 2, |args| {
                                match (&args[0], &args[1]) { (Value::Int(a), Value::Int(b)) => Ok(Value::Int((*a).max(*b))), _ => Err("max_i expects (int,int)".into()) }
                            })
                        ),
                        "max_d" => env.define_function(
                            "max_d".to_string(),
                            Value::native_function("max_d", 2, |args| {
                                match (&args[0], &args[1]) { (Value::Double(a), Value::Double(b)) => Ok(Value::Double(a.max(*b))), _ => Err("max_d expects (double,double)".into()) }
                            })
                        ),
                        _ => {}
                    }
                };
                if want_all { for n in ["abs_i","abs_d","min_i","min_d","max_i","max_d"] { add(n); } }
                else { for n in names { add(&n); } }
            }
            _ => {}
        }
    }
}

/// Simple signature struct for predeclaring native-only modules
#[derive(Clone)]
pub struct FuncSig {
    pub name: String,
    pub params: Vec<Ty>,
    pub ret: Ty,
}

/// Check if there is a native module by path segments
pub fn has_native_module(path: &[String]) -> bool {
    let key = path.join("/");
    matches!(key.as_str(), "std/io" | "std/math")
}

/// Return function signatures for a native-only module, for semantic predeclare
pub fn native_signatures_for(path: &[String]) -> Option<Vec<FuncSig>> {
    let key = path.join("/");
    match key.as_str() {
        "std/io" => Some(vec![
            FuncSig { name: "print".to_string(), params: vec![Ty::String], ret: Ty::Void },
            FuncSig { name: "println".to_string(), params: vec![Ty::String], ret: Ty::Void },
            FuncSig { name: "input".to_string(), params: vec![Ty::String], ret: Ty::String },
        ]),
        "std/math" => Some(vec![
            FuncSig { name: "abs_i".to_string(), params: vec![Ty::Int], ret: Ty::Int },
            FuncSig { name: "abs_d".to_string(), params: vec![Ty::Double], ret: Ty::Double },
            FuncSig { name: "min_i".to_string(), params: vec![Ty::Int, Ty::Int], ret: Ty::Int },
            FuncSig { name: "min_d".to_string(), params: vec![Ty::Double, Ty::Double], ret: Ty::Double },
            FuncSig { name: "max_i".to_string(), params: vec![Ty::Int, Ty::Int], ret: Ty::Int },
            FuncSig { name: "max_d".to_string(), params: vec![Ty::Double, Ty::Double], ret: Ty::Double },
        ]),
        _ => None,
    }
}

// NOTE: maybe move to frut_lib?
/// Dispatch primitive methods at runtime.
pub fn call_primitive_method(recv: &Value, name: &str, args: Vec<Value>) -> Option<Result<Value, String>> {
    match recv {
        Value::String(s) => match name {
            "len" => Some(Ok(Value::Int(s.len() as i64))),
            "contains" => {
                if args.len() != 1 { return Some(Err("contains expects 1 arg".into())); }
                match &args[0] {
                    Value::String(sub) => Some(Ok(Value::Bool(s.contains(sub)) )),
                    _ => Some(Err("contains expects string".into())),
                }
            }
            _ => None,
        },
        // FIXME: umm doesnt work
        Value::Int(n) => match name {
            "abs" => Some(Ok(Value::Int(n.abs()))),
            _ => None,
        },
        Value::Double(n) => match name {
            "abs" => Some(Ok(Value::Double(n.abs()))),
            _ => None,
        },
        Value::Bool(b) => match name {
            "to_int" => Some(Ok(Value::Int(if *b {1} else {0}))),
            _ => None,
        },
        _ => None,
    }
}
