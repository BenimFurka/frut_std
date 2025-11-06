//! Standard library for the Frut language

use frut_lib::value::{RuntimeEnvironment, Value};
use frut_lib::types::Type as Ty;

/// Register implementations for imported modules.
pub fn register_modules(env: &mut RuntimeEnvironment, modules: &[(String, Option<Vec<String>>)]) {
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
                    "min_i".into(), "min_d".into(),
                    "max_i".into(), "max_d".into(),
                    "clamp_i".into(), "clamp_d".into(),
                ]);
                let mut add = |name: &str| {
                    match name {
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
                        "clamp_i" => env.define_function(
                            "clamp_i".to_string(),
                            Value::native_function("clamp_i", 3, |args| {
                                match (&args[0], &args[1], &args[2]) {
                                    (Value::Int(v), Value::Int(min), Value::Int(max)) => Ok(Value::Int((*v).clamp(*min, *max))),
                                    _ => Err("clamp_i expects (int,int,int)".into()),
                                }
                            })
                        ),
                        "clamp_d" => env.define_function(
                            "clamp_d".to_string(),
                            Value::native_function("clamp_d", 3, |args| {
                                match (&args[0], &args[1], &args[2]) {
                                    (Value::Double(v), Value::Double(min), Value::Double(max)) => Ok(Value::Double(v.clamp(*min, *max))),
                                    _ => Err("clamp_d expects (double,double,double)".into()),
                                }
                            })
                        ),
                        "pi" => env.define_function(
                            "pi".to_string(),
                            Value::native_function("pi", 0, |_args| {
                                Ok(Value::Double(core::f64::consts::PI))
                            }),
                        ),
                        "factorial" => env.define_function(
                            "factorial".to_string(),
                            Value::native_function("factorial", 1, |args| {
                                match &args[0] {
                                    Value::Int(n) => {
                                        if *n < 0 {
                                            return Err("factorial expects non-negative int".to_string());
                                        }
                                        let mut acc: i128 = 1;
                                        let mut i: i128 = 1;
                                        let limit: i128 = *n as i128;
                                        while i <= limit {
                                            acc = acc.checked_mul(i).ok_or_else(|| "factorial overflow".to_string())?;
                                            i += 1;
                                        }

                                        if acc > i128::from(i64::MAX) { return Err("factorial overflow".to_string()); }
                                        Ok(Value::Int(acc as i64))
                                    }
                                    _ => Err("factorial expects (int)".to_string()),
                                }
                            }),
                        ),
                        "sqrt" => env.define_function(
                            "sqrt".to_string(),
                            Value::native_function("sqrt", 1, |args| {
                                match &args[0] {
                                    Value::Double(d) => {
                                        if *d < 0.0 { return Err("sqrt expects non-negative double".to_string()); }
                                        Ok(Value::Double(d.sqrt()))
                                    }
                                    _ => Err("sqrt expects (double)".to_string()),
                                }
                            }),
                        ),
                        "sin" => env.define_function(
                            "sin".to_string(),
                            Value::native_function("sin", 1, |args| {
                                match &args[0] {
                                    Value::Double(d) => Ok(Value::Double(d.sin())),
                                    _ => Err("sin expects (double)".to_string()),
                                }
                            }),
                        ),
                        "cos" => env.define_function(
                            "cos".to_string(),
                            Value::native_function("cos", 1, |args| {
                                match &args[0] {
                                    Value::Double(d) => Ok(Value::Double(d.cos())),
                                    _ => Err("cos expects (double)".to_string()),
                                }
                            }),
                        ),
                        "tan" => env.define_function(
                            "tan".to_string(),
                            Value::native_function("tan", 1, |args| {
                                match &args[0] {
                                    Value::Double(d) => Ok(Value::Double(d.tan())),
                                    _ => Err("tan expects (double)".to_string()),
                                }
                            }),
                        ),
                        "asin" => env.define_function(
                            "asin".to_string(),
                            Value::native_function("asin", 1, |args| {
                                match &args[0] {
                                    Value::Double(d) => Ok(Value::Double(d.asin())),
                                    _ => Err("asin expects (double)".to_string()),
                                }
                            }),
                        ),
                        "acos" => env.define_function(
                            "acos".to_string(),
                            Value::native_function("acos", 1, |args| {
                                match &args[0] {
                                    Value::Double(d) => Ok(Value::Double(d.acos())),
                                    _ => Err("acos expects (double)".to_string()),
                                }
                            }),
                        ),
                        "atan" => env.define_function(
                            "atan".to_string(),
                            Value::native_function("atan", 1, |args| {
                                match &args[0] {
                                    Value::Double(d) => Ok(Value::Double(d.atan())),
                                    _ => Err("atan expects (double)".to_string()),
                                }
                            }),
                        ),
                        "atan2" => env.define_function(
                            "atan2".to_string(),
                            Value::native_function("atan2", 2, |args| {
                                match (&args[0], &args[1]) {
                                    (Value::Double(y), Value::Double(x)) => Ok(Value::Double(y.atan2(*x))),
                                    _ => Err("atan2 expects (double,double)".to_string()),
                                }
                            }),
                        ),
                        _ => {}
                    }
                };
                if want_all { for n in ["min_i","min_d","max_i","max_d","abs_i","abs_d","pi","factorial","sqrt","sin","cos","tan","asin","acos","atan","atan2"] { add(n); } }
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

/// Return function signatures for a module, for semantic predeclare
pub fn signatures_for(path: &[String]) -> Option<Vec<FuncSig>> {
    let key = path.join("/");
    match key.as_str() {
        "std/io" => Some(vec![
            FuncSig { name: "print".to_string(), params: vec![Ty::String], ret: Ty::Void },
            FuncSig { name: "println".to_string(), params: vec![Ty::String], ret: Ty::Void },
            FuncSig { name: "input".to_string(), params: vec![Ty::String], ret: Ty::String },
        ]),
        "std/math" => Some(vec![
            FuncSig { name: "min_i".to_string(), params: vec![Ty::Int, Ty::Int], ret: Ty::Int },
            FuncSig { name: "min_d".to_string(), params: vec![Ty::Double, Ty::Double], ret: Ty::Double },
            FuncSig { name: "max_i".to_string(), params: vec![Ty::Int, Ty::Int], ret: Ty::Int },
            FuncSig { name: "max_d".to_string(), params: vec![Ty::Double, Ty::Double], ret: Ty::Double },
            FuncSig { name: "clamp_i".to_string(), params: vec![Ty::Int, Ty::Int, Ty::Int], ret: Ty::Int },
            FuncSig { name: "clamp_d".to_string(), params: vec![Ty::Double, Ty::Double, Ty::Double], ret: Ty::Double },
            FuncSig { name: "pi".to_string(), params: vec![], ret: Ty::Double },
            FuncSig { name: "factorial".to_string(), params: vec![Ty::Int], ret: Ty::Int },
            FuncSig { name: "sqrt".to_string(), params: vec![Ty::Double], ret: Ty::Double },
            FuncSig { name: "sin".to_string(), params: vec![Ty::Double], ret: Ty::Double },
            FuncSig { name: "cos".to_string(), params: vec![Ty::Double], ret: Ty::Double },
            FuncSig { name: "tan".to_string(), params: vec![Ty::Double], ret: Ty::Double },
            FuncSig { name: "asin".to_string(), params: vec![Ty::Double], ret: Ty::Double },
            FuncSig { name: "acos".to_string(), params: vec![Ty::Double], ret: Ty::Double },
            FuncSig { name: "atan".to_string(), params: vec![Ty::Double], ret: Ty::Double },
            FuncSig { name: "atan2".to_string(), params: vec![Ty::Double, Ty::Double], ret: Ty::Double },
        ]),
        _ => None,
    }
}
