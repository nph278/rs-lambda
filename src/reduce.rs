use crate::types::Expression;

fn replace(expr: &Expression, var: &str, replacement: &Expression) -> Expression {
    match expr {
        Expression::Lambda(a, b) => {
            if a == var {
                Expression::Lambda(a.clone(), b.clone())
            } else {
                Expression::Lambda(a.to_string(), Box::new(replace(b, var, replacement)))
            }
        }
        Expression::Application(a, b) => Expression::Application(
            Box::new(replace(a, var, replacement)),
            Box::new(replace(b, var, replacement)),
        ),
        Expression::Variable(x) => {
            if x == var {
                replacement.clone()
            } else {
                Expression::Variable(x.to_string())
            }
        }
    }
}

pub fn reduce(input: &Expression) -> Option<Expression> {
    match input {
        Expression::Application(left, right) => match &**left {
            Expression::Lambda(var, expr) => Some(replace(&expr, &var, right)),
            _ => {
                if let Some(left) = reduce(left) {
                    Some(Expression::Application(Box::new(left), right.clone()))
                } else {
                    if let Some(right) = reduce(right) {
                        Some(Expression::Application(left.clone(), Box::new(right)))
                    } else {
                        None
                    }
                }
            }
        },
        _ => None,
    }
}
