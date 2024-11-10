// const := 0-9 | 0-9<const>
// op := + | - | * | /
//
// expr := <const> | <list>
//
// content := <expr> | <expr> <content>
// list := (<op> <content>)

use std::io::{self, Write};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    multi::{many0, many1, separated_list0},
    IResult,
};

#[derive(Debug)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Expr {
    Const(i32),
    List(Operator, Vec<Expr>),
}

impl TryFrom<&str> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> anyhow::Result<Self> {
        match value {
            "+" => Ok(Operator::Plus),
            "-" => Ok(Operator::Minus),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            _ => Err(anyhow::anyhow!("unknown operator '{}'", value)),
        }
    }
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    let (input, op) = alt((tag("+"), tag("-"), tag("*"), tag("/")))(input)?;
    Ok((input, op.try_into().unwrap()))
}

fn parse_list(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("(")(input)?;
    let (input, op) = parse_operator(input)?;
    let (input, _) = many0(tag(" "))(input)?;
    let (input, exprs) = separated_list0(tag(" "), parse_expr)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Expr::List(op, exprs)))
}

fn parse_constant(input: &str) -> IResult<&str, Expr> {
    let (input, number) = many1(digit1)(input)?;
    let num: i32 = number.join("").parse().unwrap();
    Ok((input, Expr::Const(num)))
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    // either parse constant or parse list
    alt((parse_constant, parse_list))(input)
}

fn evaluate(e: &Expr) -> anyhow::Result<i32> {
    match e {
        Expr::Const(c) => Ok(*c),
        Expr::List(op, exprs) => {
            let mut value = evaluate(&exprs[0])?;
            let mut exprs = exprs.iter();
            exprs.next();
            for e in exprs {
                let other = evaluate(e)?;

                match op {
                    Operator::Plus => value += other,
                    Operator::Minus => value -= other,
                    Operator::Multiply => value *= other,
                    Operator::Divide => value /= other,
                }
            }

            Ok(value)
        }
    }
}

fn repl() -> anyhow::Result<()> {
    loop {
        print!(">>> ");
        io::stdout().flush()?;
        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            return Ok(());
        } else if input == "\n" {
            continue;
        }

        match parse_expr(&input) {
            Ok((rest, expr)) => {
                if rest != "\n" {
                    eprintln!("invalid expr: {input}");
                    continue;
                }
                match evaluate(&expr) {
                    Ok(res) => println!("=> {res}"),
                    Err(e) => {
                        dbg!(e);
                    }
                }
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    repl()?;
    Ok(())
}
