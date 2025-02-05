#[derive(Default, Debug)]
pub struct Parser {
    buffer: Vec<Token>,
    word_buffer: Vec<char>,
    context: ParserContext,
}
impl Parser {
    pub fn parse(&mut self, i: String) -> ParseRes {
        for c in i.chars() {
            if !self.context.break_char(c) {
                self.word_buffer.push(c);
            } else {
                match self.push_to_buff(self.context.incl_break(c)) {
                    ParseRes::Null => (),
                    ParseRes::Break => return ParseRes::Break,
                }
            }
        }
        if !self.word_buffer.is_empty() {
            match self.push_to_buff(None) {
                ParseRes::Null => (),
                ParseRes::Break => return ParseRes::Break,
            }
        }

        ParseRes::Null
    }
    fn push_to_buff(&mut self, incl_bc: Option<Token>) -> ParseRes {
        let new_piece = self.word_buffer.drain(..).collect::<String>();
        let new_token = Token::make(new_piece, &self.context);
        if !new_token.is_sudden() {
            self.buffer.push(new_token);
            if let Some(bc) = incl_bc {
                self.buffer.push(bc);
            }
        } else {
            match new_token {
                Token::UtilFunc(util) => {
                    if let Util::Exit = util {
                        return ParseRes::Break;
                    }
                }
                Token::Word(_) => todo!(),
                Token::Op(_) => todo!(),
                Token::Number => todo!(),
            }
        }
        //self.buffer.push();
        ParseRes::Null
    }
}
#[derive(Default, Debug)]
enum ParserContext {
    #[default]
    Null,
    String,
    Parens,
}

impl ParserContext {
    pub fn break_char(&self, i: char) -> bool {
        match self {
            ParserContext::Null => i.is_whitespace() || i == '=',
            ParserContext::String => todo!(),
            ParserContext::Parens => todo!(),
        }
    }

    pub fn incl_break(&self, i: char) -> Option<Token> {
        match self {
            ParserContext::Null => {
                if i == '=' {
                    Some(Token::Op(Op::Equal))
                } else {
                    None
                }
            }
            ParserContext::String => todo!(),
            ParserContext::Parens => todo!(),
        }
    }
}

#[derive(Debug)]
enum Token {
    UtilFunc(Util),
    Word(String),
    Number,
    Op(Op),
}

impl Token {
    pub fn make(i: String, context: &ParserContext) -> Self {
        match context {
            ParserContext::Null => Self::from_null_context(i),
            ParserContext::String => todo!(),
            ParserContext::Parens => todo!(),
        }
    }

    fn from_null_context(i: String) -> Self {
        match i.as_str() {
            "exit" => Self::UtilFunc(Util::Exit),
            "export" => Self::UtilFunc(Util::Export),
            _ => Self::Word(i),
        }
    }
    pub fn is_sudden(&self) -> bool {
        match self {
            Token::UtilFunc(util) => util.is_sudden(),
            Token::Word(_) => false,
            Token::Number => false,
            Token::Op(_) => false,
        }
    }
}

#[derive(Debug)]
enum Op {
    Equal,
}

#[derive(Debug)]
enum Util {
    Exit,
    Export,
}

impl Util {
    pub fn is_sudden(&self) -> bool {
        matches!(self, Util::Exit)
    }
}

#[derive(Debug)]
pub enum ParseRes {
    Null,
    Break,
}
