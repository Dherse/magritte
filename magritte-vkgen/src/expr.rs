//! C expression parsing

use std::{borrow::Cow, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, alphanumeric1, one_of, space0},
    combinator::{all_consuming, map, map_res, opt, recognize},
    error::{ErrorKind, ParseError},
    multi::{fold_many0, many0, many1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult, InputTakeAtPosition,
};

use smallvec::SmallVec;
use string::parse_string;

pub mod string;

/// A parsed Vulkan C-like expression
#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    /// A literal string
    String(String),

    /// Resolve a variable
    Variable(Cow<'a, str>),

    /// Resolve a constant
    Constant(Cow<'a, str>),

    /// Constant integer value
    ConstantInt(i64),

    /// Constant float value
    ConstantFloat(f32),

    /// Variable resolving (i.e `a->b`)
    Resolve(Box<Expr<'a>>, Box<Expr<'a>>),

    /// Divide operation
    Divide(Box<Expr<'a>>, Box<Expr<'a>>),

    /// Multiply operation
    Multiply(Box<Expr<'a>>, Box<Expr<'a>>),

    /// Add operation
    Add(Box<Expr<'a>>, Box<Expr<'a>>),

    /// Subtract operatin
    Subtract(Box<Expr<'a>>, Box<Expr<'a>>),

    /// Bitwise not-ing
    BitwiseNot(Box<Expr<'a>>),

    /// Negative of a value
    Neg(Box<Expr<'a>>),
}

impl<'a> Expr<'a> {
    /// Parses an expression from an input
    pub fn new(input: &'a str) -> Self {
        all_consuming(parse_expr)(input).unwrap().1
    }

    /// Whether the expression contains a specific variable
    pub fn contains_variable(&self, var: &str) -> bool {
        match self {
            Expr::Variable(a) => a == var,
            Expr::Resolve(a, b)
            | Expr::Divide(a, b)
            | Expr::Multiply(a, b)
            | Expr::Add(a, b)
            | Expr::Subtract(a, b) => a.contains_variable(var) || b.contains_variable(var),
            Expr::BitwiseNot(a) => a.contains_variable(var),
            _ => false,
        }
    }

    /// Computes the value, panics otherwise
    pub fn compute(&self) -> i64 {
        match self {
            Expr::ConstantInt(a) => *a,
            Expr::Divide(a, b) => a.compute() / b.compute(),
            Expr::Multiply(a, b) => a.compute() * b.compute(),
            Expr::Add(a, b) => a.compute() + b.compute(),
            Expr::Subtract(a, b) => a.compute() - b.compute(),
            Expr::BitwiseNot(a) => !a.compute(),

            _ => unreachable!("Expression cannot be computed: {:?}", self),
        }
    }

    /// List the **variables** (not constants) used in this expression
    pub fn variables(&self) -> SmallVec<[Cow<'a, str>; 1]> {
        let mut out = SmallVec::default();

        self.variables_internal(&mut out);

        out
    }

    fn variables_internal(&self, out: &mut SmallVec<[Cow<'a, str>; 1]>) {
        match self {
            Expr::Variable(var) => out.push(var.clone()),
            Expr::Resolve(var, _) => var.variables_internal(out),
            Expr::Divide(a, b) | Expr::Multiply(a, b) | Expr::Add(a, b) | Expr::Subtract(a, b) => {
                a.variables_internal(out);
                b.variables_internal(out);
            },
            Expr::BitwiseNot(var) | Expr::Neg(var) => var.variables_internal(out),
            _ => (),
        }
    }

    /// List the **constants** (not variables) used in this expression
    pub fn constants(&self) -> SmallVec<[Cow<'a, str>; 1]> {
        let mut out = SmallVec::default();

        self.constants_internal(&mut out);

        out
    }

    fn constants_internal(&self, out: &mut SmallVec<[Cow<'a, str>; 1]>) {
        match self {
            Expr::Constant(var) => out.push(var.clone()),
            Expr::Resolve(var, _) => var.constants_internal(out),
            Expr::Divide(a, b) | Expr::Multiply(a, b) | Expr::Add(a, b) | Expr::Subtract(a, b) => {
                a.constants_internal(out);
                b.constants_internal(out);
            },
            Expr::BitwiseNot(var) | Expr::Neg(var) => var.constants_internal(out),
            _ => (),
        }
    }
}

#[doc(hidden)]
pub fn parse_expr<'a>(input: &'a str) -> IResult<&'a str, Expr<'a>> {
    let a = delimited(space0, expr, space0)(input);
    let b = delimited(space0, term, space0)(input);

    match (a, b) {
        (Ok((rest1, expr1)), Ok((rest2, expr2))) => {
            let (rest, expr) = if rest1.len() <= rest2.len() {
                (rest1, expr1)
            } else {
                (rest2, expr2)
            };

            let expr_other = expr.clone();
            if let Ok((rest, expr)) = fold_many0(
                pair(
                    alt((
                        complete::char('+'),
                        complete::char('-'),
                        complete::char('*'),
                        complete::char('/'),
                    )),
                    term,
                ),
                move || expr_other.clone(),
                |acc, (op, val): (char, Expr<'a>)| {
                    if op == '+' {
                        Expr::Add(box acc, box val)
                    } else if op == '-' {
                        Expr::Subtract(box acc, box val)
                    } else if op == '*' {
                        Expr::Multiply(box acc, box val)
                    } else {
                        Expr::Divide(box acc, box val)
                    }
                },
            )(rest)
            {
                Ok((rest, expr))
            } else {
                Ok((rest, expr))
            }
        },
        (Ok((rest, expr)), Err(_)) | (Err(_), Ok((rest, expr))) => Ok((rest, expr)),
        (Err(e), _) => Err(e),
    }
}

fn factor(input: &'_ str) -> IResult<&'_ str, Expr<'_>> {
    delimited(
        space0,
        alt((
            map(parse_string, Expr::String),
            bitwise_not,
            parenthesized,
            literal_hexadecimal,
            literal_binary,
            resolve_ptr,
            resolve_value,
            constant,
            variable,
            literal_float,
            literal_integer,
        )),
        space0,
    )(input)
}

fn term<'a>(input: &'a str) -> IResult<&'a str, Expr<'a>> {
    let (i, init) = factor(input)?;

    fold_many0(
        pair(alt((complete::char('*'), complete::char('/'))), factor),
        move || init.clone(),
        |acc, (op, val): (char, Expr<'a>)| {
            if op == '*' {
                Expr::Multiply(box acc, box val)
            } else {
                Expr::Divide(box acc, box val)
            }
        },
    )(i)
}

fn expr<'a>(input: &'a str) -> IResult<&'a str, Expr<'a>> {
    let (i, init) = factor(input)?;

    fold_many0(
        pair(alt((complete::char('+'), complete::char('-'))), term),
        move || init.clone(),
        |acc, (op, val): (char, Expr<'a>)| {
            if op == '+' {
                Expr::Add(box acc, box val)
            } else {
                Expr::Subtract(box acc, box val)
            }
        },
    )(i)
}

/// Matches a constant (i.e `VK_UUID_SIZE`)
fn constant<'a>(input: &'a str) -> IResult<&'a str, Expr<'a>> {
    let rust_like_parser = recognize::<&'a str, _, _, _>(pair(
        alt((alpha1_caps, tag("_"))),
        many0(alt((alphanumeric1_caps, tag("_")))),
    ));

    let vulkan_like_parser = recognize::<&'a str, _, _, _>(pair(
        pair(many1(alphanumeric1_caps), tag("_")),
        many0(alt((alphanumeric1_caps, tag("_")))),
    ));

    map(alt((rust_like_parser, vulkan_like_parser)), |s| {
        Expr::Constant(Cow::Borrowed(s))
    })(input)
}

fn float_info(input: &str) -> IResult<&str, &str> {
    recognize(one_of("DF"))(input)
}

fn length_info(input: &str) -> IResult<&str, &str> {
    recognize(pair(complete::char('U'), many0(complete::char('L'))))(input)
}

/// Matches a variable (i.e `value`)
#[doc(hidden)]
pub fn variable(input: &'_ str) -> IResult<&'_ str, Expr<'_>> {
    map(variable_raw, |s| Expr::Variable(Cow::Borrowed(s)))(input)
}

/// Matches a variable (i.e `value`)
#[doc(hidden)]
pub fn variable_raw(input: &'_ str) -> IResult<&'_ str, &'_ str> {
    recognize(pair(alt((alpha1, tag("_"))), many0(alt((alphanumeric1, tag("_"))))))(input)
}

/// Matches a resolve (i.e `this->value`)
fn bitwise_not(input: &'_ str) -> IResult<&'_ str, Expr<'_>> {
    map(pair(complete::char('~'), parse_expr), |(_, a)| Expr::BitwiseNot(box a))(input)
}

/// Matches a resolve (i.e `this->value`)
fn resolve_ptr(input: &'_ str) -> IResult<&'_ str, Expr<'_>> {
    map(separated_pair(variable, tag("->"), variable), |(a, b)| {
        Expr::Resolve(box a, box b)
    })(input)
}

/// Matches a resolve (i.e `this->value`)
fn resolve_value(input: &'_ str) -> IResult<&'_ str, Expr<'_>> {
    map(separated_pair(variable, tag("."), variable), |(a, b)| {
        Expr::Resolve(box a, box b)
    })(input)
}

fn parenthesized(input: &'_ str) -> IResult<&'_ str, Expr<'_>> {
    delimited(
        space0,
        delimited(complete::char('('), parse_expr, complete::char(')')),
        space0,
    )(input)
}

/// Matches a literal integer (i.e `4`)
fn literal_integer(input: &'_ str) -> IResult<&'_ str, Expr<'_>> {
    map(
        map_res(
            terminated(
                recognize(pair(
                    many0(complete::char('-')),
                    many1(terminated(one_of("0123456789"), many0(complete::char('_')))),
                )),
                many0(length_info),
            ),
            FromStr::from_str,
        ),
        Expr::ConstantInt,
    )(input)
}

fn literal_hexadecimal(input: &'_ str) -> IResult<&'_ str, Expr<'_>> {
    map(
        map_res(
            preceded(
                alt((tag("0x"), tag("0X"))),
                recognize(many1(terminated(
                    one_of("0123456789abcdefABCDEF"),
                    many0(complete::char('_')),
                ))),
            ),
            |out: &str| i64::from_str_radix(&str::replace(out, "_", ""), 16),
        ),
        Expr::ConstantInt,
    )(input)
}

fn literal_binary(input: &'_ str) -> IResult<&'_ str, Expr<'_>> {
    map(
        map_res(
            preceded(
                alt((tag("0b"), tag("0B"))),
                recognize(many1(terminated(one_of("01"), many0(complete::char('_'))))),
            ),
            |out: &str| i64::from_str_radix(&str::replace(out, "_", ""), 16),
        ),
        Expr::ConstantInt,
    )(input)
}

/// Matches a literal float (i.e `32.0`)
fn literal_float(input: &'_ str) -> IResult<&'_ str, Expr<'_>> {
    map(
        map_res(
            terminated(
                alt((
                    // Case one: .42
                    recognize(pair(
                        many0(complete::char('-')),
                        tuple((
                            complete::char('.'),
                            decimal,
                            opt(tuple((one_of("eE"), opt(one_of("+-")), decimal))),
                        )),
                    )), // Case two: 42e42 and 42.42e42
                    recognize(pair(
                        many0(complete::char('-')),
                        tuple((
                            decimal,
                            opt(preceded(complete::char('.'), decimal)),
                            one_of("eE"),
                            opt(one_of("+-")),
                            decimal,
                        )),
                    )), // Case three: 42. and 42.42
                    recognize(pair(
                        many0(complete::char('-')),
                        tuple((decimal, complete::char('.'), opt(decimal))),
                    )),
                )),
                many0(float_info),
            ),
            FromStr::from_str,
        ),
        Expr::ConstantFloat,
    )(input)
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(complete::char('_')))))(input)
}

#[allow(clippy::needless_pass_by_value)]
fn alpha1_caps<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition<Item = char>,
{
    input.split_at_position1_complete(|item| !(item.is_alphabetic() && item.is_uppercase()), ErrorKind::Alpha)
}

#[allow(clippy::needless_pass_by_value)]
fn alphanumeric1_caps<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition<Item = char>,
{
    input.split_at_position1_complete(
        |item| !(item.is_numeric() || (item.is_alphabetic() && item.is_ascii_uppercase())),
        ErrorKind::AlphaNumeric,
    )
}

#[doc(hidden)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_test() {
        assert_eq!(parse_expr("3"), Ok(("", Expr::ConstantInt(3))));
        assert_eq!(parse_expr(" 12"), Ok(("", Expr::ConstantInt(12))));
        assert_eq!(parse_expr("537  "), Ok(("", Expr::ConstantInt(537))));
        assert_eq!(parse_expr("  24   "), Ok(("", Expr::ConstantInt(24))));
        assert_eq!(parse_expr("  -24   "), Ok(("", Expr::ConstantInt(-24))));
        assert_eq!(parse_expr("-668   "), Ok(("", Expr::ConstantInt(-668))));

        assert_eq!(parse_expr("  24U   "), Ok(("", Expr::ConstantInt(24))));
        assert_eq!(parse_expr("  24UL   "), Ok(("", Expr::ConstantInt(24))));
        assert_eq!(parse_expr("  24ULL   "), Ok(("", Expr::ConstantInt(24))));

        assert_eq!(parse_expr("3.2"), Ok(("", Expr::ConstantFloat(3.2))));
        assert_eq!(parse_expr(" 12e12"), Ok(("", Expr::ConstantFloat(12e12))));
        assert_eq!(parse_expr("-537.345  "), Ok(("", Expr::ConstantFloat(-537.345))));
        assert_eq!(
            parse_expr("  1.000000001   "),
            Ok(("", Expr::ConstantFloat(1.000000001)))
        );
        assert_eq!(parse_expr("  -2e-12   "), Ok(("", Expr::ConstantFloat(-2e-12))));
        assert_eq!(parse_expr("   64.3  "), Ok(("", Expr::ConstantFloat(64.3))));
    }

    #[test]
    fn variable_test() {
        assert_eq!(parse_expr("this"), Ok(("", Expr::Variable(Cow::Borrowed("this")))));

        assert_eq!(parse_expr("that"), Ok(("", Expr::Variable(Cow::Borrowed("that")))));

        assert_eq!(
            parse_expr("thisLength   "),
            Ok(("", Expr::Variable(Cow::Borrowed("thisLength"))))
        );

        assert_eq!(
            parse_expr("  thatThatThat3"),
            Ok(("", Expr::Variable(Cow::Borrowed("thatThatThat3"))))
        );

        assert_eq!(
            parse_expr("a_variable_with_a_very_long_name"),
            Ok(("", Expr::Variable(Cow::Borrowed("a_variable_with_a_very_long_name"))))
        );

        assert_eq!(
            parse_expr("    this  "),
            Ok(("", Expr::Variable(Cow::Borrowed("this"))))
        );

        assert_eq!(
            parse_expr("VK_UUID_SIZE"),
            Ok(("", Expr::Constant(Cow::Borrowed("VK_UUID_SIZE"))))
        );

        assert_eq!(
            parse_expr(" A_LONG_CONSTANT"),
            Ok(("", Expr::Constant(Cow::Borrowed("A_LONG_CONSTANT"))))
        );

        assert_eq!(
            parse_expr("  A_LONGER_CONSTANT   "),
            Ok(("", Expr::Constant(Cow::Borrowed("A_LONGER_CONSTANT"))))
        );

        assert_eq!(
            parse_expr("32_SURFACE_CREATE_INFO_KHR"),
            Ok(("", Expr::Constant(Cow::Borrowed("32_SURFACE_CREATE_INFO_KHR"))))
        );
    }

    #[test]
    fn resolve() {
        assert_eq!(
            parse_expr("this->that"),
            Ok((
                "",
                Expr::Resolve(
                    box Expr::Variable(Cow::Borrowed("this")),
                    box Expr::Variable(Cow::Borrowed("that"))
                )
            ))
        );

        assert_eq!(
            parse_expr("that->vkThat"),
            Ok((
                "",
                Expr::Resolve(
                    box Expr::Variable(Cow::Borrowed("that")),
                    box Expr::Variable(Cow::Borrowed("vkThat"))
                )
            ))
        );

        assert_eq!(
            parse_expr("that.vkThat"),
            Ok((
                "",
                Expr::Resolve(
                    box Expr::Variable(Cow::Borrowed("that")),
                    box Expr::Variable(Cow::Borrowed("vkThat"))
                )
            ))
        );
    }

    #[test]
    fn expr_test() {
        assert_eq!(
            parse_expr(" 1 +  2 "),
            Ok(("", Expr::Add(box Expr::ConstantInt(1), box Expr::ConstantInt(2))))
        );

        assert_eq!(
            parse_expr(" 12 + 6 - 4+  3"),
            Ok((
                "",
                Expr::Add(
                    box Expr::Subtract(
                        box Expr::Add(box Expr::ConstantInt(12), box Expr::ConstantInt(6)),
                        box Expr::ConstantInt(4)
                    ),
                    box Expr::ConstantInt(3)
                )
            ))
        );

        assert_eq!(
            parse_expr(" 1 + 2*3 + 4"),
            Ok((
                "",
                Expr::Add(
                    box Expr::Add(
                        box Expr::ConstantInt(1),
                        box Expr::Multiply(box Expr::ConstantInt(2), box Expr::ConstantInt(3))
                    ),
                    box Expr::ConstantInt(4)
                )
            ))
        );

        assert_eq!(
            parse_expr("this->that * 64"),
            Ok((
                "",
                Expr::Multiply(
                    box Expr::Resolve(
                        box Expr::Variable(Cow::Borrowed("this")),
                        box Expr::Variable(Cow::Borrowed("that"))
                    ),
                    box Expr::ConstantInt(64)
                )
            ))
        );
    }

    #[test]
    fn paren_test() {
        assert_eq!(expr(" (  2 )"), Ok(("", Expr::ConstantInt(2))));
        assert_eq!(
            parse_expr(" 2* (  3.0 + 4 ) "),
            Ok((
                "",
                Expr::Multiply(
                    box Expr::ConstantInt(2),
                    box Expr::Add(box Expr::ConstantFloat(3.0), box Expr::ConstantInt(4))
                )
            ))
        );
        assert_eq!(
            parse_expr("  2*2 / ( 5 - 1) + 3"),
            Ok((
                "",
                Expr::Add(
                    box Expr::Divide(
                        box Expr::Multiply(box Expr::ConstantInt(2), box Expr::ConstantInt(2)),
                        box Expr::Subtract(box Expr::ConstantInt(5), box Expr::ConstantInt(1)),
                    ),
                    box Expr::ConstantInt(3)
                )
            ))
        );
    }

    #[test]
    fn bitwise_test() {
        assert_eq!(expr(" (  ~2 )"), Ok(("", Expr::BitwiseNot(box Expr::ConstantInt(2)))));
        assert_eq!(
            expr(" (  ~0ULL )"),
            Ok(("", Expr::BitwiseNot(box Expr::ConstantInt(0))))
        );
    }
}
