// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

use sqlparser::ast;
use std::fmt;

use crate::contexts::Context;
use crate::error::Result;
use crate::planners::{EmptyPlan, ExpressionPlan, FormatterSettings, PlanNode};

#[derive(Clone)]
pub struct FilterPlan {
    description: String,
    pub predicate: ExpressionPlan,
}

impl FilterPlan {
    pub fn build_plan(ctx: Context, limit: &Option<ast::Expr>) -> Result<PlanNode> {
        match limit {
            Some(ref expr) => Ok(PlanNode::Filter(FilterPlan {
                description: "".to_string(),
                predicate: ExpressionPlan::build_plan(ctx, expr)?,
            })),
            None => Ok(PlanNode::Empty(EmptyPlan {})),
        }
    }
    pub fn name(&self) -> &'static str {
        "FilterPlan"
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = format!(" ({})", description);
    }

    pub fn format(&self, f: &mut fmt::Formatter, setting: &mut FormatterSettings) -> fmt::Result {
        let indent = setting.indent;

        if indent > 0 {
            writeln!(f)?;
            for _ in 0..indent {
                write!(f, "{}", setting.indent_char)?;
            }
        }
        write!(
            f,
            "{} Filter: {:?}{}",
            setting.prefix, self.predicate, self.description
        )
    }
}
