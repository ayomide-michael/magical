use magical::{token, Error, Expected, Location, Node};

const SOURCE: &str = "abc";

#[test]
fn should_succeed_on_a() {
    let parse_result: Result<Node<char>, Error> = token('a').parse(SOURCE);

    assert_eq!(
        parse_result,
        Ok(Node {
            name: "token",
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
fn should_fail_on_b() {
    let parse_result: Result<Node<char>, Error> = token('b').parse(SOURCE);

    assert_eq!(
        parse_result,
        Err(Error::MisMatchedCharacter {
            expected: Expected::Character('b'),
            found: 'a'
        })
    )
}

#[test]
fn should_fail_on_empty_input() {
    let parse_result: Result<Node<char>, Error> = token('b').parse("");

    assert_eq!(
        parse_result,
        Err(Error::UnexpectedEndOfInput {
            expected: Expected::Character('b')
        })
    )
}
