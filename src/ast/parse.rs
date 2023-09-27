use std::ops::Add;

use nom::{
    branch::{self, alt},
    bytes::complete::{take_while, take_while1},
    character::{
        self,
        complete::{self, multispace0},
    },
    combinator::{map, opt, recognize},
    multi::{self},
    sequence::{delimited, preceded, tuple},
    IResult, Parser,
};

#[derive(Debug,Clone)]
pub struct DynFunction {
    pub name: String,
    pub params: Vec<DynParam>,
}

#[derive(Debug,Clone)]
pub enum DynParam {
    Function(DynFunction),
    Value(DynType),
}

#[derive(Debug,Clone)]
pub enum DynType {
    I64(i64),
    F64(f64),
    Str(String),
}

impl DynFunction {
    pub fn get_all_value(self)->Vec<DynType>{
        self.params
        .iter()
        .filter_map(|param| match param {
            DynParam::Value(dyn_type) => Some(dyn_type.clone()), 
            _ => None,
        }) 
        .collect::<Vec<DynType>>() 
    }  
}

impl DynParam {
    #[allow(dead_code)]
    pub fn extract_value(self) -> Option<DynType> {
        match self {
            DynParam::Value(dyn_type) => Some(dyn_type),
            _ => None,
        }
    }
}

impl Add for DynType {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (DynType::I64(a), DynType::I64(b)) => DynType::I64(a + b),
            (DynType::F64(a), DynType::F64(b)) => DynType::F64(a + b),
            (DynType::Str(a), DynType::Str(b)) => DynType::Str(a+&b),
            _ => panic!("Cannot add String types or mix different types."),
        }
    }
}

macro_rules! impl_into_for_dynamic_arg {
    ($type:ty, $variant:ident) => {
        impl From<DynType> for $type {
            fn from(dynamic_arg: DynType) -> Self {
                match dynamic_arg {
                    DynType::$variant(val) => val,
                    _ => panic!(concat!("Cannot convert DynamicArg to ", stringify!($type))),
                }
            }
        }
    };
}

impl DynType {
    #[allow(dead_code)]
    fn into_i64(self) -> i64 {
        impl_into_for_dynamic_arg!(i64, I64);
        self.into()
    }
    #[allow(dead_code)]
    fn into_f64(self) -> f64 {
        impl_into_for_dynamic_arg!(f64, F64);
        self.into()
    }
    #[allow(dead_code)]
    fn into_str(self) -> String {
        impl_into_for_dynamic_arg!(String, Str);
        self.into()
    }
}

fn is_valid_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn parse_identifier(input: &str) -> IResult<&str, String> {
    map(
        take_while1(is_valid_identifier_char),
        |s: &str| s.to_string(),
    )(input)
}

fn parse_parameter(input: &str) -> IResult<&str, DynParam> {
    let (input, _) = multispace0(input)?;
    alt((
        map(
            recognize(tuple((
                take_while1(|c: char| c.is_ascii_digit()),
                opt(preceded(
                    character::complete::char('.'),
                    take_while(|c: char| c.is_ascii_digit()),
                )),
            ))),
            |s: &str| {
                if s.contains('.') {
                    DynParam::Value(DynType::F64(s.parse::<f64>().unwrap()))
                } else {
                    DynParam::Value(DynType::I64(s.parse::<i64>().unwrap()))
                }
            },
        ),
        delimited(
            character::complete::char('\''),
            take_while(|c| c != '\''),
            character::complete::char('\''),
        )
        .map(|s: &str| DynParam::Value(DynType::Str(s.to_owned()))),
        map(parser, DynParam::Function),
    ))(input)
}

pub fn parser(input: &str) -> IResult<&str, DynFunction> {
    let (input, _) = multispace0(input)?;
    let (input, method_name) = parse_identifier(input)?;
    let (input, _) = complete::char('(')(input)?;

    let (input, params) = branch::alt((
        multi::separated_list0(complete::char(','), parse_parameter),
        complete::char(')').map(|_| vec![]),
    ))(input)?;

    let (input, _) = character::complete::char(')')(input)?;

    Ok((
        input,
        DynFunction {
            name: method_name,
            params,
        },
    ))
}

#[test]
fn ast_parser_test() {
    let input = "sum(
        sendHttp('http://127.0.0.1:8080/getNumber','id=123'),
        add( 1.26, 2, 3, add( 7, 8, 9)),
        multiplication(4, 5, 6),
        division(1,3))";
    let res = parser(input);
    println!("{:#?}", res);
}
