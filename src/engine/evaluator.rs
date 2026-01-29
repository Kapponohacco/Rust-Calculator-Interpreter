use crate::engine::CalcError;
use std::collections::{HashMap, HashSet};
use crate::engine::Expr;

pub fn eval_ast(expr: &Expr, vars: &HashMap<String, Expr>, visited: &mut HashSet<String>) -> Result<Expr, CalcError>{
    match eval(expr,vars,visited) {
        Ok(result) => Ok(normalize(result)),
        Err(e) => Err(e)
    }
}

pub fn eval(expr: &Expr, vars: &HashMap<String, Expr>, visited: &mut HashSet<String>) -> Result<Expr, CalcError> {
    match expr {
        Expr::Number(_) => Ok(expr.clone()),

        Expr::Var(name) => {
            if visited.contains(name) {
                return Ok(Expr::Var(name.clone()));
            }

            if let Some(val) = vars.get(name) {
                visited.insert(name.clone());
                let result = eval_ast(val, vars, visited)?;
                visited.remove(name);
                Ok(result)
            } else {
                Ok(Expr::Var(name.clone()))
            }
        }

        Expr::Add(terms) => {
            let evaluated = terms
                .iter()
                .map(|t| eval_ast(t, vars, visited))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(normalize(Expr::Add(evaluated)))
        }

        Expr::Mul(terms) => {
            let evaluated = terms
                .iter()
                .map(|t| eval_ast(t, vars, visited))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(normalize(Expr::Mul(evaluated)))
        }

        Expr::Neg(inner) => {
            let val = eval_ast(inner, vars, visited)?;
            Ok(normalize(Expr::Neg(Box::new(val))))
        }

        Expr::Sub(a, b) => {
            let left = eval_ast(a, vars, visited)?;
            let right = eval_ast(b, vars, visited)?;
            Ok(normalize(Expr::Add(vec![left, Expr::Neg(Box::new(right))])))
        }

        Expr::Div(a, b) => {
            let left = eval_ast(a, vars, visited)?;
            let right = eval_ast(b, vars, visited)?;

            match (&left, &right) {
                (_, Expr::Number(0.0)) => Err(CalcError::DivisionByZero),
                (Expr::Number(x), Expr::Number(y)) => Ok(Expr::Number(x / y)),
                _ => Ok(normalize(Expr::Div(Box::new(left), Box::new(right)))),
            }
        }

        Expr::Pow(a, b) => {
            let base = eval_ast(a, vars, visited)?;
            let exp = eval_ast(b, vars, visited)?;

            match (&base, &exp) {
                (Expr::Number(x), Expr::Number(y)) => Ok(Expr::Number(x.powf(*y))),
                (_, Expr::Number(0.0)) => Ok(Expr::Number(1.0)),
                (x, Expr::Number(1.0)) => Ok(x.clone()),
                _ => Ok(Expr::Pow(Box::new(base), Box::new(exp))),
            }
        }
    }
}

pub fn normalize(expr: Expr) -> Expr {
    match expr {
        Expr::Add(terms) => {
            let mut flat = Vec::new();
            let mut sum = 0.0;

            for t in terms {
                match normalize(t) {
                    Expr::Number(n) => sum += n,
                    Expr::Add(inner) => flat.extend(inner),
                    other => flat.push(other),
                }
            }

            if sum != 0.0 {
                flat.insert(0, Expr::Number(sum));
            }

            match flat.len() {
                0 => Expr::Number(0.0),
                1 => flat.pop().unwrap(),
                _ => Expr::Add(flat),
            }
        }

        Expr::Mul(terms) => {
            let mut flat = Vec::new();
            let mut num = 1.0;
            let mut denom = 1.0;

            for t in terms {
                match normalize(t) {
                    Expr::Number(n) => num *= n,

                    Expr::Div(x, y) => {
                        flat.push(*x);
                        if let Expr::Number(d) = *y {
                            denom *= d;
                        } else {
                            flat.push(Expr::Div(Box::new(Expr::Number(1.0)), y));
                        }
                    }

                    Expr::Mul(inner) => flat.extend(inner),
                    other => flat.push(other),
                }
            }

            if denom != 1.0 {
                flat.push(Expr::Div(Box::new(Expr::Number(1.0)), Box::new(Expr::Number(denom))));
            }

            if num != 1.0 {
                flat.insert(0, Expr::Number(num));
            }

            match flat.len() {
                0 => Expr::Number(1.0),
                1 => flat.pop().unwrap(),
                _ => Expr::Mul(flat),
            }
        }


        Expr::Neg(e) => match normalize(*e) {
            Expr::Number(n) => Expr::Number(-n),
            other => Expr::Neg(Box::new(other)),
        },

        Expr::Div(a, b) => {
            let left = normalize(*a);
            let right = normalize(*b);

            match (&left, &right) {
                (Expr::Div(x, y), Expr::Number(bn)) => {
                    match **y {
                        Expr::Number(an) => {
                            Expr::Div(
                                x.clone(),
                                Box::new(Expr::Number(an * bn)),
                            )
                        }
                        _ => Expr::Div(Box::new(left), Box::new(right)),
                    }
                }

                (Expr::Number(x), Expr::Number(y)) => Expr::Number(x / y),

                _ => Expr::Div(Box::new(left), Box::new(right)),
            }
        },

        other => other,
    }
}