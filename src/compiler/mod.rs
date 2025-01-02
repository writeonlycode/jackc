use crate::tokenizer::*;
use anyhow::{bail, Result};
use std::fs::File;
use std::io::Write;

pub struct Compiler<'a> {
    tokenizer: &'a mut Tokenizer<'a>,
    output: &'a mut File,
    current_token: Option<Token>,
}

impl<'a> Compiler<'a> {
    pub fn new(tokenizer: &'a mut Tokenizer<'a>, output: &'a mut File) -> Self {
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
            Keyword::Class => write!(self.output, "<keyword> class </keyword>")?,
            Keyword::Constructor => write!(self.output, "<keyword> constructor </eyword>")?,
            Keyword::Function => write!(self.output, "<keyword> function </keyword>")?,
            Keyword::Method => write!(self.output, "<keyword> method </keyword>")?,
            Keyword::Field => write!(self.output, "<keyword> field </keyword>")?,
            Keyword::Static => write!(self.output, "<keyword> static </keyword>")?,
            Keyword::Var => write!(self.output, "<keyword> var </keyword>")?,
            Keyword::Int => write!(self.output, "<keyword> int </keyword>")?,
            Keyword::Char => write!(self.output, "<keyword> char </keyword>")?,
            Keyword::Boolean => write!(self.output, "<keyword> boolean </keyword>")?,
            Keyword::Void => write!(self.output, "<keyword> void </keyword>")?,
            Keyword::True => write!(self.output, "<keyword> true </keyword>")?,
            Keyword::False => write!(self.output, "<keyword> false </keyword>")?,
            Keyword::Null => write!(self.output, "<keyword> null </keyword>")?,
            Keyword::This => write!(self.output, "<keyword> this </keyword>")?,
            Keyword::Let => write!(self.output, "<keyword> let </keyword>")?,
            Keyword::Do => write!(self.output, "<keyword> do </keyword>")?,
            Keyword::If => write!(self.output, "<keyword> if </keyword>")?,
            Keyword::Else => write!(self.output, "<keyword> else </keyword>")?,
            Keyword::While => write!(self.output, "<keyword> while </keyword>")?,
            Keyword::Return => write!(self.output, "<keyword> return </keyword>")?,
        }

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
                "Expected to find a symbol. But found this instead: {:?}",
                token
            ),
        };

        match symbol {
            Symbol::LeftCurlyBracket => write!(self.output, "<symbol> {{ </symbol>")?,
            Symbol::RightCurlyBracket => write!(self.output, "<symbol> }} </symbol>")?,
            Symbol::LeftRoundBracket => write!(self.output, "<symbol> ( </symbol>")?,
            Symbol::RightRoundBracket => write!(self.output, "<symbol> ) </symbol>")?,
            Symbol::LeftSquareBracket => write!(self.output, "<symbol> [ </symbol>")?,
            Symbol::RightSquareBracket => write!(self.output, "<symbol> ] </symbol>")?,
            Symbol::Dot => write!(self.output, "<symbol> . </symbol>")?,
            Symbol::Comma => write!(self.output, "<symbol> , </symbol>")?,
            Symbol::Semicolon => write!(self.output, "<symbol> ; </symbol>")?,
            Symbol::Plus => write!(self.output, "<symbol> + </symbol>")?,
            Symbol::Minus => write!(self.output, "<symbol> - </symbol>")?,
            Symbol::Times => write!(self.output, "<symbol> * </symbol>")?,
            Symbol::Divide => write!(self.output, "<symbol> / </symbol>")?,
            Symbol::And => write!(self.output, "<symbol> &amp; </symbol>")?,
            Symbol::Or => write!(self.output, "<symbol> | </symbol>")?,
            Symbol::SmallerThan => write!(self.output, "<symbol> &lt; </symbol>")?,
            Symbol::GreaterThan => write!(self.output, "<symbol> &gt; </symbol>")?,
            Symbol::Equal => write!(self.output, "<symbol> = </symbol>")?,
            Symbol::Not => write!(self.output, "<symbol> ! </symbol>")?,
        }

        Ok(())
    }

    fn compile_integer_constant(tokenizer: &mut Tokenizer, output: &mut File) -> Result<()> {
        todo!();
    }

    fn compile_string_constant(tokenizer: &mut Tokenizer, output: &mut File) -> Result<()> {
        todo!();
    }

    fn compile_identifier(&mut self) -> Result<()> {
        match &self.current_token {
            Some(Token::Identifier(IdentifierValue { value })) => {
                write!(self.output, "<identifier> {} </identifier>", value)?;
                Ok(())
            }
            _ => bail!(
                "Expected to find identifier. But found this instead: {:?}.",
                self.current_token
            ),
        }
    }

    // Pogram structure
    fn compile_class(&mut self) -> Result<()> {
        write!(self.output, "<class>")?;

        self.compile_keyword(Keyword::Class)?;

        self.current_token = self.tokenizer.next();
        self.compile_identifier()?;

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::LeftCurlyBracket)?;

        self.current_token = self.tokenizer.next();

        while self.current_token == Some(Token::Keyword(Keyword::Static))
            || self.current_token == Some(Token::Keyword(Keyword::Field))
        {
            self.compile_classvardec()?;
            self.current_token = self.tokenizer.next();
        }

        while self.current_token == Some(Token::Keyword(Keyword::Constructor))
            || self.current_token == Some(Token::Keyword(Keyword::Function))
            || self.current_token == Some(Token::Keyword(Keyword::Method))
        {
            self.compile_subroutinedec()?;
            self.current_token = self.tokenizer.next();
        }

        self.compile_symbol(Symbol::RightCurlyBracket)?;

        write!(self.output, "</class>")?;
        Ok(())
    }

    fn compile_classvardec(&mut self) -> Result<()> {
        write!(self.output, "<classVarDec>")?;

        match self.current_token {
            Some(Token::Keyword(Keyword::Static)) => self.compile_keyword(Keyword::Static)?,
            Some(Token::Keyword(Keyword::Field)) => self.compile_keyword(Keyword::Field)?,
            _ => bail!(
                "Expected to find keyword static or field. But found this instead: {:?}.",
                self.current_token
            ),
        }

        self.current_token = self.tokenizer.next();
        self.compile_type()?;

        // one or more var name
        self.current_token = self.tokenizer.next();
        self.compile_identifier()?;

        self.current_token = self.tokenizer.next();

        while self.current_token == Some(Token::Symbol(Symbol::Comma)) {
            self.compile_symbol(Symbol::Comma)?;
            self.current_token = self.tokenizer.next();
            self.compile_identifier()?;
            self.current_token = self.tokenizer.next();
        }

        self.compile_symbol(Symbol::Semicolon)?;

        write!(self.output, "</classVarDec>")?;
        Ok(())
    }

    fn compile_type(&mut self) -> Result<()> {
        match &self.current_token {
            Some(Token::Keyword(Keyword::Int)) => {
                write!(self.output, "<type>")?;
                self.compile_keyword(Keyword::Int)?;
                write!(self.output, "</type>")?;
            }
            Some(Token::Keyword(Keyword::Char)) => {
                write!(self.output, "<type>")?;
                self.compile_keyword(Keyword::Char)?;
                write!(self.output, "</type>")?;
            }
            Some(Token::Keyword(Keyword::Boolean)) => {
                write!(self.output, "<type>")?;
                self.compile_keyword(Keyword::Boolean)?;
                write!(self.output, "</type>")?;
            }
            Some(Token::Identifier(_)) => {
                write!(self.output, "<type>")?;
                self.compile_identifier()?;
                write!(self.output, "</type>")?;
            }
            _ => bail!(
                "Expected to find type. But found this instead: {:?}.",
                self.current_token
            ),
        }

        Ok(())
    }

    fn compile_subroutinedec(&mut self) -> Result<()> {
        write!(self.output, "<subRoutineDec>")?;

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

        self.current_token = self.tokenizer.next();

        match &self.current_token {
            Some(Token::Keyword(Keyword::Void)) => self.compile_keyword(Keyword::Void)?,
            _ => self.compile_type()?,
        }

        self.current_token = self.tokenizer.next();
        self.compile_identifier()?;

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::LeftRoundBracket)?;

        self.current_token = self.tokenizer.next();
        self.compile_parameterlist()?;
        self.compile_symbol(Symbol::RightRoundBracket)?;

        self.current_token = self.tokenizer.next();
        self.compile_subroutinebody()?;

        write!(self.output, "</subRoutineDec>")?;

        Ok(())
    }

    fn compile_parameterlist(&mut self) -> Result<()> {
        write!(self.output, "<parameterList>")?;

        match self.compile_type() {
            Ok(_) => {
                self.current_token = self.tokenizer.next();
                self.compile_identifier()?;

                self.current_token = self.tokenizer.next();
                while self.current_token == Some(Token::Symbol(Symbol::Comma)) {
                    self.current_token = self.tokenizer.next();
                    self.compile_type()?;

                    self.current_token = self.tokenizer.next();
                    self.compile_identifier()?;
                }
            }
            Err(_) => {}
        };

        write!(self.output, "</parameterList>")?;

        Ok(())
    }

    fn compile_subroutinebody(&mut self) -> Result<()> {
        write!(self.output, "<subRoutineBody>")?;

        self.compile_symbol(Symbol::LeftCurlyBracket)?;

        self.current_token = self.tokenizer.next();

        while self.current_token == Some(Token::Keyword(Keyword::Var)) {
            self.compile_vardec()?;
            self.current_token = self.tokenizer.next();
        }

        while self.compile_statement().is_ok() {
            self.current_token = self.tokenizer.next();
        }

        self.compile_symbol(Symbol::RightCurlyBracket)?;

        write!(self.output, "</subRoutineBody>")?;

        Ok(())
    }

    fn compile_vardec(&mut self) -> Result<()> {
        write!(self.output, "<varDec>")?;

        self.compile_keyword(Keyword::Var)?;

        self.current_token = self.tokenizer.next();
        self.compile_type()?;

        self.current_token = self.tokenizer.next();
        self.compile_identifier()?;

        self.current_token = self.tokenizer.next();

        while self.current_token == Some(Token::Symbol(Symbol::Comma)) {
            self.compile_symbol(Symbol::Comma)?;
            self.current_token = self.tokenizer.next();

            self.compile_identifier()?;
            self.current_token = self.tokenizer.next();
        }

        self.compile_symbol(Symbol::Semicolon)?;

        write!(self.output, "</varDec>")?;

        Ok(())
    }

    // Statements
    fn compile_statement(&mut self) -> Result<()> {
        match self.current_token {
            Some(Token::Keyword(Keyword::Let)) => {
                write!(self.output, "<statement>")?;
                self.compile_let_statement()?;
                write!(self.output, "</statement>")?;
            }
            Some(Token::Keyword(Keyword::If)) => {
                write!(self.output, "<statement>")?;
                self.compile_if_statement()?;
                write!(self.output, "</statement>")?;
            }
            Some(Token::Keyword(Keyword::While)) =>{
                write!(self.output, "<statement>")?;
                self.compile_while_statement()?;
                write!(self.output, "</statement>")?;
            }
            Some(Token::Keyword(Keyword::Do)) => {
                write!(self.output, "<statement>")?;
                self.compile_do_statement()?;
                write!(self.output, "</statement>")?;
            }
            Some(Token::Keyword(Keyword::Return)) =>{
                write!(self.output, "<statement>")?;
                self.compile_return_statement()?;
                write!(self.output, "</statement>")?;
            }
            _ => bail!(
                "Expected to find 'let', 'if', 'while', 'do', or 'return'. But found this instead: {:?}.",
                self.current_token
            ),
        }

        Ok(())
    }

    fn compile_let_statement(&mut self) -> Result<()> {
        write!(self.output, "<letStatment>")?;

        self.compile_keyword(Keyword::Let)?;

        self.current_token = self.tokenizer.next();
        self.compile_identifier()?;

        //todo!("Compile array indexing")

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::Equal)?;

        self.current_token = self.tokenizer.next();
        self.compile_expression()?;

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::Semicolon)?;

        write!(self.output, "</letStatment>")?;

        Ok(())
    }

    fn compile_if_statement(&mut self) -> Result<()> {
        write!(self.output, "<ifStatement>")?;

        self.compile_keyword(Keyword::If)?;

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::LeftRoundBracket)?;

        self.current_token = self.tokenizer.next();
        self.compile_expression()?;

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::RightRoundBracket)?;

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::LeftCurlyBracket)?;

        while self.compile_statement().is_ok() {
            self.current_token = self.tokenizer.next();
        }

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::RightCurlyBracket)?;

        self.current_token = self.tokenizer.next();

        if self.current_token == Some(Token::Keyword(Keyword::Else)) {
            self.compile_keyword(Keyword::Else)?;

            self.current_token = self.tokenizer.next();
            self.compile_symbol(Symbol::LeftCurlyBracket)?;

            while self.compile_statement().is_ok() {
                self.current_token = self.tokenizer.next();
            }

            self.current_token = self.tokenizer.next();
            self.compile_symbol(Symbol::RightCurlyBracket)?;
        }

        write!(self.output, "</ifStatement>")?;

        Ok(())
    }

    fn compile_while_statement(&mut self) -> Result<()> {
        write!(self.output, "<whileStatment>")?;

        self.compile_keyword(Keyword::While)?;

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::LeftRoundBracket)?;

        self.current_token = self.tokenizer.next();
        self.compile_expression()?;

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::RightRoundBracket)?;

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::LeftCurlyBracket)?;

        while self.compile_statement().is_ok() {
            self.current_token = self.tokenizer.next();
        }

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::RightCurlyBracket)?;

        write!(self.output, "</whileStatment>")?;

        Ok(())
    }

    fn compile_do_statement(&mut self) -> Result<()> {
        write!(self.output, "<doStatement>")?;

        self.compile_keyword(Keyword::Do)?;

        self.current_token = self.tokenizer.next();
        self.compile_subroutine_call()?;

        self.current_token = self.tokenizer.next(); // probably remove after implementing compile_subroutine_call()
        self.compile_symbol(Symbol::Semicolon)?;

        write!(self.output, "</doStatement>")?;

        Ok(())
    }

    fn compile_return_statement(&mut self) -> Result<()> {
        write!(self.output, "<returnStatement>")?;

        self.compile_keyword(Keyword::Return)?;

        self.current_token = self.tokenizer.next();

        if self.compile_expression().is_ok() {
            self.current_token = self.tokenizer.next(); // probably remove after implementing compile_expression()
        }

        self.compile_symbol(Symbol::Semicolon)?;

        write!(self.output, "</returnStatement>")?;

        Ok(())
    }

    // Expressions
    fn compile_expression(&mut self) -> Result<()> {
        // todo!("Implement other cases");
        self.compile_identifier()
    }

    fn compile_term(&mut self) -> Result<()> {
        todo!();
    }

    fn compile_subroutine_call(&mut self) -> Result<()> {
        write!(self.output, "<subroutineCall>")?;

        self.compile_identifier()?;

        self.current_token = self.tokenizer.next();

        if self.current_token == Some(Token::Symbol(Symbol::Dot)) {
            self.compile_symbol(Symbol::Dot)?;

            self.current_token = self.tokenizer.next();
            self.compile_identifier()?;
            self.current_token = self.tokenizer.next();
        }

        self.compile_symbol(Symbol::LeftRoundBracket)?;

        self.current_token = self.tokenizer.next();
        self.compile_expression_list()?;

        self.compile_symbol(Symbol::RightRoundBracket)?;

        write!(self.output, "</subroutineCall>")?;

        Ok(())
    }

    fn compile_expression_list(&mut self) -> Result<()> {
        write!(self.output, "<expressionList>")?;

        match self.compile_expression() {
            Ok(_) => {
                self.current_token = self.tokenizer.next();
                while self.current_token == Some(Token::Symbol(Symbol::Comma)) {
                    self.current_token = self.tokenizer.next();
                    self.compile_expression()?;
                }
            }
            Err(_) => {}
        };

        write!(self.output, "</expressionList>")?;

        Ok(())
    }

    fn compile_op(&mut self) -> Result<()> {
        todo!();
    }

    fn compile_unary_op(&mut self) -> Result<()> {
        todo!();
    }

    fn compile_keyword_constant(&mut self) -> Result<()> {
        todo!();
    }
}
