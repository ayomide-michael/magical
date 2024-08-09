#[derive(Debug)]
pub enum Error {}

#[derive(Debug)]
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

#[derive(Debug)]
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
