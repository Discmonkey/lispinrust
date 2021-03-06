use crate::types::ast::{LispValue};
use crate::types::list::{List};
use crate::types::unit::Unit;
use crate::reader::tokenizer::TokenType;
use crate::types::env::Scope;

#[macro_export]
macro_rules! arg_return {
    ($fname:ident, $numargs:expr, $list:expr) => {
        if $list.len() != $numargs + 1 {
            return LispValue::Error(stringify!($fname, " takes ", $numargs, " args").to_string());
        }
    }
}


pub fn eval_ast(root: &LispValue, mut env: &mut Scope) -> LispValue {

    // handle macro expansion
    if let LispValue::List(list) = root {
        if let Some(first_token) = list.first_token() {
            if let Some(LispValue::Macro(l)) = env.get(first_token.get_text()) {
                let evaluated_macro = l(list, env);
                return eval_ast(&evaluated_macro, env)
            }
        }
    }

    match root {
        LispValue::List(list) => eval_list(&list, &mut env),
        LispValue::Unit(atom) => eval_symbol(&atom, env),
        _ => root.clone()
    }
}

pub fn eval_list(list: &List, env: &mut Scope) -> LispValue {

    if list.len() == 0 {
        return LispValue::Nil
    }

    let op = eval_ast(&list[0], env);

    match op {
        LispValue::Function(f) => f(list, env),
        LispValue::Error(s) => LispValue::Error(s),
        _ => LispValue::Error(format!("cannot evaluate list: {}", list).to_string())
    }
}

fn convert_string(s: &String) -> LispValue {
    if !s.starts_with("\"")|| !s.ends_with("\"") {
        LispValue::Error("malformatted string".to_string())
    } else {
        let mut copy = s.replace("\\n", "\n");
        copy.pop();
        copy.remove(0);

        LispValue::String(copy)
    }
}

pub fn eval_symbol(atom: &Unit, env: &mut Scope) -> LispValue {
    if atom.token().get_type() == TokenType::String {
        return convert_string(atom.token().get_text())
    }

    let string = atom.token().get_text();
    // check if this symbol is defined
    // note that this means our language currently allows for redefinitions
    if let Some(result) = env.get(string) {
        return result
    }

    // in order try nil, true, false and finally check for floats and ints
    match string.as_str() {
        "nil" => LispValue::Nil,
        "true" => LispValue::Boolean(true),
        "false" => LispValue::Boolean(false),
        _ => {
            match string.parse::<i64>() {
                Ok(i) => LispValue::Int(i),

                _ => match string.parse::<f64>() {
                    Ok(f) => LispValue::Float(f),
                    _ => LispValue::Error(format!("could not parse symbol: {}", string))
                }
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::{eval_symbol, LispValue};
    use crate::reader::tokenizer::{Token, TokenType};
    use crate::types::atom::Unit;
    use crate::types::env::Scope;

    #[test]
    fn test_ints_and_floats() {
        let mut env = Scope::new();

        let test_token_float = Token::new("3.14".to_string(), TokenType::Symbol);
        let test_token_int = Token::new("3".to_string(), TokenType::Symbol);
        let test_token_gibberish = Token::new("1984.38471jf".to_string(), TokenType::Symbol);

        let lisp_value_float =Unit::new(test_token_float);
        let lisp_value_int = Unit::new(test_token_int);
        let lisp_val_bad = Unit::new(test_token_gibberish);

        match eval_symbol(&lisp_value_float, &mut env) {
            LispValue::Float(v) => assert_eq!(v, 3.14),
            _ => assert!(false)
        }

        match eval_symbol(&lisp_value_int, &mut env) {
            LispValue::Int(v) => assert_eq!(v, 3),
            _ => assert!(false)
        }

        match eval_symbol(&(lisp_val_bad), &mut env) {
            LispValue::Error(_) => assert!(true),
            _ => assert!(false)
        }

    }


    #[test]
    fn true_false() {
        let mut env = Scope::new();
        let test_token_false = Unit::new(Token::new("false".to_string(), TokenType::Symbol));
        let test_token_true = Unit::new(Token::new("true".to_string(), TokenType::Symbol));

        match eval_symbol(&(test_token_false), &mut env) {
            LispValue::Boolean(v) => assert!(!v),
            _ => assert!(false)
        }

        match eval_symbol(&(test_token_true), &mut env) {
            LispValue::Boolean(v) => assert!(v),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_nil() {
        let mut env = Scope::new();

        let test_nil_token = Unit::new(Token::new("nil".to_string(), TokenType::Symbol));

        match eval_symbol(&test_nil_token, &mut env) {
            LispValue::Nil => assert!(true),
            _ => assert!(false)
        }
    }
}