#[derive(Clone, Debug)]
pub enum Expr<'s> {
    Id(&'s str),
    Num(f64),
    Un(UnOp, Box<Expr<'s>>),
    Bin(BinOp, Box<Expr<'s>>, Box<Expr<'s>>),
    Inv(Box<Expr<'s>>, Box<Expr<'s>>),
    Map(Box<Expr<'s>>, Option<Box<Expr<'s>>>, Box<Expr<'s>>),
}

#[derive(Clone, Debug)]
pub enum UnOp {
    Mul,
    Div,
    Add,
    Sub,
    Not,
}

#[derive(Clone, Debug)]
pub enum BinOp {
    Mul,
    Div,
    Add,
    Sub,
    Less,
    Greater,
    Equal,
    NotEqual,
    And,
    Or,
    Subst,
    Comma,
}

macro_rules! def_parser {
    ($pub:vis, $name:ident, $prev:ident, $ops:expr) => {
        $pub fn $name(input: &str) -> nom::IResult<&str, Expr> {
            let (input, head) = $prev(input)?;
            nom::multi::fold_many0(
                nom::sequence::tuple(($ops, $prev)),
                head,
                |prev, (bin_op, factor)| Expr::Bin(bin_op, prev.into(), factor.into()),
            )(input)
        }
    };
    ($pub:vis, $name:ident, $prev:ident, $from:expr => $to:expr) => {
        def_parser!($pub, $name, $prev, nom::combinator::map(nom::bytes::complete::tag($from), |_| $to));
    };
    ($pub:vis, $name:ident, $prev:ident, $($from:expr => $to:expr),+) => {
        def_parser!($pub, $name, $prev, nom::branch::alt(($(nom::combinator::map(nom::bytes::complete::tag($from), |_| $to)),+)));
    }
}

def_parser!(, parse_expr1, parse_factor, "*" => BinOp::Mul, "/" => BinOp::Div);
def_parser!(, parse_expr2, parse_expr1, "+" => BinOp::Add, "-" => BinOp::Sub);
def_parser!(, parse_expr3, parse_expr2, "<" => BinOp::Less, ">" => BinOp::Greater);
def_parser!(, parse_expr4, parse_expr3, "==" => BinOp::Equal, "!=" => BinOp::NotEqual);
def_parser!(, parse_expr5, parse_expr4, "&&" => BinOp::And);
def_parser!(, parse_expr6, parse_expr5, "||" => BinOp::Or);
def_parser!(, parse_expr7, parse_expr6, "=" => BinOp::Subst);
def_parser!(pub, parse_expr, parse_expr7, "," => BinOp::Comma);

fn parse_factor(input: &str) -> nom::IResult<&str, Expr> {
    use nom::branch::alt;
    use nom::character::complete as character;
    use nom::combinator::map;
    use nom::sequence::tuple;

    alt((
        map(
            tuple((
                alt((
                    map(character::char('+'), |_| UnOp::Add),
                    map(character::char('-'), |_| UnOp::Sub),
                    map(character::char('*'), |_| UnOp::Mul),
                    map(character::char('/'), |_| UnOp::Div),
                    map(character::char('!'), |_| UnOp::Not),
                )),
                parse_factor,
            )),
            |(un_op, factor)| Expr::Un(un_op, factor.into()),
        ),
        parse_single,
    ))(input)
}

fn parse_single(input: &str) -> nom::IResult<&str, Expr> {
    use nom::branch::alt;
    use nom::bytes::complete as bytes;
    use nom::character::complete as character;
    use nom::combinator::map;
    use nom::combinator::opt;
    use nom::multi::fold_many0;
    use nom::number::complete as number;
    use nom::sequence::delimited;
    use nom::sequence::preceded;
    use nom::sequence::tuple;

    let (input, expr) = alt((
        map(number::double, |value| Expr::Num(value)),
        map(
            bytes::take_while1(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '$')),
            |s| Expr::Id(s),
        ),
        delimited(character::char('('), parse_expr, character::char(')')),
    ))(input)?;
    let (input, expr) = fold_many0(
        delimited(character::char('('), parse_expr, character::char(')')),
        expr,
        |prev, arg| Expr::Inv(prev.into(), arg.into()),
    )(input)?;
    fold_many0(
        preceded(
            character::char('|'),
            tuple((parse_expr, opt(preceded(character::char(':'), parse_expr)))),
        ),
        expr,
        |prev, map| match map {
            (cond, Some(map)) => Expr::Map(prev.into(), Some(cond.into()), map.into()),
            (map, None) => Expr::Map(prev.into(), None, map.into()),
        },
    )(input)
}
