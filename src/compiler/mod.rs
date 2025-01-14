use crate::tokenizer::*;
use anyhow::{bail, Result};
use std::fs::File;
use std::io::Write;
use std::iter::Peekable;

pub struct Compiler<'a> {
    tokenizer: &'a mut Peekable<Tokenizer<'a>>,
    output: &'a mut File,
    current_token: Option<Token>,
}

impl<'a> Compiler<'a> {
    pub fn new(tokenizer: &'a mut Peekable<Tokenizer<'a>>, output: &'a mut File) -> Self {
        Compiler {
            tokenizer,
            output,
            current_token: None,
        }
    }

    pub fn compile(&mut self) -> Result<()> {
        self.current_token = self.tokenizer.next();

        match &self.current_token {
            Some(Token::Keyword(Keyword::Class)) => self.compile_class(),
            _ => bail!(
                "Expected keyword class. Found instead: {:?}.",
                &self.current_token
            ),
        }
    }

    // Lexical elements
    fn compile_keyword(&mut self, expected_keyword: Keyword) -> Result<()> {
        let token = match &self.current_token {
            Some(token) => token,
            None => bail!("Expected to find a token."),
        };

        let keyword = match token {
            Token::Keyword(keyword) => {
                if &expected_keyword == keyword {
                    keyword
                } else {
                    bail!(
                        "Expected keyword: {:?}. Found: {:?}. ",
                        expected_keyword,
                        keyword
                    )
                }
            }
            _ => bail!("Expected to find a keyword."),
        };

        match keyword {
            Keyword::Class => write!(self.output, "<keyword> class </keyword>\n")?,
            Keyword::Constructor => write!(self.output, "<keyword> constructor </keyword>\n")?,
            Keyword::Function => write!(self.output, "<keyword> function </keyword>\n")?,
            Keyword::Method => write!(self.output, "<keyword> method </keyword>\n")?,
            Keyword::Field => write!(self.output, "<keyword> field </keyword>\n")?,
            Keyword::Static => write!(self.output, "<keyword> static </keyword>\n")?,
            Keyword::Var => write!(self.output, "<keyword> var </keyword>\n")?,
            Keyword::Int => write!(self.output, "<keyword> int </keyword>\n")?,
            Keyword::Char => write!(self.output, "<keyword> char </keyword>\n")?,
            Keyword::Boolean => write!(self.output, "<keyword> boolean </keyword>\n")?,
            Keyword::Void => write!(self.output, "<keyword> void </keyword>\n")?,
            Keyword::True => write!(self.output, "<keyword> true </keyword>\n")?,
            Keyword::False => write!(self.output, "<keyword> false </keyword>\n")?,
            Keyword::Null => write!(self.output, "<keyword> null </keyword>\n")?,
            Keyword::This => write!(self.output, "<keyword> this </keyword>\n")?,
            Keyword::Let => write!(self.output, "<keyword> let </keyword>\n")?,
            Keyword::Do => write!(self.output, "<keyword> do </keyword>\n")?,
            Keyword::If => write!(self.output, "<keyword> if </keyword>\n")?,
            Keyword::Else => write!(self.output, "<keyword> else </keyword>\n")?,
            Keyword::While => write!(self.output, "<keyword> while </keyword>\n")?,
            Keyword::Return => write!(self.output, "<keyword> return </keyword>\n")?,
        }

        self.current_token = self.tokenizer.next();
        Ok(())
    }

    fn compile_symbol(&mut self, expected_symbol: Symbol) -> Result<()> {
        let token = match &self.current_token {
            Some(token) => token,
            None => bail!("Expected to find a token."),
        };

        let symbol = match token {
            Token::Symbol(symbol) => {
                if &expected_symbol == symbol {
                    symbol
                } else {
                    bail!(
                        "Expected symbol: {:?}. Found: {:?}. ",
                        expected_symbol,
                        symbol
                    )
                }
            }
            _ => bail!(
                "Expected to find symbol {:?}. But found this instead: {:?}",
                expected_symbol,
                token
            ),
        };

        match symbol {
            Symbol::LeftCurlyBracket => write!(self.output, "<symbol> {{ </symbol>\n")?,
            Symbol::RightCurlyBracket => write!(self.output, "<symbol> }} </symbol>\n")?,
            Symbol::LeftRoundBracket => write!(self.output, "<symbol> ( </symbol>\n")?,
            Symbol::RightRoundBracket => write!(self.output, "<symbol> ) </symbol>\n")?,
            Symbol::LeftSquareBracket => write!(self.output, "<symbol> [ </symbol>\n")?,
            Symbol::RightSquareBracket => write!(self.output, "<symbol> ] </symbol>\n")?,
            Symbol::Dot => write!(self.output, "<symbol> . </symbol>\n")?,
            Symbol::Comma => write!(self.output, "<symbol> , </symbol>\n")?,
            Symbol::Semicolon => write!(self.output, "<symbol> ; </symbol>\n")?,
            Symbol::Plus => write!(self.output, "<symbol> + </symbol>\n")?,
            Symbol::Minus => write!(self.output, "<symbol> - </symbol>\n")?,
            Symbol::Times => write!(self.output, "<symbol> * </symbol>\n")?,
            Symbol::Divide => write!(self.output, "<symbol> / </symbol>\n")?,
            Symbol::And => write!(self.output, "<symbol> &amp; </symbol>\n")?,
            Symbol::Or => write!(self.output, "<symbol> | </symbol>\n")?,
            Symbol::SmallerThan => write!(self.output, "<symbol> &lt; </symbol>\n")?,
            Symbol::GreaterThan => write!(self.output, "<symbol> &gt; </symbol>\n")?,
            Symbol::Equal => write!(self.output, "<symbol> = </symbol>\n")?,
            Symbol::Not => write!(self.output, "<symbol> ~ </symbol>\n")?,
        }

        self.current_token = self.tokenizer.next();
        Ok(())
    }

    fn compile_integer_constant(&mut self) -> Result<()> {
        match &self.current_token {
            Some(Token::IntegerConstant(IntegerConstantValue { value })) => {
                write!(
                    self.output,
                    "<integerConstant> {} </integerConstant>\n",
                    value
                )?;
            }
            _ => bail!(
                "Expected to find integer constant. But found this instead: {:?}.",
                self.current_token
            ),
        }

        self.current_token = self.tokenizer.next();
        Ok(())
    }

    fn compile_string_constant(&mut self) -> Result<()> {
        match &self.current_token {
            Some(Token::StringConstant(value)) => {
                write!(
                    self.output,
                    "<stringConstant> {} </stringConstant>\n",
                    value
                )?;
            }
            _ => bail!(
                "Expected to find string constant. But found this instead: {:?}.",
                self.current_token
            ),
        }

        self.current_token = self.tokenizer.next();
        Ok(())
    }

    fn compile_identifier(&mut self) -> Result<()> {
        match &self.current_token {
            Some(Token::Identifier(IdentifierValue { value })) => {
                write!(self.output, "<identifier> {} </identifier>\n", value)?;
            }
            _ => bail!(
                "Expected to find identifier. But found this instead: {:?}.",
                self.current_token
            ),
        }

        self.current_token = self.tokenizer.next();
        Ok(())
    }

    // Pogram structure
    fn compile_class(&mut self) -> Result<()> {
        write!(self.output, "<class>\n")?;
        self.compile_keyword(Keyword::Class)?;
        self.compile_identifier()?;
        self.compile_symbol(Symbol::LeftCurlyBracket)?;

        while self.current_token == Some(Token::Keyword(Keyword::Static))
            || self.current_token == Some(Token::Keyword(Keyword::Field))
        {
            self.compile_classvardec()?;
        }

        while self.current_token == Some(Token::Keyword(Keyword::Constructor))
            || self.current_token == Some(Token::Keyword(Keyword::Function))
            || self.current_token == Some(Token::Keyword(Keyword::Method))
        {
            self.compile_subroutinedec()?;
        }

        self.compile_symbol(Symbol::RightCurlyBracket)?;
        write!(self.output, "</class>\n")?;

        Ok(())
    }

    fn compile_classvardec(&mut self) -> Result<()> {
        write!(self.output, "<classVarDec>\n")?;

        match self.current_token {
            Some(Token::Keyword(Keyword::Static)) => self.compile_keyword(Keyword::Static)?,
            Some(Token::Keyword(Keyword::Field)) => self.compile_keyword(Keyword::Field)?,
            _ => bail!(
                "Expected to find keyword static or field. But found this instead: {:?}.",
                self.current_token
            ),
        }

        self.compile_type()?;
        self.compile_identifier()?;

        while self.current_token == Some(Token::Symbol(Symbol::Comma)) {
            self.compile_symbol(Symbol::Comma)?;
            self.compile_identifier()?;
        }

        self.compile_symbol(Symbol::Semicolon)?;
        write!(self.output, "</classVarDec>\n")?;
        Ok(())
    }

    fn compile_type(&mut self) -> Result<()> {
        match &self.current_token {
            Some(Token::Keyword(Keyword::Int)) => {
                self.compile_keyword(Keyword::Int)?;
            }
            Some(Token::Keyword(Keyword::Char)) => {
                self.compile_keyword(Keyword::Char)?;
            }
            Some(Token::Keyword(Keyword::Boolean)) => {
                self.compile_keyword(Keyword::Boolean)?;
            }
            Some(Token::Identifier(_)) => {
                self.compile_identifier()?;
            }
            _ => bail!(
                "Expected to find type. But found this instead: {:?}.",
                self.current_token
            ),
        }

        Ok(())
    }

    fn compile_subroutinedec(&mut self) -> Result<()> {
        write!(self.output, "<subroutineDec>\n")?;

        match &self.current_token {
            Some(Token::Keyword(Keyword::Constructor)) => {
                self.compile_keyword(Keyword::Constructor)?
            }
            Some(Token::Keyword(Keyword::Function)) => self.compile_keyword(Keyword::Function)?,
            Some(Token::Keyword(Keyword::Method)) => self.compile_keyword(Keyword::Method)?,
            _ => bail!(
                "Expected to find keyword 'constructor', 'function' or 'method'. But found this instead: {:?}.",
                self.current_token
            ),
        }

        match &self.current_token {
            Some(Token::Keyword(Keyword::Void)) => self.compile_keyword(Keyword::Void)?,
            _ => self.compile_type()?,
        }

        self.compile_identifier()?;
        self.compile_symbol(Symbol::LeftRoundBracket)?;
        self.compile_parameterlist()?;
        self.compile_symbol(Symbol::RightRoundBracket)?;
        self.compile_subroutinebody()?;

        write!(self.output, "</subroutineDec>\n")?;
        Ok(())
    }

    fn compile_parameterlist(&mut self) -> Result<()> {
        write!(self.output, "<parameterList>\n")?;

        match self.compile_type() {
            Ok(_) => {
                self.compile_identifier()?;

                while self.current_token == Some(Token::Symbol(Symbol::Comma)) {
                    self.compile_symbol(Symbol::Comma)?;
                    self.compile_type()?;
                    self.compile_identifier()?;
                }
            }
            Err(_) => {}
        };

        write!(self.output, "</parameterList>\n")?;
        Ok(())
    }

    fn compile_subroutinebody(&mut self) -> Result<()> {
        write!(self.output, "<subroutineBody>\n")?;

        self.compile_symbol(Symbol::LeftCurlyBracket)?;

        while self.current_token == Some(Token::Keyword(Keyword::Var)) {
            self.compile_vardec()?;
        }

        write!(self.output, "<statements>\n")?;

        while self.is_statement() {
            self.compile_statement()?;
        }

        write!(self.output, "</statements>\n")?;

        self.compile_symbol(Symbol::RightCurlyBracket)?;

        write!(self.output, "</subroutineBody>\n")?;
        Ok(())
    }

    fn compile_vardec(&mut self) -> Result<()> {
        write!(self.output, "<varDec>\n")?;

        self.compile_keyword(Keyword::Var)?;
        self.compile_type()?;
        self.compile_identifier()?;

        while self.current_token == Some(Token::Symbol(Symbol::Comma)) {
            self.compile_symbol(Symbol::Comma)?;
            self.compile_identifier()?;
        }

        self.compile_symbol(Symbol::Semicolon)?;

        write!(self.output, "</varDec>\n")?;
        Ok(())
    }

    // Statements
    fn compile_statement(&mut self) -> Result<()> {
        match self.current_token {
            Some(Token::Keyword(Keyword::Let)) => {
                self.compile_let_statement()?;
            }
            Some(Token::Keyword(Keyword::If)) => {
                self.compile_if_statement()?;
            }
            Some(Token::Keyword(Keyword::While)) =>{
                self.compile_while_statement()?;
            }
            Some(Token::Keyword(Keyword::Do)) => {
                self.compile_do_statement()?;
            }
            Some(Token::Keyword(Keyword::Return)) =>{
                self.compile_return_statement()?;
            }
            _ => bail!(
                "Expected to find 'let', 'if', 'while', 'do', or 'return'. But found this instead: {:?}.",
                self.current_token
            ),
        }

        Ok(())
    }

    fn compile_let_statement(&mut self) -> Result<()> {
        write!(self.output, "<letStatement>\n")?;

        self.compile_keyword(Keyword::Let)?;
        self.compile_identifier()?;

        if self.current_token == Some(Token::Symbol(Symbol::LeftSquareBracket)) {
            self.compile_symbol(Symbol::LeftSquareBracket)?;
            self.compile_expression()?;
            self.compile_symbol(Symbol::RightSquareBracket)?;
        }

        self.compile_symbol(Symbol::Equal)?;
        self.compile_expression()?;
        self.compile_symbol(Symbol::Semicolon)?;

        write!(self.output, "</letStatement>\n")?;
        Ok(())
    }

    fn compile_if_statement(&mut self) -> Result<()> {
        write!(self.output, "<ifStatement>\n")?;

        self.compile_keyword(Keyword::If)?;
        self.compile_symbol(Symbol::LeftRoundBracket)?;
        self.compile_expression()?;
        self.compile_symbol(Symbol::RightRoundBracket)?;
        self.compile_symbol(Symbol::LeftCurlyBracket)?;

        write!(self.output, "<statements>\n")?;

        while self.is_statement() {
            self.compile_statement()?;
        }

        write!(self.output, "</statements>\n")?;

        self.compile_symbol(Symbol::RightCurlyBracket)?;

        if self.current_token == Some(Token::Keyword(Keyword::Else)) {
            self.compile_keyword(Keyword::Else)?;
            self.compile_symbol(Symbol::LeftCurlyBracket)?;

            write!(self.output, "<statements>\n")?;

            while self.is_statement() {
                self.compile_statement()?;
            }

            write!(self.output, "</statements>\n")?;

            self.compile_symbol(Symbol::RightCurlyBracket)?;
        }

        write!(self.output, "</ifStatement>\n")?;
        Ok(())
    }

    fn compile_while_statement(&mut self) -> Result<()> {
        write!(self.output, "<whileStatement>\n")?;

        self.compile_keyword(Keyword::While)?;
        self.compile_symbol(Symbol::LeftRoundBracket)?;
        self.compile_expression()?;
        self.compile_symbol(Symbol::RightRoundBracket)?;
        self.compile_symbol(Symbol::LeftCurlyBracket)?;

        write!(self.output, "<statements>\n")?;

        while self.is_statement() {
            self.compile_statement()?;
        }

        write!(self.output, "</statements>\n")?;

        self.compile_symbol(Symbol::RightCurlyBracket)?;

        write!(self.output, "</whileStatement>\n")?;
        Ok(())
    }

    fn compile_do_statement(&mut self) -> Result<()> {
        write!(self.output, "<doStatement>\n")?;

        self.compile_keyword(Keyword::Do)?;
        self.compile_subroutine_call()?;
        self.compile_symbol(Symbol::Semicolon)?;

        write!(self.output, "</doStatement>\n")?;
        Ok(())
    }

    fn compile_return_statement(&mut self) -> Result<()> {
        write!(self.output, "<returnStatement>\n")?;

        self.compile_keyword(Keyword::Return)?;

        if self.is_expression() {
            self.compile_expression()?;
        }

        self.compile_symbol(Symbol::Semicolon)?;

        write!(self.output, "</returnStatement>\n")?;
        Ok(())
    }

    // Expressions
    fn compile_expression(&mut self) -> Result<()> {
        write!(self.output, "<expression>\n")?;

        self.compile_term()?;

        while self.is_op() {
            self.compile_op()?;
            self.compile_term()?;
        }

        write!(self.output, "</expression>\n")?;

        Ok(())
    }

    fn compile_term(&mut self) -> Result<()> {
        write!(self.output, "<term>\n")?;

        match &self.current_token {
            Some(Token::IntegerConstant(_value)) => {
                self.compile_integer_constant()?;
            }
            Some(Token::StringConstant(_value)) => {
                self.compile_string_constant()?;
            }
            Some(Token::Keyword(Keyword::True)) => {
                self.compile_keyword(Keyword::True)?;
            }
            Some(Token::Keyword(Keyword::False)) => {
                self.compile_keyword(Keyword::False)?;
            }
            Some(Token::Keyword(Keyword::Null)) => {
                self.compile_keyword(Keyword::Null)?;
            }
            Some(Token::Keyword(Keyword::This)) => {
                self.compile_keyword(Keyword::This)?;
            }
            Some(Token::Identifier(_value)) => {
                if self.tokenizer.peek() == Some(&Token::Symbol(Symbol::LeftSquareBracket)) {
                    // Process array indexing
                    self.compile_identifier()?;
                    self.compile_symbol(Symbol::LeftSquareBracket)?;
                    self.compile_expression()?;
                    self.compile_symbol(Symbol::RightSquareBracket)?;
                } else if self.tokenizer.peek() == Some(&Token::Symbol(Symbol::LeftRoundBracket)) {
                    self.compile_subroutine_call()?;
                } else if self.tokenizer.peek() == Some(&Token::Symbol(Symbol::Dot)) {
                    self.compile_subroutine_call()?;
                } else {
                    self.compile_identifier()?;
                }
            }
            Some(Token::Symbol(Symbol::LeftRoundBracket)) => {
                self.compile_symbol(Symbol::LeftRoundBracket)?;
                self.compile_expression()?;
                self.compile_symbol(Symbol::RightRoundBracket)?;
            }
            Some(Token::Symbol(Symbol::Minus)) => {
                self.compile_symbol(Symbol::Minus)?;
                self.compile_term()?;
            }
            Some(Token::Symbol(Symbol::Not)) => {
                self.compile_symbol(Symbol::Not)?;
                self.compile_term()?;
            }
            _ => bail!(
                "Expected to find term. But found this instead: {:?}.",
                self.current_token
            ),
        }

        write!(self.output, "</term>\n")?;
        Ok(())
    }

    fn compile_subroutine_call(&mut self) -> Result<()> {
        self.compile_identifier()?;

        if self.current_token == Some(Token::Symbol(Symbol::Dot)) {
            self.compile_symbol(Symbol::Dot)?;
            self.compile_identifier()?;
        }

        self.compile_symbol(Symbol::LeftRoundBracket)?;
        self.compile_expression_list()?;
        self.compile_symbol(Symbol::RightRoundBracket)?;

        Ok(())
    }

    fn compile_expression_list(&mut self) -> Result<()> {
        write!(self.output, "<expressionList>\n")?;

        if self.is_expression() {
            self.compile_expression()?;

            while self.current_token == Some(Token::Symbol(Symbol::Comma)) {
                self.compile_symbol(Symbol::Comma)?;
                self.compile_expression()?;
            }
        }

        write!(self.output, "</expressionList>\n")?;
        Ok(())
    }

    fn compile_op(&mut self) -> Result<()> {
        match self.current_token {
            Some(Token::Symbol(Symbol::Plus)) => self.compile_symbol(Symbol::Plus)?,
            Some(Token::Symbol(Symbol::Minus)) => self.compile_symbol(Symbol::Minus)?,
            Some(Token::Symbol(Symbol::Times)) => self.compile_symbol(Symbol::Times)?,
            Some(Token::Symbol(Symbol::Divide)) => self.compile_symbol(Symbol::Divide)?,
            Some(Token::Symbol(Symbol::And)) => self.compile_symbol(Symbol::And)?,
            Some(Token::Symbol(Symbol::Or)) => self.compile_symbol(Symbol::Or)?,
            Some(Token::Symbol(Symbol::SmallerThan)) => self.compile_symbol(Symbol::SmallerThan)?,
            Some(Token::Symbol(Symbol::GreaterThan)) => self.compile_symbol(Symbol::GreaterThan)?,
            Some(Token::Symbol(Symbol::Equal)) => self.compile_symbol(Symbol::Equal)?,
            _ => bail!(
                "Expected op symbol. Found instead: {:?}.",
                &self.current_token
            ),
        };

        Ok(())
    }

    // Utilitites
    fn is_expression(&self) -> bool {
        match &self.current_token {
            Some(Token::IntegerConstant(_value)) => true,
            Some(Token::StringConstant(_value)) => true,
            Some(Token::Keyword(Keyword::True)) => true,
            Some(Token::Keyword(Keyword::False)) => true,
            Some(Token::Keyword(Keyword::Null)) => true,
            Some(Token::Keyword(Keyword::This)) => true,
            Some(Token::Identifier(_value)) => true,
            Some(Token::Symbol(Symbol::LeftRoundBracket)) => true,
            Some(Token::Symbol(Symbol::Minus)) => true,
            Some(Token::Symbol(Symbol::Not)) => true,
            _ => false,
        }
    }

    fn is_statement(&self) -> bool {
        match self.current_token {
            Some(Token::Keyword(Keyword::Let)) => true,
            Some(Token::Keyword(Keyword::If)) => true,
            Some(Token::Keyword(Keyword::While)) => true,
            Some(Token::Keyword(Keyword::Do)) => true,
            Some(Token::Keyword(Keyword::Return)) => true,
            _ => false,
        }
    }

    fn is_op(&self) -> bool {
        match self.current_token {
            Some(Token::Symbol(Symbol::Plus)) => true,
            Some(Token::Symbol(Symbol::Minus)) => true,
            Some(Token::Symbol(Symbol::Times)) => true,
            Some(Token::Symbol(Symbol::Divide)) => true,
            Some(Token::Symbol(Symbol::And)) => true,
            Some(Token::Symbol(Symbol::Or)) => true,
            Some(Token::Symbol(Symbol::SmallerThan)) => true,
            Some(Token::Symbol(Symbol::GreaterThan)) => true,
            Some(Token::Symbol(Symbol::Equal)) => true,
            _ => false,
        }
    }
}
