#[derive(Clone, Debug)]
pub enum Token<'s> {
    Number(f64),
    Identifier(&'s str),
    Operator(Operator),
}

#[derive(Clone, Debug)]
pub enum Operator {
    Plus,
    Minus,
}

fn comment(input: &str) -> nom::IResult<&str, ()> {
    use nom::branch::alt;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator::value;
    use nom::sequence::tuple;
    let mut state = (bytes::tag("/*")(input)?.0, true);
    while state.1 {
        state = alt((
            value(false, bytes::tag("*/")),
            value(true, comment),
            value(true, character::anychar),
        ))(state.0)?;
    }
    Ok((state.0, ()))
}

pub fn token(input: &str) -> nom::IResult<&str, Token> {
    use nom::branch::alt;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator::map;
    use nom::combinator::value;
    use nom::multi::fold_many0;
    use nom::number::complete as number;
    use nom::sequence::pair;
    use nom::sequence::preceded;
    alt((
        map(number::double, |value| Token::Number(value)),
        map(
            bytes::take_while1(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '$')),
            |s| Token::Identifier(s),
        ),
        value(Token::Operator(Operator::Plus), character::char('+')),
        value(Token::Operator(Operator::Minus), character::char('-')),
        // preceded(pair(bytes::tag("//"), bytes::is_not("\n\r")), token),
        preceded(comment, token),
        preceded(character::multispace1, token),
    ))(input)
}
