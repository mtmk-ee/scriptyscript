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
pub struct GrammarParser {}

/// Try to parse a string into an [`AstNode`].
///
/// # Errors
/// Returns a [`pest::error::Error`] if the string cannot be parsed.
pub fn parse(s: impl AsRef<str>) -> Result<AstNode, pest::error::Error<Rule>> {
    let mut pairs = GrammarParser::parse(Rule::script, s.as_ref())?;
    Ok(parse_block(pairs.next().unwrap().into_inner()))
}

/// Parse a block of statements into an [`AstNode`]
fn parse_block(pairs: Pairs) -> AstNode {
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
pub fn parse_statement(pairs: Pairs) -> AstNode {
    let mut pairs = pairs;
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::assign => parse_assignment(pair.into_inner()),
        Rule::expression => parse_expression(pair.into_inner()),
        _ => unreachable!(),
    }
}

/// Parse an expression primary into an [`AstNode`]
pub fn parse_assignment(pairs: Pairs) -> AstNode {
    let mut pairs = pairs;
    let identifier = pairs.next().unwrap().as_str().to_string();
    let value = pairs.next().unwrap().into_inner();
    AstNode::Assignment {
        identifier,
        value: Box::new(parse_expression(value)),
    }
}

/// Get or create a Pratt parser to use for parsing expressions with correct operator precedence.
///
/// The expression parser is a singleton, so it will only be created once.
fn expression_parser() -> &'static PrattParser<Rule> {
    EXPRESSION_PARSER.get_or_init(|| {
        PrattParser::new()
            .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
            .op(Op::infix(Rule::mul, Assoc::Left)
                | Op::infix(Rule::div, Assoc::Left)
                | Op::infix(Rule::rem, Assoc::Left))
            .op(Op::prefix(Rule::neg) | Op::prefix(Rule::not))
    })
}

/// Parse an expression into an [`AstNode`]
pub fn parse_expression(pairs: Pairs) -> AstNode {
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
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => AstNode::BinaryOperation {
                kind: BinaryOperationKind::Add,
                left: Box::new(lhs),
                right: Box::new(rhs),
            },
            Rule::sub => AstNode::BinaryOperation {
                kind: BinaryOperationKind::Subtract,
                left: Box::new(lhs),
                right: Box::new(rhs),
            },
            Rule::mul => AstNode::BinaryOperation {
                kind: BinaryOperationKind::Multiply,
                left: Box::new(lhs),
                right: Box::new(rhs),
            },
            Rule::div => AstNode::BinaryOperation {
                kind: BinaryOperationKind::Divide,
                left: Box::new(lhs),
                right: Box::new(rhs),
            },
            Rule::rem => AstNode::BinaryOperation {
                kind: BinaryOperationKind::Remainder,
                left: Box::new(lhs),
                right: Box::new(rhs),
            },
            _ => unreachable!(),
        })
        .parse(pairs)
}

/// Parse an expression primary (i.e. atom) into an [`AstNode`].
///
/// This function is theoretically infallible for a successfully parsed expression primary.
pub fn parse_expression_primary(pair: Pair) -> AstNode {
    match pair.as_rule() {
        Rule::identifier => AstNode::Identifier(pair.as_str().to_string()),
        Rule::dec_literal
        | Rule::hex_literal
        | Rule::bin_literal
        | Rule::float_literal
        | Rule::scinot_literal => AstNode::NumberLiteral(parse_number_literal(pair)),
        Rule::string_literal => AstNode::StringLiteral(parse_string_literal(pair)),
        Rule::bool_literal => AstNode::BooleanLiteral(parse_boolean_literal(pair)),
        Rule::expression => parse_expression(pair.into_inner()),
        Rule::function_call => parse_function_call(pair.into_inner()),
        _ => unreachable!(),
    }
}

/// Parse a function call into an [`AstNode`].
pub fn parse_function_call(pairs: Pairs) -> AstNode {
    let mut pairs = pairs;
    let identifier = pairs.next().unwrap().as_str().to_string();
    AstNode::FunctionCall {
        identifier,
        args: pairs
            .map(|pair| parse_expression(pair.into_inner()))
            .collect(),
    }
}

/// Parse a number literal into a [`Number`].
pub fn parse_number_literal(pair: Pair) -> Number {
    match pair.as_rule() {
        Rule::dec_literal | Rule::hex_literal | Rule::bin_literal => {
            Number::Integer(pair.as_str().parse().unwrap())
        }
        Rule::float_literal | Rule::scinot_literal => Number::Float(pair.as_str().parse().unwrap()),
        _ => unreachable!(),
    }
}

/// Parse a string literal into a `String`.
pub fn parse_string_literal(pair: Pair) -> String {
    let token = pair.as_str();
    token[1..token.len() - 1].to_string()
}

/// Parse a boolean literal into a bool.
pub fn parse_boolean_literal(pair: Pair) -> bool {
    pair.as_str() == "true"
}
