use once_cell::sync::OnceCell;
use pest::{
    pratt_parser::{Assoc, Op, PrattParser},
    Parser,
};
use serde::{Deserialize, Serialize};

type Pair<'a> = pest::iterators::Pair<'a, Rule>;
type Pairs<'a> = pest::iterators::Pairs<'a, Rule>;

static EXPRESSION_PARSER: OnceCell<PrattParser<Rule>> = OnceCell::new();

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser {}

pub fn parse(s: impl AsRef<str>) -> Result<AstNode, pest::error::Error<Rule>> {
    let mut pairs = GrammarParser::parse(Rule::script, s.as_ref())?;
    Ok(parse_block(pairs.next().unwrap().into_inner()))
}

pub fn parse_block(pairs: Pairs) -> AstNode {
    AstNode::Block(
        pairs
            .map(|pair| match pair.as_rule() {
                Rule::statement => parse_statement(pair.into_inner()),
                _ => unreachable!(),
            })
            .collect(),
    )
}

pub fn parse_statement(pairs: Pairs) -> AstNode {
    let mut pairs = pairs;
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::assign => parse_assignment(pair.into_inner()),
        _ => unreachable!(),
    }
}

pub fn parse_assignment(pairs: Pairs) -> AstNode {
    let mut pairs = pairs;
    let identifier = pairs.next().unwrap().as_str().to_string();
    let value = pairs.next().unwrap().into_inner();
    AstNode::Assignment {
        identifier,
        value: Box::new(parse_expression(value)),
    }
}

pub fn expression_parser() -> &'static PrattParser<Rule> {
    EXPRESSION_PARSER.get_or_init(|| {
        PrattParser::new()
            .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::sub, Assoc::Left))
            .op(Op::infix(Rule::mul, Assoc::Left) | Op::infix(Rule::div, Assoc::Left))
            .op(Op::prefix(Rule::neg) | Op::prefix(Rule::not))
    })
}

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
            _ => unreachable!(),
        })
        .parse(pairs)
}

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
        _ => unreachable!(),
    }
}

pub fn parse_number_literal(pair: Pair) -> Number {
    match pair.as_rule() {
        Rule::dec_literal | Rule::hex_literal | Rule::bin_literal => {
            Number::Integer(pair.as_str().parse().unwrap())
        }
        Rule::float_literal | Rule::scinot_literal => Number::Float(pair.as_str().parse().unwrap()),
        _ => unreachable!(),
    }
}

pub fn parse_string_literal(pair: Pair) -> String {
    let token = pair.as_str();
    token[1..token.len() - 1].to_string()
}

pub fn parse_boolean_literal(pair: Pair) -> bool {
    pair.as_str() == "true"
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Identifier(String),
    NumberLiteral(Number),
    StringLiteral(String),
    BooleanLiteral(bool),
    UnaryOperation {
        kind: UnaryOperationKind,
        operand: Box<AstNode>,
    },
    BinaryOperation {
        kind: BinaryOperationKind,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Assignment {
        identifier: String,
        value: Box<AstNode>,
    },
    Block(Vec<AstNode>),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum UnaryOperationKind {
    Negate,
    Not,
}

impl UnaryOperationKind {
    pub fn dunder(&self) -> String {
        match self {
            UnaryOperationKind::Negate => "__neg__",
            UnaryOperationKind::Not => "__not__",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BinaryOperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    And,
    Or,
    Xor,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

impl BinaryOperationKind {
    pub fn dunder(&self) -> String {
        match self {
            BinaryOperationKind::Add => "__add__",
            BinaryOperationKind::Subtract => "__sub__",
            BinaryOperationKind::Multiply => "__mul__",
            BinaryOperationKind::Divide => "__div__",
            BinaryOperationKind::Modulo => "__mod__",
            BinaryOperationKind::Power => "__pow__",
            BinaryOperationKind::And => "__and__",
            BinaryOperationKind::Or => "__or__",
            BinaryOperationKind::Xor => "__xor__",
            BinaryOperationKind::Equal => "__eq__",
            BinaryOperationKind::NotEqual => "__ne__",
            BinaryOperationKind::GreaterThan => "__gt__",
            BinaryOperationKind::GreaterThanOrEqual => "__ge__",
            BinaryOperationKind::LessThan => "__lt__",
            BinaryOperationKind::LessThanOrEqual => "__le__",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Number {
    Integer(i64),
    Float(f64),
}
