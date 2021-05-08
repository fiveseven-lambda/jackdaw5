#[derive(Clone, Debug)]
pub struct Identifier<'s>(&'s str);
pub fn parse_identifier(input: &str) -> nom::IResult<&str, Identifier> {
    nom::combinator::map(
        nom::bytes::complete::take_while1(
            |c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '$'),
        ),
        |s| Identifier(s),
    )(input)
}

#[derive(Clone, Debug)]
pub enum Expr<'s> {
    Rhs(Vec<Vec<Cond<'s>>>),
    Substitution(Identifier<'s>, Box<Expr<'s>>),
}
pub fn parse_expr(input: &str) -> nom::IResult<&str, Expr> {
    nom::branch::alt((
        nom::combinator::map(
            nom::sequence::tuple((
                nom::sequence::delimited(
                    nom::character::complete::multispace0,
                    parse_identifier,
                    nom::character::complete::multispace0,
                ),
                nom::character::complete::char('='),
                parse_expr,
            )),
            |(id, _, expr)| Expr::Substitution(id, expr.into()),
        ),
        nom::combinator::map(
            nom::multi::separated_list1(
                nom::bytes::complete::tag("||"),
                nom::multi::separated_list1(nom::bytes::complete::tag("&&"), parse_cond),
            ),
            |vec| Expr::Rhs(vec),
        ),
    ))(input)
}

#[derive(Clone, Debug)]
pub enum Cond<'s> {
    Side(Side<'s>),
    Cmp(Box<Cond<'s>>, CmpOp, Side<'s>),
}
#[derive(Clone, Debug)]
pub enum CmpOp {
    Equal,
    NotEqual,
    Less,
    Greater,
}
fn parse_cond(input: &str) -> nom::IResult<&str, Cond> {
    let (input, head) = parse_side(input)?;
    nom::multi::fold_many0(
        nom::sequence::tuple((
            nom::branch::alt((
                nom::combinator::map(nom::bytes::complete::tag("=="), |_| CmpOp::Equal),
                nom::combinator::map(nom::bytes::complete::tag("!="), |_| CmpOp::NotEqual),
                nom::combinator::map(nom::character::complete::char('<'), |_| CmpOp::Less),
                nom::combinator::map(nom::character::complete::char('>'), |_| CmpOp::Greater),
            )),
            parse_side,
        )),
        Cond::Side(head),
        |prev, (operator, side)| Cond::Cmp(Box::new(prev), operator, side),
    )(input)
}

#[derive(Clone, Debug)]
pub enum Side<'s> {
    Term(Term<'s>),
    Operation(Box<Side<'s>>, AddOrSub, Term<'s>),
}
#[derive(Clone, Debug)]
pub enum AddOrSub {
    Add,
    Sub,
}
fn parse_side(input: &str) -> nom::IResult<&str, Side> {
    let (input, head) = parse_term(input)?;
    nom::multi::fold_many0(
        nom::sequence::tuple((
            nom::branch::alt((
                nom::combinator::map(nom::character::complete::char('+'), |_| AddOrSub::Add),
                nom::combinator::map(nom::character::complete::char('-'), |_| AddOrSub::Sub),
            )),
            parse_term,
        )),
        Side::Term(head),
        |prev, (operator, term)| Side::Operation(Box::new(prev), operator, term),
    )(input)
}

#[derive(Clone, Debug)]
pub enum Term<'s> {
    Factor(Factor<'s>),
    Operation(Box<Term<'s>>, MulOrDiv, Factor<'s>),
}
#[derive(Clone, Debug)]
pub enum MulOrDiv {
    Mul,
    Div,
}

fn parse_term(input: &str) -> nom::IResult<&str, Term> {
    let (input, head) = parse_factor(input)?;
    nom::multi::fold_many0(
        nom::sequence::tuple((
            nom::branch::alt((
                nom::combinator::map(nom::character::complete::char('*'), |_| MulOrDiv::Mul),
                nom::combinator::map(nom::character::complete::char('/'), |_| MulOrDiv::Div),
            )),
            parse_factor,
        )),
        Term::Factor(head),
        |prev, (operator, factor)| Term::Operation(Box::new(prev), operator, factor),
    )(input)
}

#[derive(Clone, Debug)]
pub enum Factor<'s> {
    Number(f64),
    Identifier(Identifier<'s>),
    Prefix(Prefix, Box<Factor<'s>>),
}

#[derive(Clone, Debug)]
pub enum Prefix {
    Add,
    Sub,
    Mul,
    Div,
}

fn parse_factor(input: &str) -> nom::IResult<&str, Factor> {
    nom::sequence::delimited(
        nom::character::complete::multispace0,
        nom::branch::alt((
            nom::combinator::map(nom::number::complete::double, |value| Factor::Number(value)),
            nom::combinator::map(parse_identifier, |id| Factor::Identifier(id)),
            nom::combinator::map(
                nom::sequence::tuple((
                    nom::branch::alt((
                        nom::combinator::map(nom::character::complete::char('+'), |_| Prefix::Add),
                        nom::combinator::map(nom::character::complete::char('-'), |_| Prefix::Sub),
                        nom::combinator::map(nom::character::complete::char('*'), |_| Prefix::Mul),
                        nom::combinator::map(nom::character::complete::char('/'), |_| Prefix::Div),
                    )),
                    parse_factor,
                )),
                |(prefix, factor)| Factor::Prefix(prefix, factor.into()),
            ),
        )),
        nom::character::complete::multispace0,
    )(input)
}
