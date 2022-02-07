use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, space0},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

use crate::expr::variable_raw;

/// The external synchronization requierments
#[derive(Debug, Clone, PartialEq)]
pub enum ExternallySynced<'a> {
    /// The value must be externally synchronized
    Yes,

    /// The value does not need to be externally synchronized
    No,

    /// Multiple values need to be externally synchronized
    Multiple(Vec<ExternallySynced<'a>>),

    /// A variable (inside of the argument) must be externally synchronized
    Variable(&'a str),

    /// All the values in an array must be synchronized
    All(Box<ExternallySynced<'a>>),

    /// A value obtained by resolving a pointer must be externally synchronized
    Resolve(Box<ExternallySynced<'a>>, Box<ExternallySynced<'a>>),

    /// All of the values obtained by resolving a pointer must be externally synchronized
    ForEach(Box<ExternallySynced<'a>>, Box<ExternallySynced<'a>>),
}

impl<'a> ExternallySynced<'a> {
    /// Parses an optional string into an external synchronization requierment
    pub fn new(value: Option<&'a str>) -> ExternallySynced<'a> {
        if let Some(input) = value {
            all_consuming(externally_synced)(input).unwrap().1
        } else {
            Self::No
        }
    }
}

fn externally_synced<'a>(input: &'a str) -> IResult<&'a str, ExternallySynced<'a>> {
    alt((yes, no, multi, single))(input)
}

fn single<'a>(input: &'a str) -> IResult<&'a str, ExternallySynced<'a>> {
    alt((for_each, all, resolve, var))(input)
}

fn all<'a>(input: &'a str) -> IResult<&'a str, ExternallySynced<'a>> {
    map(terminated(delimited(space0, var, space0), tag("[]")), |a| {
        ExternallySynced::All(box a)
    })(input)
}

fn var<'a>(input: &'a str) -> IResult<&'a str, ExternallySynced<'a>> {
    map(delimited(space0, variable_raw, space0), |s| {
        ExternallySynced::Variable(s)
    })(input)
}

fn multi<'a>(input: &'a str) -> IResult<&'a str, ExternallySynced<'a>> {
    map(separated_list1(complete::char(','), single), |mut vals| {
        if vals.len() > 1 {
            ExternallySynced::Multiple(vals)
        } else {
            unsafe { vals.pop().unwrap_unchecked() }
        }
    })(input)
}

fn for_each<'a>(input: &'a str) -> IResult<&'a str, ExternallySynced<'a>> {
    map(
        separated_pair(
            delimited(space0, var, space0),
            tag("[]."),
            delimited(space0, single, space0),
        ),
        |(a, b)| ExternallySynced::ForEach(box a, box b),
    )(input)
}

fn resolve<'a>(input: &'a str) -> IResult<&'a str, ExternallySynced<'a>> {
    map(
        separated_pair(
            delimited(space0, var, space0),
            alt((tag("->"), tag("-&gt;"))),
            delimited(space0, single, space0),
        ),
        |(a, b)| ExternallySynced::Resolve(box a, box b),
    )(input)
}

fn yes<'a>(input: &'a str) -> IResult<&str, ExternallySynced<'a>> {
    map(delimited(space0, tag("true"), space0), |_| ExternallySynced::Yes)(input)
}

fn no<'a>(input: &'a str) -> IResult<&str, ExternallySynced<'a>> {
    map(delimited(space0, tag("false"), space0), |_| ExternallySynced::No)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(externally_synced("true"), Ok(("", ExternallySynced::Yes)));
        assert_eq!(externally_synced("false"), Ok(("", ExternallySynced::No)));
        assert_eq!(externally_synced("  true "), Ok(("", ExternallySynced::Yes)));
        assert_eq!(externally_synced("    false    "), Ok(("", ExternallySynced::No)));
    }

    #[test]
    fn resolve_test() {
        assert_eq!(
            externally_synced("this->that"),
            Ok((
                "",
                ExternallySynced::Resolve(
                    box ExternallySynced::Variable("this"),
                    box ExternallySynced::Variable("that")
                )
            ))
        );

        assert_eq!(
            externally_synced("this  ->    that"),
            Ok((
                "",
                ExternallySynced::Resolve(
                    box ExternallySynced::Variable("this"),
                    box ExternallySynced::Variable("that")
                )
            ))
        );

        assert_eq!(
            externally_synced("thisValue-&gt;that_other_value"),
            Ok((
                "",
                ExternallySynced::Resolve(
                    box ExternallySynced::Variable("thisValue"),
                    box ExternallySynced::Variable("that_other_value")
                )
            ))
        );
    }

    #[test]
    fn foreach_test() {
        assert_eq!(
            externally_synced("this[].that"),
            Ok((
                "",
                ExternallySynced::ForEach(
                    box ExternallySynced::Variable("this"),
                    box ExternallySynced::Variable("that")
                )
            ))
        );

        assert_eq!(
            externally_synced("this[].that[].other"),
            Ok((
                "",
                ExternallySynced::ForEach(
                    box ExternallySynced::Variable("this"),
                    box ExternallySynced::ForEach(
                        box ExternallySynced::Variable("that"),
                        box ExternallySynced::Variable("other")
                    )
                )
            ))
        );

        assert_eq!(
            externally_synced("this[].that[].other[].last"),
            Ok((
                "",
                ExternallySynced::ForEach(
                    box ExternallySynced::Variable("this"),
                    box ExternallySynced::ForEach(
                        box ExternallySynced::Variable("that"),
                        box ExternallySynced::ForEach(
                            box ExternallySynced::Variable("other"),
                            box ExternallySynced::Variable("last"),
                        )
                    )
                )
            ))
        );
    }

    #[test]
    fn multi_test() {
        assert_eq!(
            externally_synced("first, this[].that"),
            Ok((
                "",
                ExternallySynced::Multiple(vec![
                    ExternallySynced::Variable("first"),
                    ExternallySynced::ForEach(
                        box ExternallySynced::Variable("this"),
                        box ExternallySynced::Variable("that")
                    ),
                ])
            ))
        );

        assert_eq!(
            externally_synced("first,this[].that,this[].other->value"),
            Ok((
                "",
                ExternallySynced::Multiple(vec![
                    ExternallySynced::Variable("first"),
                    ExternallySynced::ForEach(
                        box ExternallySynced::Variable("this"),
                        box ExternallySynced::Variable("that")
                    ),
                    ExternallySynced::ForEach(
                        box ExternallySynced::Variable("this"),
                        box ExternallySynced::Resolve(
                            box ExternallySynced::Variable("other"),
                            box ExternallySynced::Variable("value")
                        )
                    ),
                ])
            ))
        );
    }
}
