//! # Lambda Calculus Module
//!
//! This module provides data structures and functions for representing, parsing, manipulating, and simulating lambda calculus expressions.
//! It supports lambda expressions with multiple parameters, application, abstraction, currying, substitution, and beta reduction.
//!
//! ## Main Types
//!
//! - `LambdaExpr`: Enum representing a lambda calculus expression. Variants:
//!     - `Var(String)`: A variable.
//!     - `Abs(Vec<String>, Box<LambdaExpr>)`: An abstraction (lambda function) with one or more parameters and a body.
//!     - `App(Vec<LambdaExpr>)`: An application of one or more expressions.
//!
//! - `Lambda`: Struct representing a named lambda expression, with optional references to other named expressions and a flag for forced currying.
//!
//! ## Key Functions
//!
//! - `parse_lambda(input: &str) -> Result<LambdaExpr, String>`: Parses a string into a `LambdaExpr`.
//! - `substitute(expr: &mut LambdaExpr, sub: LambdaExpr, var: String) -> LambdaExpr`: Substitutes all occurrences of a variable in an expression with another expression.
//! - `beta_reduction(expr: &LambdaExpr) -> LambdaExpr`: Performs a single step of beta reduction on a lambda expression.
//!
//! ## LambdaExpr Methods
//!
//! - `to_tokens(&self) -> Vec<String>`: Converts the expression into a vector of tokens for further processing or display.
//! - `curry(self) -> LambdaExpr`: Converts a multi-parameter abstraction into curried form.
//! - `to_string(&self, dict: Vec<Lambda>, force_currying: bool) -> String`: Converts the expression to a string, optionally using a dictionary of named expressions and currying.
//!
//! ## Lambda Methods
//!
//! - `substitute_names(&mut self)`: Substitutes all named references in the expression with their definitions.
//! - `simulate(&mut self) -> Result<computer::SimulationResult, String>`: Simulates the reduction of the lambda expression, returning the result and computation steps.
//! - `to_tokens(&self) -> Vec<String>`: Converts the contained expression to tokens.
//!
//! ## Testing
//!
//! The module includes comprehensive unit tests for parsing, substitution, beta reduction, currying, and string/token conversion.
//!
//! ## Author
//!
//! - dp
//! 
//! # License
//! 
//! This project is licensed under the MIT License. See the LICENSE file for details.

use std::ops::Deref;

use crate::computer;

#[derive(Debug, Clone)]
pub enum LambdaExpr {
    Var(String),
    Abs(Vec<String>, Box<LambdaExpr>),
    App(Vec<LambdaExpr>),
}

impl PartialEq for LambdaExpr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LambdaExpr::Var(x), LambdaExpr::Var(y)) => x == y,
            (LambdaExpr::Abs(params1, body1), LambdaExpr::Abs(params2, body2)) => {
                params1 == params2 && body1 == body2
            }
            (LambdaExpr::App(exprs1), LambdaExpr::App(exprs2)) => {
                if exprs1.len() != exprs2.len() {
                    return false;
                }
                exprs1.iter().zip(exprs2.iter()).all(|(e1, e2)| e1 == e2)
            }
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct Lambda {
    pub expr: LambdaExpr,
    pub references: Vec<Lambda>,
    pub name: String,
    pub force_currying: bool,
}

impl PartialEq for Lambda {
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr
    }
}

/// Implementation of Lambda Calculus expression operations
impl LambdaExpr {
    /// Converts the lambda expression into a vector of tokens
    /// 
    /// Returns a vector of strings representing the tokenized lambda expression
    /// where each token is a separate string in the vector.
    /// 
    /// # Examples
    /// For abstraction: `(λx.x)` becomes `["(", "/", "x", ".", "x", ")"]`
    /// For variable: `x` becomes `["x"]`
    /// For application: `(f x)` becomes `["(", "f", "x", ")"]`
    pub fn to_tokens(&self) -> Vec<String> {
        match self {
            LambdaExpr::Abs(vars, arg) => [
                vec!["(".to_string(), "/".to_string()],
                vars.clone(),
                vec![".".to_string()],
                arg.to_tokens(),
                vec![")".to_string()],
            ]
            .concat(),
            LambdaExpr::Var(v) => vec![v.to_string()],
            LambdaExpr::App(vec) => [
                vec!["(".to_string()],
                vec.iter()
                    .map(|e| e.clone().to_tokens())
                    .collect::<Vec<Vec<String>>>()
                    .concat(),
                vec![")".to_string()],
            ]
            .concat(),
        }
    }

    /// Transforms the lambda expression into its curried form
    /// 
    /// Transforms a series of nested abstractions into a single
    /// abstraction with multiple variables
    /// 
    /// Returns a new LambdaExpr in curried form
    pub fn curry(self) -> LambdaExpr {
        match self.clone() {
            LambdaExpr::Var(_) => self,
            LambdaExpr::App(lambdas) => {
                let mut new_lambdas = Vec::new();
                for lambda in lambdas {
                    new_lambdas.push(lambda.curry());
                }
                LambdaExpr::App(new_lambdas)
            }
            LambdaExpr::Abs(vars, param) => match *param {
                LambdaExpr::Var(_) => self,
                LambdaExpr::App(_) => LambdaExpr::Abs(vars, Box::new((*param).curry())),
                LambdaExpr::Abs(vars2, param2) => {
                    LambdaExpr::Abs([vars, vars2].concat(), Box::new(param2.curry())).curry()
                }
            },
        }
    }

    /// Converts the lambda expression to a string representation
    /// 
    /// # Arguments
    /// 
    /// * `dict` - A vector of Lambda definitions that may be used for name substitution
    /// * `force_currying` - A boolean flag indicating whether to force currying before comparison
    /// 
    /// # Returns
    /// 
    /// A string representation of the lambda expression, potentially using named
    /// expressions from the dictionary if matches are found
    /// 
    /// If `force_currying` is true, expressions are compared in their curried form
    pub fn to_string(&self, dict: Vec<Lambda>, force_currying: bool) -> String {
        for dict_expr in dict.clone() {
            if force_currying {
                if dict_expr.expr.curry() == self.clone().curry() {
                    return dict_expr.name;
                }
            } else if dict_expr.expr == self.clone() {
                return dict_expr.name;
            }
        }
        match self {
            LambdaExpr::Var(v) => v.to_string(),
            LambdaExpr::Abs(params, body) => {
                "(\\".to_string()
                    + &params.join("")
                    + ".("
                    + &(*body).to_string(dict.clone(), force_currying)
                    + "))"
            }
            LambdaExpr::App(exprs) => {
                let mut s = "(".to_string();
                let mut first = true;
                for e in exprs.iter() {
                    if first {
                        first = false;
                    } else {
                        s += " ";
                    }
                    s = s + &e.clone().to_string(dict.clone(), force_currying);
                }
                s += ")";
                s.to_string()
            }
        }
    }

}

/// Implementation block for Lambda struct providing core lambda calculus operations
impl Lambda {
    /// Recursively substitutes named references in the lambda expression
    /// with their corresponding expressions until no further substitutions are possible.
    /// This process continues until the expression reaches a fixed point where no more
    /// substitutions change the overall expression.
    pub fn substitute_names(&mut self) {
        let mut self_clone = self.clone();
        for r in self.references.clone() {
            self.expr = substitute(&mut self.expr.clone(), r.expr.clone(), r.name.clone());
        }
        while *self != self_clone {
            self_clone = self.clone();
            for r in self.references.clone() {
                self.expr = substitute(&mut self.expr.clone(), r.expr.clone(), r.name.clone());
            }
        }
    }

    /// Simulates the evaluation of a lambda expression using beta reduction.
    /// 
    /// # Arguments
    /// * `max_steps` - The maximum number of reduction steps to perform.
    /// 
    /// # Returns
    /// - `Ok(SimulationResult)` containing:
    ///   - The final reduced expression as a string
    ///   - Number of registers used (always 0 for lambda calculus)
    ///   - Vector of memory operations (empty for lambda calculus)
    ///   - Number of reduction steps performed
    ///   - Vector of intermediate expressions showing the reduction process
    /// - `Err(String)` if the simulation fails
    pub fn simulate(&mut self, max_steps: usize) -> Result<computer::SimulationResult, String> {
        let mut computation = Vec::new();
        self.substitute_names();
        let mut result = self.clone();
        computation.push(result.to_string());
        let mut new_result = Lambda {
            expr: beta_reduction(&self.clone().expr),
            references: self.references.clone(),
            name: self.name.clone(),
            force_currying: self.force_currying,
        };
        computation.push(new_result.to_string());
        let mut steps = 1;
        while result != new_result.clone() || steps < max_steps {
            result = new_result.clone();
            new_result = Lambda {
                expr: beta_reduction(&new_result.clone().expr),
                references: self.references.clone(),
                name: self.name.clone(),
                force_currying: self.force_currying,
            };
            steps += 1;
            computation.push(new_result.to_string());
        }
        new_result.force_currying = true;
        Ok((new_result.to_string(), 0, Vec::new(), steps, computation))
    }

    /// Converts the lambda expression into a vector of tokens.
    /// 
    /// # Returns
    /// A vector of strings representing the tokenized lambda expression
    pub fn to_tokens(&self) -> Vec<String> {
        self.expr.to_tokens()
    }
}

impl std::fmt::Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &self
                .expr
                .clone()
                .to_string(self.references.clone(), self.force_currying)
                .as_str()
        )
    }
}

/// Parses a string into a `LambdaExpr`.
///
/// # Arguments
///
/// * `input` - A string slice representing the lambda calculus expression to parse.
///
/// # Returns
///
/// * `Ok(LambdaExpr)` if parsing is successful.
/// * `Err(String)` if the input is not a valid lambda expression.
pub fn parse_lambda(input: &str) -> Result<LambdaExpr, String> {
    let input_chars = input.chars().peekable();
    if input_chars.clone().next() != Some('(') || input_chars.clone().last() != Some(')') {
        Err("expected ()".to_string())
    } else if input_chars.clone().nth(1) == Some('\\') {
        let splitted = input.split(".");
        let variables = splitted
            .clone()
            .take(1)
            .collect::<Vec<&str>>()
            .join("")
            .chars()
            .skip(2)
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("")
            .split(" ")
            .map(|e| e.to_string())
            .collect::<Vec<String>>();
        let mut argument = splitted.skip(1).collect::<Vec<&str>>().join(".");
        argument.pop();
        Ok(LambdaExpr::Abs(
            variables,
            Box::new(parse_lambda(argument.as_str())?),
        ))
    } else {
        let mut par_count = 0;
        let mut expr_vec = Vec::new();
        let mut current = "".to_string();
        for char in input_chars.skip(1) {
            if char == '(' {
                par_count += 1;
                current = current + &char.to_string();
            } else if char == ')' {
                par_count -= 1;
                if par_count < 0 {
                    break;
                }
                current = current + &char.to_string();
                if par_count == 0 {
                    expr_vec.push(parse_lambda(current.as_str())?);
                    current = "".to_string();
                }
            } else if par_count == 0 {
                if char == ' ' {
                    if !current.is_empty() {
                        expr_vec.push(LambdaExpr::Var(current.to_string()))
                    }
                    current = "".to_string();
                } else {
                    current = current + &char.to_string();
                }
            } else {
                current = current + &char.to_string();
            }
        }
        if par_count > 0 {
            return Err("lambda format not correct".to_string());
        }
        if !current.is_empty() {
            expr_vec.push(LambdaExpr::Var(current));
        }
        if expr_vec.is_empty() {
            Err("empty body of a function".to_string())
        } else if expr_vec.len() == 1 {
            return Ok(expr_vec[0].clone());
        } else {
            return Ok(LambdaExpr::App(expr_vec));
        }
    }
}

/// Substitutes all occurrences of a variable in a lambda expression with another expression.
///
/// # Arguments
///
/// * `expr` - The lambda expression in which to perform substitution.
/// * `sub` - The expression to substitute in place of the variable.
/// * `var` - The variable name to be replaced.
///
/// # Returns
///
/// * A new `LambdaExpr` with the substitution applied.
pub fn substitute(expr: &mut LambdaExpr, sub: LambdaExpr, var: String) -> LambdaExpr {
    match expr {
        LambdaExpr::Var(x) => {
            if var == *x {
                sub
            } else {
                LambdaExpr::Var(x.to_string())
            }
        }
        LambdaExpr::Abs(param, body) => {
            let mut change = true;
            for variable in param.clone() {
                if *variable == var {
                    change = false;
                }
            }
            if change {
                LambdaExpr::Abs(param.clone(), Box::new(substitute(body, sub, var)))
            } else {
                LambdaExpr::Abs(param.clone(), body.clone())
            }
        }
        LambdaExpr::App(args) => {
            let mut new_args = Vec::new();
            for expr in args.iter() {
                new_args.push(substitute(&mut expr.to_owned(), sub.clone(), var.clone()))
            }
            LambdaExpr::App(new_args)
        }
    }
}

/// Performs a single step of beta reduction on a lambda expression.
///
/// # Arguments
///
/// * `expr` - A reference to the lambda expression to reduce.
///
/// # Returns
///
/// * A new `LambdaExpr` after applying one step of beta reduction.
pub fn beta_reduction(expr: &LambdaExpr) -> LambdaExpr {
    match expr {
        LambdaExpr::Var(x) => LambdaExpr::Var(x.clone()),
        LambdaExpr::Abs(param, body) => {
            LambdaExpr::Abs(param.clone(), Box::new(beta_reduction(body.as_ref())))
        }
        LambdaExpr::App(params) => match (*params).deref()[0].clone() {
            LambdaExpr::Var(_) => {
                let mut pars_new = Vec::new();
                let mut found = false;
                for par in params.iter() {
                    if found {
                        pars_new.push(par.clone());
                    } else {
                        let par_clone = par.clone();
                        let par_new = beta_reduction(par);
                        if par_clone != par_new {
                            found = false;
                        }
                        pars_new.push(par_new);
                    }
                }
                LambdaExpr::App(pars_new)
            }
            LambdaExpr::App(_) => {
                let mut pars_new = Vec::new();
                let mut found = false;
                for par in params.iter() {
                    if found {
                        pars_new.push(par.clone());
                    } else {
                        let par_clone = par.clone();
                        let par_new = beta_reduction(par);
                        if par_clone != par_new {
                            found = false;
                        }
                        pars_new.push(par_new);
                    }
                }
                LambdaExpr::App(pars_new)
            }
            LambdaExpr::Abs(vars, body) => {
                let mut body_copy = *body.clone();
                let mut curr_i = 0;
                for (ind, val) in params.iter().skip(1).enumerate() {
                    if ind < vars.len() {
                        body_copy = substitute(&mut body_copy, val.clone(), vars[ind].clone())
                    } else {
                        return LambdaExpr::App(
                            [vec![body_copy], params[(ind + 1)..].to_vec()].concat(),
                        );
                    }
                    curr_i = ind;
                }
                if curr_i < vars.len() - 1 {
                    LambdaExpr::Abs(vars[(curr_i + 1)..].to_vec(), Box::new(body_copy))
                } else {
                    body_copy
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_lambda() {
        let result = parse_lambda("(\\x.(x))").unwrap();
        assert_eq!(
            result,
            LambdaExpr::Abs(
                vec!["x".to_string()],
                Box::new(LambdaExpr::Var("x".to_string()))
            )
        );
    }

    #[test]
    fn test_parse_multi_param_lambda() {
        let result = parse_lambda("(\\x y.(x y))").unwrap();
        assert_eq!(
            result,
            LambdaExpr::Abs(
                vec!["x".to_string(), "y".to_string()],
                Box::new(LambdaExpr::App(vec![
                    LambdaExpr::Var("x".to_string()),
                    LambdaExpr::Var("y".to_string())
                ]))
            )
        );
    }

    #[test]
    fn test_beta_reduction() {
        let expr = parse_lambda("((\\x.(x)) y)").unwrap();
        let result = beta_reduction(&expr);
        assert_eq!(result, LambdaExpr::Var("y".to_string()));
    }

    #[test]
    fn test_nested_application() {
        let expr = parse_lambda("((\\x.(\\y.(x y))) a b)").unwrap();
        let result = beta_reduction(&expr);
        let result = beta_reduction(&result);
        assert_eq!(
            result,
            LambdaExpr::App(vec![
                LambdaExpr::Var("a".to_string()),
                LambdaExpr::Var("b".to_string())
            ])
        );
    }

    #[test]
    fn test_substitute() {
        let mut expr = LambdaExpr::Var("x".to_string());
        let sub = LambdaExpr::Var("y".to_string());
        let result = substitute(&mut expr, sub, "x".to_string());
        assert_eq!(result, LambdaExpr::Var("y".to_string()));
    }
    #[test]
    fn test_lambda_with_multiple_args() {
        let expr = parse_lambda("((\\x y z.(x y z)) a b c)").unwrap();
        let result = beta_reduction(&expr);
        assert_eq!(
            result,
            LambdaExpr::App(vec![
                LambdaExpr::Var("a".to_string()),
                LambdaExpr::Var("b".to_string()),
                LambdaExpr::Var("c".to_string())
            ])
        );
    }

    #[test]
    fn test_partial_application() {
        let expr = parse_lambda("((\\x y.(x y)) a)").unwrap();
        let result = beta_reduction(&expr);
        assert_eq!(
            result,
            LambdaExpr::Abs(
                vec!["y".to_string()],
                Box::new(LambdaExpr::App(vec![
                    LambdaExpr::Var("a".to_string()),
                    LambdaExpr::Var("y".to_string())
                ]))
            )
        );
    }

    #[test]
    fn test_nested_lambda() {
        let expr = parse_lambda("(\\x.(\\y.(\\z.(x y z))))").unwrap();
        assert_eq!(
            expr,
            LambdaExpr::Abs(
                vec!["x".to_string()],
                Box::new(LambdaExpr::Abs(
                    vec!["y".to_string()],
                    Box::new(LambdaExpr::Abs(
                        vec!["z".to_string()],
                        Box::new(LambdaExpr::App(vec![
                            LambdaExpr::Var("x".to_string()),
                            LambdaExpr::Var("y".to_string()),
                            LambdaExpr::Var("z".to_string())
                        ]))
                    ))
                ))
            )
        );
    }

    #[test]
    fn test_curry_lambda() {
        let expr = parse_lambda("(\\x y.(x y))").unwrap();
        let result = expr.curry();
        assert_eq!(
            result,
            LambdaExpr::Abs(
                vec!["x".to_string(), "y".to_string()],
                Box::new(LambdaExpr::App(vec![
                    LambdaExpr::Var("x".to_string()),
                    LambdaExpr::Var("y".to_string())
                ]))
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_invalid_lambda() {
        parse_lambda("(x y").unwrap();
    }
    #[test]
    fn test_complex_beta_reduction() {
        let expr = parse_lambda("((\\x.((\\y.(y x)) z)) a)").unwrap();
        let result1 = beta_reduction(&expr);
        let result2 = beta_reduction(&result1);
        assert_eq!(
            result2,
            LambdaExpr::App(vec![
                LambdaExpr::Var("z".to_string()),
                LambdaExpr::Var("a".to_string())
            ])
        );
    }

    #[test]
    fn test_beta_reduction_with_multiple_applications() {
        let expr = parse_lambda("((\\x y.((\\z.(z x)) y)) a b)").unwrap();
        let result1 = beta_reduction(&expr);
        let result2 = beta_reduction(&result1);
        let result3 = beta_reduction(&result2);
        assert_eq!(
            result3,
            LambdaExpr::App(vec![
                LambdaExpr::Var("b".to_string()),
                LambdaExpr::Var("a".to_string())
            ])
        );
    }

    #[test]
    fn test_beta_reduction_identity() {
        let expr = parse_lambda("((\\x.(x)) ((\\y.(y)) a))").unwrap();
        let result1 = beta_reduction(&expr);
        let result2 = beta_reduction(&result1);
        assert_eq!(result2, LambdaExpr::Var("a".to_string()));
    }

    #[test]
    fn test_beta_reduction_no_reduction_possible() {
        let expr = parse_lambda("(x y)").unwrap();
        let result = beta_reduction(&expr);
        assert_eq!(
            result,
            LambdaExpr::App(vec![
                LambdaExpr::Var("x".to_string()),
                LambdaExpr::Var("y".to_string())
            ])
        );
    }

    #[test]
    fn test_beta_reduction_nested_abstractions() {
        let expr = parse_lambda("((\\x.(\\y.(x y))) a)").unwrap();
        let result = beta_reduction(&expr);
        assert_eq!(
            result,
            LambdaExpr::Abs(
                vec!["y".to_string()],
                Box::new(LambdaExpr::App(vec![
                    LambdaExpr::Var("a".to_string()),
                    LambdaExpr::Var("y".to_string())
                ]))
            )
        );
    }
    #[test]
    fn test_lambda_expr_to_string() {
        let expr = parse_lambda("(\\x.(x))").unwrap();
        assert_eq!(expr.to_string(vec![], false), "(\\x.(x))");
    }

    #[test]
    fn test_lambda_expr_to_string_with_application() {
        let expr = parse_lambda("((\\x.(x)) y)").unwrap();
        assert_eq!(expr.to_string(vec![], false), "((\\x.(x)) y)");
    }

    #[test]
    fn test_lambda_expr_to_tokens_simple() {
        let expr = parse_lambda("(\\x.(x))").unwrap();
        assert_eq!(
            expr.to_tokens(),
            vec!["(", "/", "x", ".", "x", ")"]
        );
    }

    #[test]
    fn test_lambda_expr_to_tokens_application() {
        let expr = parse_lambda("(x y)").unwrap();
        assert_eq!(
            expr.to_tokens(),
            vec!["(", "x", "y", ")"]
        );
    }

    #[test]
    fn test_lambda_expr_to_tokens_complex() {
        let expr = parse_lambda("(\\x y.(x y))").unwrap();
        assert_eq!(
            expr.to_tokens(),
            vec!["(", "/", "x", "y", ".", "(", "x", "y", ")", ")"]
        );
    }

    #[test]
    fn test_lambda_to_string_with_references() {
        let expr = parse_lambda("(\\x.(x))").unwrap();
        let reference = Lambda {
            expr: expr.clone(),
            references: vec![],
            name: "ID".to_string(),
            force_currying: false
        };
        assert_eq!(expr.to_string(vec![reference], false), "ID");
    }
}
