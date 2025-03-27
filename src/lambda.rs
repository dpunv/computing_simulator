// file: lambda.rs
// Project: Computing Simulator
// author: dp

// lambda calculus: parsing, execution, output

use std::ops::Deref;

use crate::computer;

#[derive(Debug, Clone)]
pub enum LambdaExpr {
    Var(String),
    Abs(Vec<String>, Box<LambdaExpr>),
    App(Box<Vec<LambdaExpr>>),
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
    pub name: String
}

impl PartialEq for Lambda {
    fn eq(&self, other: &Self) -> bool {
        &self.expr == &other.expr
    }
}

impl Lambda {
    pub fn simulate(&mut self) -> Result<computer::SimulationResult, String>{
        let mut computation = Vec::new();
        for r in self.references.clone() {
            self.expr = substitute(&mut self.expr.clone(), r.expr.clone(), r.name.clone());
        }
        //print_lambda_as_tree(self.clone());
        let mut result = self.clone();
        computation.push(result.to_string());
        let mut new_result = Lambda{
            expr:beta_reduction(&self.clone().expr),
            references: self.references.clone(),
            name: self.name.clone()
        };
        computation.push(new_result.to_string());
        //println!("{}", new_result.clone().to_string());
        let mut steps = 1;
        while result != new_result.clone() {
            result = new_result.clone();
            new_result = Lambda{
                expr:beta_reduction(&new_result.clone().expr),
                references: self.references.clone(),
                name: self.name.clone()
            };
            steps += 1;
            computation.push(new_result.to_string());
            //println!("{}", new_result.clone().to_string());
        }
        return Ok((new_result.to_string(), 0, Vec::new(), steps, computation));
    }

    pub fn to_string(&self) -> String{
        //println!("NAME: {}", self.name);
        fn expr_to_string(expr: &LambdaExpr) -> String {
            //let padding = " ".repeat(indent);
            match expr {
                LambdaExpr::Var(v) => v.to_string(),
                LambdaExpr::Abs(params, body) => "(\\".to_string() + &params.join("") + "." + &expr_to_string(body) + ")",
                LambdaExpr::App(exprs) => {
                    let mut s = "(".to_string();
                    let mut first = true;
                    for e in exprs.iter() {
                        if first{
                            first = false;
                        } else {
                            s = s + " ";
                        }
                        s = s + &expr_to_string(e);
                    }
                    s = s + ")";
                    s.to_string()
                },
            }
        }
        return expr_to_string(&self.expr);
    }
}

pub fn parse_lambda(input: &str) -> Result<LambdaExpr, String> {
    //println!("input: {}", input);
    let input_chars = input.chars().peekable();
    // every expression must have a starting and closing ()
    if input_chars.clone().next() != Some('(') || input_chars.clone().last() != Some(')') {
        return Err("expected ()".to_string());
    } else {
        if input_chars.clone().skip(1).next() == Some('\\'){
            let splitted = input.split(".");
            let variables = splitted.clone().take(1).collect::<Vec<&str>>().join("").chars().skip(2).map(|e| e.to_string()).collect::<Vec<String>>().join("").split(" ").map(|e| e.to_string()).collect::<Vec<String>>();
            let mut argument = splitted.skip(1).collect::<Vec<&str>>().join(".");
            argument.pop();
            match parse_lambda(argument.as_str()){
                Ok(expr) => {
                    //println!("arg = {}, vars = {}", argument, variables.join(" "));
                    return Ok(LambdaExpr::Abs(variables, Box::new(expr)))
                },
                Err(error) => return Err(error)
            }
        } else {
            let mut par_count = 0;
            let mut expr_vec = Vec::new();
            let mut current = "".to_string();
            for char in input_chars.skip(1) {
                if char == '(' {
                    par_count += 1;
                    current = current + &char.to_string();
                } else if char == ')'{
                    par_count -= 1;
                    if par_count < 0 {
                        break;
                    }
                    current = current + &char.to_string();
                    if par_count == 0 {
                        match parse_lambda(current.as_str()) {
                            Ok(expr) => {
                                expr_vec.push(expr);
                                current = "".to_string();
                            },
                            Err(error) => return Err(error)
                        }
                    }
                } else if par_count == 0 {
                    if char == ' ' {
                        if current != ""{
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
            if current != "" {
                expr_vec.push(LambdaExpr::Var(current));
            }
            if expr_vec.len() == 0 {
                return Err("empty body of a function".to_string());
            } else if expr_vec.len() == 1{
                return Ok(expr_vec[0].clone())
            } else {
                return Ok(LambdaExpr::App(Box::new(expr_vec)));
            }
        }
    }
}

pub fn substitute(expr: &mut LambdaExpr, sub: LambdaExpr, var: String) -> LambdaExpr {
    match expr {
        LambdaExpr::Var(x) => {
            if var == *x {
                return sub;
            } else {
                return LambdaExpr::Var(x.to_string());
            }
        },
        LambdaExpr::Abs(param, body) => {
            let mut change = true;
            for variable in param.clone() {
                if *variable == var {
                    change = false;
                }
            }
            if change {
                return LambdaExpr::Abs(param.clone(), Box::new(substitute(body, sub, var)));
            } else {
                return LambdaExpr::Abs(param.clone(), body.clone());
            }
        },
        LambdaExpr::App(args) => {
            let mut new_args = Vec::new();
            for expr in args.iter() {
                new_args.push(substitute(&mut expr.to_owned(), sub.clone(), var.clone()))
            }
            return LambdaExpr::App(Box::new(new_args))
        }
    }
}

/* fn alfa_conversion(expr: &mut LambdaExpr, orig: String, new: String) -> LambdaExpr {
    return substitute(expr, LambdaExpr::Var(new), orig)
} */

fn beta_reduction(expr: &LambdaExpr) -> LambdaExpr {
    match expr {
        LambdaExpr::Var(x) => LambdaExpr::Var(x.clone()),
        LambdaExpr::Abs(param, body) => LambdaExpr::Abs(param.clone(), Box::new(beta_reduction(body.as_ref()))),
        LambdaExpr::App(params) => {
            match (*params).deref()[0].clone() {
                LambdaExpr::Var(_) => {
                    let mut par_new = Vec::new();
                    for par in params.iter() {
                        par_new.push(beta_reduction(par));
                    }
                    return LambdaExpr::App(Box::new(par_new));
                },
                LambdaExpr::App(_) => {
                    let mut par_new = Vec::new();
                    for par in params.iter() {
                        par_new.push(beta_reduction(par));
                    }
                    return LambdaExpr::App(Box::new(par_new));
                },
                LambdaExpr::Abs(vars, body) => {
                    let mut body_copy = *body.clone();
                    let mut curr_i = 0;
                    for (ind, val) in params.iter().skip(1).enumerate() {
                        if ind < vars.len() {
                            body_copy = substitute(&mut body_copy, val.clone(), vars[ind].clone())
                        } else {
                            return LambdaExpr::App(Box::new([vec![body_copy], params[ind..].to_vec()].concat()))
                        }
                        curr_i = ind;
                    }
                    if curr_i < vars.len()-1{
                        return LambdaExpr::Abs(vars[curr_i..].to_vec(), Box::new(body_copy));
                    } else {
                        return body_copy;
                    }
                },
            }
        }
    }
}

/* pub fn print_lambda_as_tree(l: Lambda) {
    println!("NAME: {}", l.name);
    fn print_expr(expr: &LambdaExpr, indent: usize) {
        let padding = " ".repeat(indent);
        match expr {
            LambdaExpr::Var(v) => println!("{}Var({})", padding, v),
            LambdaExpr::Abs(params, body) => {
                println!("{}Abs {:?}", padding, params);
                print_expr(body, indent + 4);
            },
            LambdaExpr::App(exprs) => {
                println!("{}App", padding);
                for e in exprs.iter() {
                    print_expr(e, indent + 4);
                }
            },
        }
    }
    print_expr(&l.expr, 0);
} */