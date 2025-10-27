//! This module contains the pest parser for SurrealQL, the parsing outputs
//! token and nodes that are then

use pest::Parser;
use pest_derive::Parser;
use rowan::GreenNodeBuilder;

pub mod tests;

#[allow(dead_code)]
#[derive(Parser)]
#[grammar = "syn/surrealql.pest"]
pub struct SurrealQLParser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    // ===== Tokens =====
    // Keywords
    KW_USE,
    KW_NAMESPACE,
    KW_DATABASE,
    // Others
    IDENTIFIER,
    // Trivia
    WHITESPACE,
    SEMICOLON,
    NEWLINE,
    // ===== End of Tokens =====

    // ===== Nodes =====
    use_stmt,
    use_ns_stmt,
    use_db_stmt,
    use_ns_db_stmt,
    statement,

    // ===== End of Nodes =====
    ERROR,
    ROOT,
}

pub type SyntaxNode = rowan::SyntaxNode<Lang>;
pub type SyntaxToken = rowan::SyntaxToken<Lang>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}
impl From<rowan::SyntaxKind> for SyntaxKind {
    fn from(raw: rowan::SyntaxKind) -> Self {
        assert!(raw.0 <= SyntaxKind::ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {}

impl rowan::Language for Lang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        raw.into()
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntaxKindType {
    Node(SyntaxKind),
    Token(SyntaxKind),
    Error(SyntaxKind),
    Ignore,
}

impl From<Rule> for SyntaxKindType {
    fn from(rule: Rule) -> Self {
        match rule {
            // Tokens
            Rule::KW_USE => SyntaxKindType::Token(SyntaxKind::KW_USE),
            Rule::KW_NAMESPACE => SyntaxKindType::Token(SyntaxKind::KW_NAMESPACE),
            Rule::KW_DATABASE => SyntaxKindType::Token(SyntaxKind::KW_DATABASE),
            Rule::identifier => SyntaxKindType::Token(SyntaxKind::IDENTIFIER),
            // Nodes
            Rule::use_stmt => SyntaxKindType::Node(SyntaxKind::use_stmt),
            Rule::use_ns_stmt => SyntaxKindType::Node(SyntaxKind::use_ns_stmt),
            Rule::use_db_stmt => SyntaxKindType::Node(SyntaxKind::use_db_stmt),
            Rule::use_ns_db_stmt => SyntaxKindType::Node(SyntaxKind::use_ns_db_stmt),
            Rule::statement => SyntaxKindType::Node(SyntaxKind::statement),
            // Special
            Rule::EOI => SyntaxKindType::Ignore,
            Rule::SEMICOLON => SyntaxKindType::Token(SyntaxKind::SEMICOLON),
            Rule::NEWLINE => SyntaxKindType::Token(SyntaxKind::NEWLINE),
            Rule::WHITESPACE => SyntaxKindType::Token(SyntaxKind::WHITESPACE),
            Rule::file => SyntaxKindType::Ignore,
            _ => SyntaxKindType::Error(SyntaxKind::ERROR),
        }
    }
}

//==============================================================

impl SurrealQLParser {
    pub fn parse_to_syntax_node(input: &str) -> Option<SyntaxNode> {
        let mut builder = GreenNodeBuilder::new();
        builder.start_node(SyntaxKind::ROOT.into());
        let pairs = SurrealQLParser::parse(Rule::file, input).ok()?;
        let mut token_start_stack: Vec<usize> = Vec::new();
        for token in pairs.tokens() {
            match token {
                pest::Token::Start { rule, pos } => {
                    let syntax_kind_type: SyntaxKindType = rule.into();
                    println!("Start: {:?} @ {:?} ({:?})", rule, pos, syntax_kind_type);
                    match syntax_kind_type {
                        SyntaxKindType::Node(syntax_kind) => {
                            builder.start_node(syntax_kind.into());
                        }
                        SyntaxKindType::Token(syntax_kind) => {
                            token_start_stack.push(pos.pos());
                        }
                        SyntaxKindType::Error(syntax_kind) => {
                            token_start_stack.push(pos.pos());
                        }
                        SyntaxKindType::Ignore => {}
                    }
                }
                pest::Token::End { rule, pos } => {
                    let syntax_kind_type: SyntaxKindType = rule.into();
                    println!("End: {:?} @ {:?} ({:?})", rule, pos, syntax_kind_type);
                    match syntax_kind_type {
                        SyntaxKindType::Node(syntax_kind) => {
                            builder.finish_node();
                        }
                        SyntaxKindType::Token(syntax_kind) => {
                            let token_str = &input[token_start_stack.pop().unwrap()..pos.pos()];
                            builder.token(syntax_kind.into(), token_str);
                        }
                        SyntaxKindType::Error(syntax_kind) => {
                            let token_str = &input[token_start_stack.pop().unwrap()..pos.pos()];
                            builder.token(syntax_kind.into(), token_str);
                        }
                        SyntaxKindType::Ignore => {}
                    }
                }
            }
        }
        builder.finish_node();
        Some(SyntaxNode::new_root(builder.finish()))
    }
}
