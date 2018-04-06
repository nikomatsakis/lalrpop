extern crate lalrpop_util;

pub mod calculator1; // synthesized by LALRPOP

#[test]
fn calculator1() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
    assert!(calculator1::TermParser::new().parse("(22)").is_ok());
    assert!(calculator1::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator1::TermParser::new().parse("((22)").is_err());
}

pub mod calculator2;

#[test]
fn calculator2() {
    assert!(calculator2::TermParser::new().parse("22").is_ok());
    assert!(calculator2::TermParser::new().parse("(22)").is_ok());
    assert!(calculator2::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator2::TermParser::new().parse("((22)").is_err());
}

pub mod calculator2b;

#[test]
fn calculator2b() {
    let result = calculator2b::TermParser::new().parse("33").unwrap();
    assert_eq!(result, "33");

    let result = calculator2b::TermParser::new().parse("foo33").unwrap();
    assert_eq!(result, "Id(foo33)");

    let result = calculator2b::TermParser::new().parse("(22)").unwrap();
    assert_eq!(result, "Twenty-two!");

    let result = calculator2b::TermParser::new().parse("(222)").unwrap();
    assert_eq!(result, "222");
}

pub mod calculator3;

#[cfg_attr(not(test), allow(unused_macros))]
macro_rules! test3 {
    ($expr:expr) => {
        println!("parsing {}", stringify!($expr));
        assert_eq!(calculator3::ExprParser::new().parse(stringify!($expr)).unwrap(), $expr);
    }
}

#[test]
fn calculator3() {
    test3!(22 + 44);
    test3!(22 - 44 - 66);
    test3!(22 * 44 + 66);
    test3!(22 * 44 + 66 / 3);
    test3!(22 * (44 + 66) / 3);
}

pub mod calculator4;
pub mod ast;

#[test]
fn calculator4() {
    let expr = calculator4::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}

pub mod calculator5;

#[test]
fn calculator5() {
    let expr = calculator5::ExprsParser::new().parse("").unwrap();
    assert_eq!(&format!("{:?}", expr), "[]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66)]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66,")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66)]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66, 13*3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66), (13 * 3)]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66, 13*3,")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66), (13 * 3)]");
}

pub mod calculator6;

#[test]
fn calculator6() {
    let mut errors = Vec::new();

    let expr = calculator6::ExprsParser::new()
        .parse(&mut errors, "22 * + 3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * error) + 3)]");

    let expr = calculator6::ExprsParser::new()
        .parse(&mut errors, "22 * 44 + 66, *3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66), (error * 3)]");

    let expr = calculator6::ExprsParser::new()
        .parse(&mut errors, "*")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[(error * error)]");

    assert_eq!(errors.len(), 4);
}

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
