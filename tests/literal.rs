use magical::{literal, Error, Expected, Location, Node};

const SOURCE: &str = "hello world";

#[test]
fn should_parse_hello_literal() {
    let parse_result: Result<Node<&str>, Error> = literal("hello").parse(SOURCE);

    assert_eq!(
        parse_result,
        Ok(Node::new(
            "literal",
            "hello",
            Location {
                line: 1,
                column: 6,
                offset: (0, 5)
            },
            vec![]
        ))
    )
}

#[test]
fn should_fail_on_hell() {
    let parse_result: Result<Node<&str>, Error> = literal("hello").parse("hell");

    assert_eq!(
        parse_result,
        Err(Error::InvalidLiteral {
            expected: Expected::Literal("hello"),
            found: "hell"
        })
    )
}

#[test]
fn should_fail_on_empty_input() {
    let parse_result: Result<Node<&str>, Error> = literal("hello").parse("");

    assert_eq!(
        parse_result,
        Err(Error::UnexpectedEndOfInput {
            expected: Expected::Literal("hello")
        })
    )
}
