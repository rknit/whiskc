use once_cell::sync::Lazy;

use crate::{
    ast::{
        location::Located,
        nodes::{expr::*, punctuate::Puntuated},
        parsing::{
            parsers::pratt_parser::{self, PrattParseError, PrattParseResult, PrattParser},
            token::{
                Delimiter, Identifier, Keyword, Literal, LiteralKeyword, Operator, Token, TokenKind,
            },
            Parse, ParseContext, ParseError, ParseResult,
        },
    },
    ty::Type,
};

#[derive(Debug, Clone)]
pub enum ExprParseError {
    UnexpectedToken(TokenKind),
    UnexpectedInfixOperator(TokenKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum BindingPower {
    Zero = 0,
    LogicalAdditive,
    LogicalMultiplicative,
    Comparative,
    Additive,
    Cast,
    Unary,
    Call,
    ArrayIndex,
    Primary,
}

struct ExprHandlers;
impl pratt_parser::Handlers<Expr, BindingPower> for ExprHandlers {
    fn nuds<F>(&self, mut nud: F)
    where
        F: FnMut(TokenKind, pratt_parser::NudHandler<Expr, BindingPower>),
    {
        let primaries = [
            TokenKind::LiteralKeyword(LiteralKeyword::True),
            TokenKind::LiteralKeyword(LiteralKeyword::False),
            TokenKind::Literal(Literal::Int(0)),
            TokenKind::Identifier(Identifier("".into())),
        ];
        for primary in primaries {
            nud(primary, parse_primary_expr);
        }

        let prefix_unary_ops = [Operator::Sub, Operator::Not];
        for op in prefix_unary_ops {
            nud(TokenKind::Operator(op), parse_prefix_unary_expr);
        }

        nud(TokenKind::Delimiter(Delimiter::ParenOpen), parse_group_expr);

        nud(
            TokenKind::Delimiter(Delimiter::BracketOpen),
            parse_array_expr,
        );
    }

    fn leds<F>(&self, mut led: F)
    where
        F: FnMut(TokenKind, BindingPower, pratt_parser::LedHandler<Expr, BindingPower>),
    {
        let bin_ops = [
            (BindingPower::Additive, vec![Operator::Add, Operator::Sub]),
            (BindingPower::LogicalMultiplicative, vec![Operator::And]),
            (BindingPower::LogicalAdditive, vec![Operator::Or]),
            (
                BindingPower::Comparative,
                vec![
                    Operator::Equal,
                    Operator::NotEqual,
                    Operator::Less,
                    Operator::LessEqual,
                    Operator::Greater,
                    Operator::GreaterEqual,
                ],
            ),
        ];
        for (bp, ops) in bin_ops {
            for op in ops {
                led(TokenKind::Operator(op), bp, parse_binary_expr);
            }
        }

        led(
            TokenKind::Delimiter(Delimiter::ParenOpen),
            BindingPower::Call,
            parse_call_expr,
        );

        led(
            TokenKind::Delimiter(Delimiter::BracketOpen),
            BindingPower::ArrayIndex,
            parse_array_index_expr,
        );

        led(
            TokenKind::Keyword(Keyword::As),
            BindingPower::Cast,
            parse_cast_expr,
        );
    }
}

fn parse_group_expr(
    _pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
) -> ParseResult<Expr> {
    let paren_open_tok = match_delimiter!(parser, Delimiter::ParenOpen =>);
    let expr = Expr::parse(parser)?;
    let paren_close_tok = match_delimiter!(parser, Delimiter::ParenClose =>);
    Some(Expr::Grouped(GroupedExpr {
        paren_open_tok,
        expr: Box::new(expr),
        paren_close_tok,
    }))
}

fn parse_primary_expr(
    _pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
) -> ParseResult<Expr> {
    let tok = parser.lexer.next_token();

    Some(match tok.kind {
        TokenKind::Literal(lit) => match lit {
            Literal::Int(v) => Expr::Integer(Located(v, tok.loc)),
        },
        TokenKind::LiteralKeyword(kw) => match kw {
            LiteralKeyword::True => Expr::Bool(Located(true, tok.loc)),
            LiteralKeyword::False => Expr::Bool(Located(false, tok.loc)),
        },
        TokenKind::Identifier(Identifier(ident)) => Expr::Identifier(Located(ident, tok.loc)),
        _ => unimplemented!("{:#?}", tok),
    })
}

fn parse_prefix_unary_expr(
    pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
) -> ParseResult<Expr> {
    let Token {
        kind: TokenKind::Operator(op),
        loc,
    } = parser.lexer.next_token()
    else {
        panic!("non operator token is not supported");
    };

    let expr = Expr::handle_err(pratt_parser.parse(parser, BindingPower::Unary), parser)?;
    Some(Expr::Unary(UnaryExpr {
        op: Located(op, loc),
        expr: Box::new(expr),
    }))
}

fn parse_binary_expr(
    pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
    left: Expr,
    bp: BindingPower,
) -> ParseResult<Expr> {
    let Token {
        kind: TokenKind::Operator(op),
        loc,
    } = parser.lexer.next_token()
    else {
        panic!("non operator token is not supported");
    };

    let right = Expr::handle_err(pratt_parser.parse(parser, bp), parser)?;
    Some(Expr::Binary(BinaryExpr {
        op: Located(op, loc),
        left: Box::new(left),
        right: Box::new(right),
    }))
}

fn parse_call_expr(
    _pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
    left: Expr,
    _bp: BindingPower,
) -> ParseResult<Expr> {
    let paren_open_tok = match_delimiter!(parser, Delimiter::ParenOpen =>);
    let args = Puntuated::parse(parser, Delimiter::Comma, Delimiter::ParenClose, Expr::parse)?;
    let paren_close_tok = match_delimiter!(parser, Delimiter::ParenClose =>);
    Some(Expr::Call(CallExpr {
        callee: Box::new(left),
        paren_open_tok: paren_open_tok.into(),
        args,
        paren_close_tok: paren_close_tok.into(),
    }))
}

fn parse_array_expr(
    _pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
) -> ParseResult<Expr> {
    let bracket_open_tok = match_delimiter!(parser, Delimiter::BracketOpen =>);
    let elements = Puntuated::parse(
        parser,
        Delimiter::Comma,
        Delimiter::BracketClose,
        Expr::parse,
    )?;
    let bracket_close_tok = match_delimiter!(parser, Delimiter::BracketClose =>);
    Some(Expr::Array(ArrayExpr {
        bracket_open_tok: bracket_open_tok.into(),
        elements,
        bracket_close_tok: bracket_close_tok.into(),
    }))
}

fn parse_array_index_expr(
    _pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
    left: Expr,
    _bp: BindingPower,
) -> ParseResult<Expr> {
    let bracket_open_tok = match_delimiter!(parser, Delimiter::BracketOpen =>);
    let index = Expr::parse(parser)?;
    let bracket_close_tok = match_delimiter!(parser, Delimiter::BracketClose =>);
    Some(Expr::ArrayIndex(ArrayIndexExpr {
        expr: Box::new(left),
        bracket_open_tok: bracket_open_tok.into(),
        index: Box::new(index),
        bracket_close_tok: bracket_close_tok.into(),
    }))
}

fn parse_cast_expr(
    _pratt_parser: &PrattParser<Expr, BindingPower>,
    parser: &mut ParseContext,
    left: Expr,
    _bp: BindingPower,
) -> ParseResult<Expr> {
    let as_tok = match_keyword!(parser, Keyword::As =>);
    let ty = Located::<Type>::parse(parser)?;
    Some(Expr::Cast(CastExpr {
        expr: Box::new(left),
        as_tok: as_tok.into(),
        ty,
    }))
}

impl Expr {
    pub fn parse(parser: &mut ParseContext) -> ParseResult<Expr> {
        static EXPR_PARSER: Lazy<PrattParser<Expr, BindingPower>> =
            Lazy::new(|| PrattParser::new(&ExprHandlers));
        Self::handle_err(EXPR_PARSER.parse(parser, BindingPower::Zero), parser)
    }

    fn handle_err(e: PrattParseResult<Expr>, parser: &mut ParseContext) -> ParseResult<Expr> {
        let Err(e) = e else {
            return e.ok();
        };
        match e {
            PrattParseError::NoNudHandlerFound(e) => parser.push_error(Located(
                ParseError::ExprParseError(ExprParseError::UnexpectedToken(e.kind)),
                e.loc,
            )),
            PrattParseError::NoLedHandlerFound(e) => parser.push_error(Located(
                ParseError::ExprParseError(ExprParseError::UnexpectedInfixOperator(e.kind)),
                e.loc,
            )),
            PrattParseError::ParseError => (),
        }
        None
    }
}

impl pratt_parser::BindingPower for BindingPower {
    fn primary() -> Self {
        Self::Primary
    }
    fn zero() -> Self {
        Self::Zero
    }
}