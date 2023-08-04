use crate::parse::{create_raw_string, sgr_string, unwrap_string, ParseError, UnwrappedLiteral};

#[test]
fn unwrap_str() {
    use UnwrappedLiteral::*;
    for (test, result) in [
        (r#""""#, Some(String(""))),
        (r#""This one has text""#, Some(String("This one has text"))),
        // invalid str, impossible when using lib as proc macro
        (r#"""""#, Some(String(r#"""#))),
        (r#""Shouldn't work"#, None),
        (r#"" Also Shouldn't wor"k"#, None),
        (r##"r#""""#"##, Some(RawString(r#""""#, 1))),
        (r###"r##"#""#"##"###, Some(RawString(r##"#""#"##, 2))),
        (r###"r##"#""#"#"###, None),
    ] {
        assert_eq!(unwrap_string(test), result);
    }
}

#[test]
fn raw_string() {
    assert_eq!(create_raw_string("", 0), r#"r"""#);
    assert_eq!(
        create_raw_string("This one has text", 0),
        r#"r"This one has text""#
    );
    assert_eq!(
        create_raw_string("🚋 🏥 🤐 💷 🌛", 0),
        r#"r"🚋 🏥 🤐 💷 🌛""#
    );
    assert_eq!(create_raw_string("", 5), r######"r#####""#####"######);
}

#[test]
fn sgr_string_general() {
    for (test, result) in [
        ("", Ok("")),
        ("This one has text", Ok("This one has text")),
        ("\x1b", Ok("\x1b")),
        ("\u{1f604} ☀ ☁ ☂", Ok("\u{1f604} ☀ ☁ ☂")),
    ] {
        test_eq(test, result)
    }
}

#[test]
fn compiler_pass_off_undetected() {
    for (test, result) in [
        ("{", Ok("{")),
        ("{ ", Ok("{ ")),
        ("{with text", Ok("{with text")),
        ("with text{with text", Ok("with text{with text")),
    ] {
        test_eq(test, result)
    }
}
#[test]
fn compiler_pass_off() {
    use ParseError::*;
    for (test, result) in [
        (r"\", Err(CompilerPassOff)),
        (r"\   ", Err(CompilerPassOff)),
        (r"\Nope", Err(CompilerPassOff)),
        (r"\u{p}", Err(CompilerPassOff)),
        (r"\x", Err(CompilerPassOff)),
        (r"\x'", Err(CompilerPassOff)),
    ] {
        test_eq(test, result)
    }
}

#[test]
fn skip_whitespace() {
    for (test, result) in [
        (
            "\
        ",
            Ok(""),
        ),
        (
            "This works\
        ",
            Ok("This works"),
        ),
        (
            "This works \
        wow",
            Ok("This works wow"),
        ),
        (
            "\
        \n\r\t",
            Ok("\n\r\t"),
        ),
    ] {
        test_eq(test, result)
    }
}

#[test]
fn escapes() {
    test_eq(
        r#"\' \" \x00 \n \r \t \\ \0 \u{0}\
        "#,
        Ok("\' \" \x00 \n \r \t \\ \0 \u{0}\
        "),
    )
}

#[test]
fn curly_non_param() {
    test_eq("{{}} {{ { {", Ok("{{}} {{ { {"));
}
#[test]
fn curly_var_param() {
    test_eq(
        "{This wouldn't work} {not at all} {this_would_maybe} {}",
        Ok("{This wouldn't work} {not at all} {this_would_maybe} {}"),
    );
}
#[test]
fn param_errors() {
    for test in [
        "{[not_a_var]}",
        "invalid len num{[#000]}",
        "no num{[#0]}",
        "comma error {[0,0]}",
        "bracket {[yeah}",
    ] {
        let result = sgr_string(test);
        assert!(result.is_err(), "Unexpected value: {result:#?}")
    }
}
fn test_eq(test: &str, result: Result<&str, ParseError>) {
    match sgr_string(test) {
        Ok(test) => match result {
            Ok(result) => assert_eq!(test, result),
            Err(result) => panic!("\"{test}\" does not eq {result:#?}"),
        },
        Err(test) => match result {
            Ok(result) => panic!("{test:#?} does not eq {result}",),
            Err(result) => assert_eq!(test, result),
        },
    }
}
