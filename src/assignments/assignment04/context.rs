//! Calculator.

use std::collections::HashMap;

use anyhow::*;
use etrace::*;
use itertools::max;
use rayon::string;

use crate::assignments::assignment08::church::exp;

use super::syntax::{BinOp, Command, Expression};

/// Calculator's context.
#[derive(Debug, Default, Clone)]
pub struct Context {
    anonymous_counter: usize,
    variables: HashMap<String, f64>,
}

impl Context {
    /// Creates a new context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the current anonymous variable counter.
    pub fn current_counter(&self) -> usize {
        self.anonymous_counter
    }

    /// Calculates the given expression. (We assume the absence of overflow.)
    pub fn calc_expression(&self, expression: &Expression) -> Result<f64> {
        match expression {
            Expression::Num(v) => Ok(*v),
            Expression::Variable(v) => self
                .variables
                .get(v)
                .copied()
                .ok_or(anyhow!("error getting var {}'s value!", v)),
            Expression::BinOp { op, lhs, rhs } => {
                let left_val = self.calc_expression(lhs)?;
                let right_val = self.calc_expression(rhs)?;
                match op {
                    BinOp::Add => Ok(left_val + right_val),
                    BinOp::Subtract => Ok(left_val - right_val),
                    BinOp::Divide => {
                        if right_val == 0f64 {
                            bail!("Division by zero error!");
                        }
                        Ok(left_val / right_val)
                    }
                    BinOp::Multiply => Ok(left_val * right_val),
                    BinOp::Power => Ok(left_val.powf(right_val)),
                }
            }
        }
    }

    /// Calculates the given command. (We assume the absence of overflow.)
    ///
    /// If there is no variable lhs in the command (i.e. `command.variable = None`), its value
    /// should be stored at `$0`, `$1`, `$2`, ... respectively.
    ///
    /// # Example
    ///
    /// After calculating commad `3 + 5` => Context's variables = `{($0,8)}`
    ///
    /// After calculating commad `v = 3 - 2` => Context's variables = `{($0,8),(v,1))}`
    ///
    /// After calculating commad `3 ^ 2` => Context's variables = `{($0,8),(v,1),($1,9)}`
    pub fn calc_command(&mut self, command: &Command) -> Result<(String, f64)> {
        let Command {
            variable: var,
            expression: expr,
        } = command;
        let val = self.calc_expression(expr)?;
        let var_name = var
            .clone()
            .unwrap_or_else(|| format!("${}", self.anonymous_counter));
        let _unused = self
            .variables
            .entry(var_name.clone())
            .and_modify(|v| *v = val)
            .or_insert(val);

        if var.is_none() {
            self.anonymous_counter += 1;
        }
        Ok((var_name, val))
    }
}
