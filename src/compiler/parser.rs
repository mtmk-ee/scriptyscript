//! The parser for the `ScriptyScript` language.
//!
//! The parser takes source code and transforms it into an [AST](crate::compiler::ast).

use once_cell::sync::OnceCell;
use pest::{
    pratt_parser::{Assoc, Op, PrattParser},
    Parser,
};

use super::ast::{AstNode, BinaryOperationKind, Number, UnaryOperationKind};

type Pair<'a> = pest::iterators::Pair<'a, Rule>;
type Pairs<'a> = pest::iterators::Pairs<'a, Rule>;

static EXPRESSION_PARSER: OnceCell<PrattParser<Rule>> = OnceCell::new();

#[derive(pest_derive::Parser)]
#[grammar = "compiler/grammar.pest"]
struct GrammarParser {}

/// Try to parse a string into an [`AstNode`].
///
/// # Errors
/// Returns a [`pest::error::Error`] if the string cannot be parsed.
pub fn parse(s: impl AsRef<str>) -> Result<AstNode, Box<pest::error::Error<Rule>>> {
    let mut pairs = GrammarParser::parse(Rule::script, s.as_ref())?;
    Ok(parse_statements(pairs.next().unwrap().into_inner()))
}

/// Parse a block of statements into an [`AstNode`]
fn parse_statements(pairs: Pairs) -> AstNode {
    AstNode::Block(
        pairs
            .map(|pair| match pair.as_rule() {
                Rule::statement => parse_statement(pair.into_inner()),
                _ => unreachable!(),
            })
            .collect(),
    )
}

/// Parse a statement into an [`AstNode`]
fn parse_statement(pairs: Pairs) -> AstNode {
    let mut pairs = pairs;
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::assign_statement => parse_assignment(pair.into_inner()),
        Rule::expression => parse_expression(pair.into_inner()),
        Rule::return_statement => parse_return(pair.into_inner()),
        Rule::break_statement => AstNode::Break,
        Rule::continue_statement => AstNode::Continue,
        Rule::if_statement => parse_if(pair.into_inner()),
        Rule::for_statement => parse_for_statement(pair.into_inner()),
        Rule::while_statement => parse_while_statement(pair.into_inner()),
        Rule::inf_loop_statement => parse_infinite_loop_statement(pair.into_inner()),
        _ => unreachable!(),
    }
}

/// Parse an expression primary into an [`AstNode`]
fn parse_assignment(pairs: Pairs) -> AstNode {
    let mut pairs = pairs;
    let identifier = pairs.next().unwrap().as_str().to_string();
    let value = pairs.next().unwrap().into_inner();
    AstNode::Assignment {
        identifier,
        value: Box::new(parse_expression(value)),
    }
}

fn parse_return(pairs: Pairs) -> AstNode {
    let mut pairs = pairs;
    match pairs.next() {
        Some(pair) => {
            let value = pair.into_inner();
            AstNode::Return {
                value: Some(Box::new(parse_expression(value))),
            }
        }
        None => AstNode::Return { value: None },
    }
}

fn parse_while_statement(mut pairs: Pairs) -> AstNode {
    let condition = parse_expression(pairs.next().unwrap().into_inner());
    let body = parse_statements(pairs.next().unwrap().into_inner());
    AstNode::While {
        condition: Box::new(condition),
        body: Box::new(body),
    }
}

fn parse_infinite_loop_statement(mut pairs: Pairs) -> AstNode {
    let body = parse_statements(pairs.next().unwrap().into_inner());
    AstNode::Loop {
        body: Box::new(body),
    }
}

fn parse_for_statement(mut pairs: Pairs) -> AstNode {
    let mut initialization = None;
    let mut condition = None;
    let mut increment = None;
    let mut body = None;

    for _ in 0..4 {
        let pair = match pairs.next() {
            Some(pair) => pair,
            None => break,
        };

        match pair.as_rule() {
            Rule::for_init => {
                initialization = Some(Box::new(parse_assignment(pair.into_inner())));
            }
            Rule::for_condition => {
                condition = Some(Box::new(parse_expression(pair.into_inner())));
            }
            Rule::for_increment => {
                increment = Some(Box::new(parse_assignment(pair.into_inner())));
            }
            Rule::statements => {
                body = Some(Box::new(parse_statements(pair.into_inner())));
            }
            _ => unreachable!(),
        };
    }

    let body = body.unwrap();

    AstNode::For {
        initialization,
        condition,
        increment,
        body,
    }
}

/// Get or create a Pratt parser to use for parsing expressions with correct operator precedence.
///
/// The expression parser is a singleton, so it will only be created once.
fn expression_parser() -> &'static PrattParser<Rule> {
    EXPRESSION_PARSER.get_or_init(|| {
        // Infix operators are listed in order of increasing precedence
        PrattParser::new()
            .op(Op::infix(Rule::op_and, Assoc::Left) | Op::infix(Rule::op_or, Assoc::Left))
            .op(Op::infix(Rule::op_eq, Assoc::Left)
                | Op::infix(Rule::op_neq, Assoc::Left)
                | Op::infix(Rule::op_lt, Assoc::Left)
                | Op::infix(Rule::op_lte, Assoc::Left)
                | Op::infix(Rule::op_gt, Assoc::Left)
                | Op::infix(Rule::op_gte, Assoc::Left))
            .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
            .op(Op::infix(Rule::mul, Assoc::Left)
                | Op::infix(Rule::div, Assoc::Left)
                | Op::infix(Rule::rem, Assoc::Left))
            .op(Op::prefix(Rule::neg) | Op::prefix(Rule::not))
    })
}

/// Parse an expression into an [`AstNode`]
fn parse_expression(pairs: Pairs) -> AstNode {
    expression_parser()
        .map_primary(parse_expression_primary)
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::neg => AstNode::UnaryOperation {
                kind: UnaryOperationKind::Negate,
                operand: Box::new(rhs),
            },
            Rule::not => AstNode::UnaryOperation {
                kind: UnaryOperationKind::Not,
                operand: Box::new(rhs),
            },
            _ => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| {
            let kind = match op.as_rule() {
                Rule::add => BinaryOperationKind::Add,
                Rule::sub => BinaryOperationKind::Subtract,
                Rule::mul => BinaryOperationKind::Multiply,
                Rule::div => BinaryOperationKind::Divide,
                Rule::rem => BinaryOperationKind::Remainder,
                Rule::op_eq => BinaryOperationKind::Equal,
                Rule::op_neq => BinaryOperationKind::NotEqual,
                Rule::op_lt => BinaryOperationKind::LessThan,
                Rule::op_lte => BinaryOperationKind::LessThanOrEqual,
                Rule::op_gt => BinaryOperationKind::GreaterThan,
                Rule::op_gte => BinaryOperationKind::GreaterThanOrEqual,
                Rule::op_and => BinaryOperationKind::And,
                Rule::op_or => BinaryOperationKind::Or,
                _ => unreachable!(),
            };

            AstNode::BinaryOperation {
                kind,
                left: Box::new(lhs),
                right: Box::new(rhs),
            }
        })
        .parse(pairs)
}

/// Parse an expression primary (i.e. atom) into an [`AstNode`].
///
/// This function is theoretically infallible for a successfully parsed expression primary.
fn parse_expression_primary(pair: Pair) -> AstNode {
    match pair.as_rule() {
        Rule::identifier => AstNode::Identifier(pair.as_str().to_string()),
        Rule::dec_literal
        | Rule::hex_literal
        | Rule::bin_literal
        | Rule::float_literal
        | Rule::scinot_literal => AstNode::NumberLiteral(parse_number_literal(pair)),
        Rule::nil_literal => AstNode::NilLiteral,
        Rule::string_literal => AstNode::StringLiteral(parse_string_literal(pair)),
        Rule::bool_literal => AstNode::BooleanLiteral(parse_boolean_literal(pair)),
        Rule::expression => parse_expression(pair.into_inner()),
        Rule::function_call => parse_function_call(pair.into_inner()),
        Rule::function_def => parse_function_def(pair.into_inner()),
        _ => unreachable!(),
    }
}

/// Parse a function call into an [`AstNode`].
fn parse_function_call(pairs: Pairs) -> AstNode {
    let mut pairs = pairs;
    let identifier = pairs.next().unwrap().as_str().to_string();
    AstNode::FunctionCall {
        identifier,
        args: pairs
            .map(|pair| parse_expression(pair.into_inner()))
            .collect(),
    }
}

fn parse_function_def_arguments(pairs: Pairs) -> Vec<String> {
    pairs.map(|pair| pair.as_str().to_string()).collect()
}

fn parse_function_def(pairs: Pairs) -> AstNode {
    let mut pairs = pairs;
    let args = parse_function_def_arguments(pairs.next().unwrap().into_inner());
    let body = parse_statements(pairs.next().unwrap().into_inner());
    AstNode::FunctionDef {
        args,
        body: Box::new(body),
    }
}

fn parse_if(pairs: Pairs) -> AstNode {
    let mut pairs = pairs;
    let condition = pairs.next().unwrap().into_inner();
    let body = parse_statements(pairs.next().unwrap().into_inner());
    let else_body = match pairs.next() {
        Some(pair) => match pair.as_rule() {
            Rule::elseif_clause => Some(Box::new(parse_if(pair.into_inner()))),
            Rule::else_clause => Some(Box::new(parse_statements(
                pair.into_inner().next().unwrap().into_inner(),
            ))),
            _ => unreachable!(),
        },
        None => None,
    };
    AstNode::If {
        condition: Box::new(parse_expression(condition)),
        body: Box::new(body),
        else_body,
    }
}

/// Parse a number literal into a [`Number`].
fn parse_number_literal(pair: Pair) -> Number {
    match pair.as_rule() {
        Rule::dec_literal | Rule::hex_literal | Rule::bin_literal => {
            Number::Integer(pair.as_str().parse().unwrap())
        }
        Rule::float_literal | Rule::scinot_literal => Number::Float(pair.as_str().parse().unwrap()),
        _ => unreachable!(),
    }
}

/// Parse a string literal into a `String`.
fn parse_string_literal(pair: Pair) -> String {
    let token = pair.as_str();
    unescape::unescape(&token[1..token.len() - 1]).unwrap()
}

/// Parse a boolean literal into a bool.
fn parse_boolean_literal(pair: Pair) -> bool {
    pair.as_str() == "true"
}
