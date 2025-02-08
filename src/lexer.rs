pub enum TokenType<'a> {
    Identifier,
    Space,
    Comment,
    Keyword(&'a str),
    LiteralConstant,
    Punctuator(char),
    EndOfFile,
}

pub struct Range {
    from: i32,
    to: i32,
}

pub struct Token<'a> {
    token_type: TokenType<'a>,
    position: Range,
}

pub struct Lexer<'a> {
    source_file: &'a str,
    source_code: &'a str,
    pub tokens: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new<'b>(source_file: &'a str, source_code: &'a str) -> Self {
        let tokens = Lexer::parse_all_tokens(source_code);

        return Lexer {
            source_file,
            source_code,
            tokens,
        };
    }

    pub fn parse_all_tokens(source_code: &'a str) -> Vec<Token<'a>> {
        return vec![];
    }
}
