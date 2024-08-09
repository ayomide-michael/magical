use magical::{any_token, Error, Expected, Location, Node};

const SOURCE: &str = "abc";

#[test]
fn should_parse_a_valid_input() {
    let parse_result: Result<Node<char>, Error> = any_token().parse(SOURCE);

    assert_eq!(
        parse_result,
        Ok(Node {
            name: "anyToken",
            value: 'a',
            location: Location {
                line: 1,
                column: 2,
                offset: (0, 1)
            },
            children: vec![]
        })
    )
}

#[test]
fn should_fail_on_empty_input() {
    let parse_result: Result<Node<char>, Error> = any_token().parse("");

    assert_eq!(
        parse_result,
        Err(Error::UnexpectedEndOfInput {
            expected: Expected::AnyToken
        })
    )
}
