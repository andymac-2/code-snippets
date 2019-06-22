#![type_length_limit = "2582009"]

use std::collections::HashMap;


use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::streaming::{none_of, one_of},
    combinator::map,
    multi::{fold_many0, many0, separated_list},
    sequence::{delimited, tuple},
    IResult, InputIter, InputTake, Slice,
};
use std::hash;
use std::ops::RangeFrom;
#[derive(Debug, Clone, PartialEq)]
enum Json {
    JSObject(HashMap<String, Json>),
    JSArray(Vec<Json>),
    JSString(String),
    JSNumber(f64),
    JSTrue,
    JSFalse,
    JSNull,
}

fn void_many0<I, P, O, E>(parser: P) -> impl Fn(I) -> IResult<I, (), E>
where
    P: Fn(I) -> IResult<I, O, E>,
    I: Clone + PartialEq,
    E: nom::error::ParseError<I>,
{
    map(many0(parser), |_| ())
}

fn ws<I>(i: I) -> IResult<I, ()>
where
    I: InputIter<Item = char> + Slice<RangeFrom<usize>> + PartialEq + Copy,
{
    void_many0(one_of("\x09\x0A\x0D\x20"))(i)
}

fn token<I, P, O>(parser: P) -> impl Fn(I) -> IResult<I, O>
where
    P: Fn(I) -> IResult<I, O>,
    I: InputIter<Item = char> + Slice<RangeFrom<usize>> + PartialEq + Copy,
{
    let parse = map(tuple((parser, ws)), |(result, _)| result);
    move |stream| parse(stream)
}

fn parse_json_value<I>(input: I) -> IResult<I, Json>
where
    I: InputTake
        + InputIter<Item = char>
        + nom::Compare<&'static str>
        + Slice<RangeFrom<usize>>
        + PartialEq
        + Copy,
{
    let js_true = map(token(tag("true")), |_: I| Json::JSTrue);
    let js_false = map(token(tag("false")), |_: I| Json::JSFalse);
    let js_null = map(token(tag("null")), |_: I| Json::JSNull);

    let character = none_of::<I, _, _>("\"\\" as &str);
    let string = map(
        token(tuple((tag("\""), many0(character), tag("\"")))),
        |(_, vec, _)| vec.into_iter().collect(),
    );
    let js_string = map(&string, Json::JSString);

    let member = map(
        tuple((&string, token(tag(":")), parse_json_value)),
        |(string, _, elem)| (string, elem),
    );
    let js_object = map(
        delimited(
            token(tag("{")),
            separated_list(token(tag(",")), member),
            token(tag("}")),
        ),
        |list| {
            let mut map = HashMap::new();
            for (key, value) in list {
                map.insert(key, value);
            }
            Json::JSObject(map)
        },
    );

    let js_array = map(
        delimited(
            token(tag("[")),
            separated_list(token(tag(",")), parse_json_value),
            token(tag("]")),
        ),
        |list| Json::JSArray(list),
    );

    let value = token(alt((
        js_object, js_array, js_string, js_true, js_false, js_null,
    )));

    value(input)
}

fn parse_json<I>(input: I) -> IResult<I, Json>
where
    I: InputTake
        + InputIter<Item = char>
        + nom::Compare<&'static str>
        + Slice<RangeFrom<usize>>
        + PartialEq
        + Copy,
{
    map(tuple((tag("true"), ws)), |(_, _)| Json::JSTrue)(input)
}

// impl hash::Hash for fn(u32) -> u32 {
//     fn hash<HH: hash::Hasher>(&self, state: &mut HH) {
//         state.write_usize(*self as usize)
//     }
// }

use std::collections::HashSet;

fn id(x: u32) -> u32 {
    x
}
fn inc(x: u32) -> u32 {
    x + 1
}

struct Wrapper<A, Ret>(fn(A) -> Ret);
impl<A, Ret> PartialEq for Wrapper<A, Ret> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

fn main() {
    println!("{}", Wrapper(id) == Wrapper(id));
    println!("{}", Wrapper(id) == Wrapper(inc));

    // let set = HashSet::new();
    // set.insert(id);
}