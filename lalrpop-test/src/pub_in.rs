//! Test for pub-in support. The important thing to test here is that building with a pub-in
//! visibility specifier works, more so than running the actual test.

mod outer {
    pub(crate) mod inner {
        lalrpop_mod!(pub(crate) pub_in);
    }

    #[test]
    fn pub_in_test() {
        crate::util::test(|v| inner::pub_in::SParser::new().parse(v), "()", 0);
    }
}

// This should fail to build, because SParser is only visible to mod outer, even though
// outer::inner::pub_in is visible to the whole crate.
// #[test]
// fn pub_in_test() {
//     crate::util::test(|v| outer::inner::pub_in::SParser::new().parse(v), "()", 0);
// }
