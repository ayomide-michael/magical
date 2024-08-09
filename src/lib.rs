#[derive(Debug, PartialEq)]
pub enum Expected {
    AnyToken,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEndOfInput { expected: Expected },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub offset: (usize, usize),
}

impl Default for Location {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            offset: (0, 0),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Node<'node, T>
where
    T: ParserType,
{
    pub name: &'node str,
    pub value: T,
    pub location: Location,
    pub children: Vec<Node<'node, T>>,
}

impl<'node, T> Node<'node, T>
where
    T: ParserType,
{
    pub fn new(
        name: &'node str,
        value: T,
        location: Location,
        children: Vec<Node<'node, T>>,
    ) -> Self {
        Self {
            name,
            value,
            location,
            children,
        }
    }
}

pub type ParseResult<'parse_result, T> = Result<Node<'parse_result, T>, Error>;

pub struct Context<'context> {
    input: &'context str,
    location: Location,
}

impl<'context> Context<'context> {
    pub fn new(input: &'context str) -> Self {
        Self {
            input,
            location: Location::default(),
        }
    }

    pub fn advance(&mut self, characters: &str) {
        self.location.offset.0 = self.location.offset.1;

        for character in characters.chars() {
            if character == '\n' {
                self.location.line += 1;
                self.location.column = 1; // Reset column to 1 after a newline
            } else {
                self.location.column += 1;
            }
            self.location.offset.1 += character.len_utf8();
        }
    }

    pub fn advance_by(&mut self, length: usize) {
        if length == 0 {
            return;
        }

        if let Some(characters) = self
            .input
            .get(self.location.offset.1..self.location.offset.1 + length)
        {
            self.advance(characters);
        }
    }

    pub fn get_next_character(&self) -> Option<char> {
        self.input.chars().next()
    }
}

pub trait ParserType: std::fmt::Debug {}

impl<T> ParserType for T where T: std::fmt::Debug {}

pub trait ParseFN<'parse_fn, T>
where
    T: ParserType,
{
    fn call(&self, context: &mut Context<'parse_fn>) -> ParseResult<'parse_fn, T>;
}

impl<'parse_fn, T, F> ParseFN<'parse_fn, T> for F
where
    T: ParserType,
    F: Fn(&mut Context<'parse_fn>) -> ParseResult<'parse_fn, T>,
{
    fn call(&self, context: &mut Context<'parse_fn>) -> ParseResult<'parse_fn, T> {
        self(context)
    }
}

pub struct Parser<'parser, F, T>
where
    F: ParseFN<'parser, T>,
    T: ParserType,
{
    parse_fn: F,
    _parser_type: std::marker::PhantomData<&'parser T>,
}

impl<'parser, F, T> Parser<'parser, F, T>
where
    F: ParseFN<'parser, T>,
    T: ParserType,
{
    pub fn new(parse_fn: F) -> Self {
        Self {
            parse_fn,
            _parser_type: std::marker::PhantomData,
        }
    }

    pub fn parse(&self, input: &'parser str) -> ParseResult<'parser, T> {
        let mut context: Context = Context::new(input);

        self.parse_fn.call(&mut context)
    }
}

pub fn any_token<'any_token>() -> Parser<'any_token, impl ParseFN<'any_token, char>, char> {
    Parser::new(|ctx: &mut Context<'any_token>| match ctx.get_next_character() {
        Some(token) => {
            ctx.advance_by(token.len_utf8());

            Ok(Node::new("anyToken", token, ctx.location.clone(), vec![]))
        }
        None => Err(Error::UnexpectedEndOfInput {
            expected: Expected::AnyToken,
        }),
    })
}
