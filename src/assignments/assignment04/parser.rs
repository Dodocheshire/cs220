#![allow(deprecated)]

//! Parser.

use anyhow::{anyhow, bail, Error, Result};
use etrace::*;
use lazy_static::*;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{self, *};
use pest::Parser;

use super::syntax::*;

#[allow(missing_docs)]
#[allow(missing_debug_implementations)]
mod inner {
    use pest_derive::*;

    #[derive(Parser)]
    #[grammar = "assignments/assignment04/syntax.pest"]
    pub(crate) struct SyntaxParser;
}

use inner::*;

/// Parses command.
///
/// ## Operator Associativty
///
/// For associativity of each operator, please follow [here](https://docs.rs/pest/latest/pest/prec_climber/struct.PrecClimber.html#examples).
///
/// e.g. `1+2+3` should be parsed into `(1+2)+3`, not `1+(2+3)` because the associativity of
/// plus("add" in our hw) operator is `Left`.
pub fn parse_command(line: &str) -> Result<Command> {
    let climber = PrecClimber::new(vec![
        Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::subtract, Assoc::Left),
        Operator::new(Rule::multiply, Assoc::Left) | Operator::new(Rule::divide, Assoc::Left),
        Operator::new(Rule::power, Assoc::Right),
    ]);
    let mut parse_result = SyntaxParser::parse(Rule::command, line)?;
    let first = parse_result.next().ok_or_else(|| anyhow!("null input"))?;
    let mut variable = None;
    let expression_pair = if first.as_rule() == Rule::var {
        variable = Some(first.as_str().to_string());
        parse_result
            .next()
            .ok_or_else(|| anyhow!("match first var, but found no expression"))?
    } else {
        first
    };
    // 将1+2+3 结构表达式转换成 (1+2)+3
    let expr: Expression = consume_expr(expression_pair.into_inner(), &climber);
    Ok(Command {
        variable: variable,
        expression: expr,
    })
}

fn consume_expr(expr_pair: Pairs<'_, Rule>, climber: &PrecClimber<Rule>) -> Expression {
    let mut primary = |pair: Pair<'_, Rule>| match pair.as_rule() {
        Rule::var => Expression::Variable(pair.as_str().to_string()),
        Rule::num => Expression::Num(pair.as_str().parse::<f64>().unwrap()),
        Rule::expr => consume_expr(pair.into_inner(), &climber),
        _ => unreachable!(),
    };

    let mut infix = |expr1: Expression, operator: Pair<'_, Rule>, expr2: Expression| {
        let op_type = match operator.as_rule() {
            Rule::add => BinOp::Add,
            Rule::subtract => BinOp::Subtract,
            Rule::multiply => BinOp::Multiply,
            Rule::divide => BinOp::Divide,
            Rule::power => BinOp::Power,
            _ => unreachable!(),
        };
        Expression::BinOp {
            op: op_type,
            lhs: Box::new(expr1),
            rhs: Box::new(expr2),
        }
    };
    climber.climb(expr_pair, primary, infix)
}
