use combine::{
    many,
    parser::char::{newline, space},
    ParseError, Parser, Stream,
};

pub fn whitespaces<Input>() -> impl Parser<Input, Output = String>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many::<String, _, _>(space().or(newline()))
}
