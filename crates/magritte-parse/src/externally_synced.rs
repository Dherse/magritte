use std::borrow::Cow;

use magritte_types::ExternallySynced;
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

pub(crate) fn externally_synced_new(value: Option<String>) -> ExternallySynced<'static> {
    if let Some(input) = value {
        all_consuming(externally_synced)(&input).unwrap().1.as_static()
    } else {
        ExternallySynced::No
    }
}

fn externally_synced(input: &'_ str) -> IResult<&'_ str, ExternallySynced<'_>> {
    alt((yes, no, multi, single))(input)
}

fn single(input: &'_ str) -> IResult<&'_ str, ExternallySynced<'_>> {
    alt((for_each, all, resolve, var))(input)
}

fn all(input: &'_ str) -> IResult<&'_ str, ExternallySynced<'_>> {
    map(terminated(delimited(space0, var, space0), tag("[]")), |a| {
        ExternallySynced::All(box a)
    })(input)
}

fn var(input: &'_ str) -> IResult<&'_ str, ExternallySynced<'_>> {
    map(delimited(space0, variable_raw, space0), |s| {
        ExternallySynced::Variable(Cow::Borrowed(s))
    })(input)
}

fn multi(input: &'_ str) -> IResult<&'_ str, ExternallySynced<'_>> {
    map(separated_list1(complete::char(','), single), |mut vals| {
        if vals.len() > 1 {
            ExternallySynced::Multiple(vals)
        } else {
            unsafe { vals.pop().unwrap_unchecked() }
        }
    })(input)
}

fn for_each(input: &'_ str) -> IResult<&'_ str, ExternallySynced<'_>> {
    map(
        separated_pair(
            delimited(space0, var, space0),
            tag("[]."),
            delimited(space0, single, space0),
        ),
        |(a, b)| ExternallySynced::ForEach(box a, box b),
    )(input)
}

fn resolve(input: &'_ str) -> IResult<&'_ str, ExternallySynced<'_>> {
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