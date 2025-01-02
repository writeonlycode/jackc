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
            Symbol::Not => write!(self.output, "<symbol> ! </symbol>\n")?,
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
                write!(self.output, "<identifier> {} </identifier>\n", value)?;
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
        write!(self.output, "<class>\n")?;

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

        write!(self.output, "</classVarDec>\n")?;
        Ok(())
    }

    fn compile_type(&mut self) -> Result<()> {
        match &self.current_token {
            Some(Token::Keyword(Keyword::Int)) => {
                //write!(self.output, "\n<type>\n")?;
                self.compile_keyword(Keyword::Int)?;
                //write!(self.output, "\n</type>\n")?;
            }
            Some(Token::Keyword(Keyword::Char)) => {
                //write!(self.output, "\n<type>\n")?;
                self.compile_keyword(Keyword::Char)?;
                //write!(self.output, "\n</type>\n")?;
            }
            Some(Token::Keyword(Keyword::Boolean)) => {
                //write!(self.output, "\n<type>\n")?;
                self.compile_keyword(Keyword::Boolean)?;
                //write!(self.output, "\n</type>\n")?;
            }
            Some(Token::Identifier(_)) => {
                //write!(self.output, "\n<type>\n")?;
                self.compile_identifier()?;
                //write!(self.output, "\n</type>\n")?;
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

        write!(self.output, "</subroutineDec>\n")?;

        Ok(())
    }

    fn compile_parameterlist(&mut self) -> Result<()> {
        write!(self.output, "<parameterList>\n")?;

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

        write!(self.output, "</parameterList>\n")?;

        Ok(())
    }

    fn compile_subroutinebody(&mut self) -> Result<()> {
        write!(self.output, "<subroutineBody>\n")?;

        self.compile_symbol(Symbol::LeftCurlyBracket)?;

        self.current_token = self.tokenizer.next();

        while self.current_token == Some(Token::Keyword(Keyword::Var)) {
            self.compile_vardec()?;
            self.current_token = self.tokenizer.next();
        }

        write!(self.output, "<statements>\n")?;

        while self.compile_statement().is_ok() {
            self.current_token = self.tokenizer.next();
        }

        write!(self.output, "</statements>\n")?;

        self.compile_symbol(Symbol::RightCurlyBracket)?;

        write!(self.output, "</subroutineBody>\n")?;

        Ok(())
    }

    fn compile_vardec(&mut self) -> Result<()> {
        write!(self.output, "<varDec>\n")?;

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

        write!(self.output, "</varDec>\n")?;

        Ok(())
    }

    // Statements
    fn compile_statement(&mut self) -> Result<()> {
        match self.current_token {
            Some(Token::Keyword(Keyword::Let)) => {
                //write!(self.output, "<statement>\n")?;
                self.compile_let_statement()?;
                //write!(self.output, "</statement>\n")?;
            }
            Some(Token::Keyword(Keyword::If)) => {
                //write!(self.output, "<statement>\n")?;
                self.compile_if_statement()?;
                //write!(self.output, "</statement>\n")?;
            }
            Some(Token::Keyword(Keyword::While)) =>{
                //write!(self.output, "<statement>\n")?;
                self.compile_while_statement()?;
                //write!(self.output, "</statement>\n")?;
            }
            Some(Token::Keyword(Keyword::Do)) => {
                //write!(self.output, "<statement>\n")?;
                self.compile_do_statement()?;
                //write!(self.output, "</statement>\n")?;
            }
            Some(Token::Keyword(Keyword::Return)) =>{
                //write!(self.output, "<statement>\n")?;
                self.compile_return_statement()?;
                //write!(self.output, "</statement>\n")?;
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

        self.current_token = self.tokenizer.next();
        self.compile_identifier()?;

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::Equal)?;

        self.current_token = self.tokenizer.next();
        self.compile_expression()?;

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::Semicolon)?;

        write!(self.output, "</letStatement>\n")?;

        Ok(())
    }

    fn compile_if_statement(&mut self) -> Result<()> {
        write!(self.output, "<ifStatement>\n")?;

        self.compile_keyword(Keyword::If)?;

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::LeftRoundBracket)?;

        self.current_token = self.tokenizer.next();
        self.compile_expression()?;

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::RightRoundBracket)?;

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::LeftCurlyBracket)?;

        write!(self.output, "<statements>\n")?;

        while self.compile_statement().is_ok() {
            self.current_token = self.tokenizer.next();
        }

        write!(self.output, "</statements>\n")?;

        self.current_token = self.tokenizer.next();
        self.compile_symbol(Symbol::RightCurlyBracket)?;

        self.current_token = self.tokenizer.next();

        if self.current_token == Some(Token::Keyword(Keyword::Else)) {
            self.compile_keyword(Keyword::Else)?;

            self.current_token = self.tokenizer.next();
            self.compile_symbol(Symbol::LeftCurlyBracket)?;

            write!(self.output, "<statements>\n")?;

            while self.compile_statement().is_ok() {
                self.current_token = self.tokenizer.next();
            }

            write!(self.output, "</statements>\n")?;

            self.current_token = self.tokenizer.next();
            self.compile_symbol(Symbol::RightCurlyBracket)?;
        }

        write!(self.output, "</ifStatement>\n")?;

        Ok(())
    }

    fn compile_while_statement(&mut self) -> Result<()> {
        write!(self.output, "<whileStatement>\n")?;

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

        write!(self.output, "</whileStatement>\n")?;

        Ok(())
    }

    fn compile_do_statement(&mut self) -> Result<()> {
        write!(self.output, "<doStatement>\n")?;

        self.compile_keyword(Keyword::Do)?;

        self.current_token = self.tokenizer.next();
        self.compile_subroutine_call()?;

        self.current_token = self.tokenizer.next(); // probably remove after implementing compile_subroutine_call()
        self.compile_symbol(Symbol::Semicolon)?;

        write!(self.output, "</doStatement>\n")?;

        Ok(())
    }

    fn compile_return_statement(&mut self) -> Result<()> {
        write!(self.output, "<returnStatement>\n")?;

        self.compile_keyword(Keyword::Return)?;

        self.current_token = self.tokenizer.next();

        if self.is_expression() {
            self.compile_expression();
            self.current_token = self.tokenizer.next();
        }

        self.compile_symbol(Symbol::Semicolon)?;

        write!(self.output, "</returnStatement>\n")?;

        Ok(())
    }

    // Expressions
    fn compile_expression(&mut self) -> Result<()> {

        println!("compiling expression: {:?}", &self.current_token);

        match &self.current_token {
            Some(Token::Identifier(value)) => {
                write!(self.output, "<expression>\n")?;
                self.compile_term()?;
                write!(self.output, "</expression>\n")?;
            }
            _ => {}

        }

        Ok(())
    }

    fn compile_term(&mut self) -> Result<()> {
        println!("compiling term: {:?}", &self.current_token);

        match &self.current_token {
            Some(Token::Identifier(value)) => {
                write!(self.output, "<term>\n")?;
                self.compile_identifier()?;
                write!(self.output, "</term>\n")?;
            }
            _ => {}

        }


        Ok(())
    }

    fn compile_subroutine_call(&mut self) -> Result<()> {
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

        Ok(())
    }

    fn compile_expression_list(&mut self) -> Result<()> {
        write!(self.output, "<expressionList>\n")?;

        if self.is_expression() {
            match self.compile_expression() {
                Ok(_) => {
                    self.current_token = self.tokenizer.next();
                    while self.current_token == Some(Token::Symbol(Symbol::Comma)) {
                        self.current_token = self.tokenizer.next();
                        self.compile_expression()?;
                    }
                }
                Err(_) => {}
            }
        }

        write!(self.output, "</expressionList>\n")?;

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

    // Utilitites
    fn is_expression(&self) -> bool {
        match &self.current_token {
            Some(Token::Identifier(value)) => true,
            _ => false
        }

    }

    fn is_statement(&self) -> bool {
        match self.current_token {
            Some(Token::Keyword(Keyword::Let)) => true,
            Some(Token::Keyword(Keyword::If)) => true ,
            Some(Token::Keyword(Keyword::While)) => true ,
            Some(Token::Keyword(Keyword::Do)) => true ,
            Some(Token::Keyword(Keyword::Return)) => true ,
            _ => false,
        }
    }
}
