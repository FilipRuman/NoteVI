use std::fs;

use crate::{
    lexer::{
        lexer::tokenize,
        tokens::{Token, TokenKind},
    },
    logger::Logger,
};

use super::{
    actions::{action::Action, parser},
    key_handler::Shortcut,
};

const CONFIG_PATH: &str = "./keybindings.json";

pub struct Parser {
    pub index: usize,
    pub tokens: Vec<Token>,
}

const TAKEN_ARRAY_LENGTH_CHECK_SAFETY_CHECK: bool = false;
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { index: 0, tokens }
    }
    pub fn get_token(&self, index: usize) -> &Token {
        if TAKEN_ARRAY_LENGTH_CHECK_SAFETY_CHECK && index >= self.tokens.len() {
            panic!(
                "index:{} was greater than tokens array length:{} !",
                index,
                self.tokens.len()
            );
        }

        &self.tokens[index]
    }
    pub fn advance(&mut self) -> &Token {
        self.index += 1;
        &self.get_token(self.index - 1)
    }
    pub fn current_token(&self) -> &Token {
        &self.get_token(self.index)
    }
    pub fn current_token_kind(&self) -> &TokenKind {
        &self.get_token(self.index).kind
    }
    pub fn expect(&mut self, expected: &TokenKind) -> &Token {
        let current = self.advance();
        if &current.kind == expected {
            return current;
        }

        Logger::default_log(format!(
            "##### Expected: {:?} but found: {:?} ",
            expected, current
        ));
        panic!();
    }
    pub fn expect_vec(&mut self, expected_vec: Vec<&TokenKind>) -> &Token {
        let current = self.advance();
        for expected in expected_vec.clone() {
            if &current.kind == expected {
                return current;
            }
        }
        Logger::default_log(format!(
            "##### Expected: {:?} but found: {:?} ",
            expected_vec, current
        ));
        panic!();
    }
}
pub struct ShortcutParsingOutput {
    pub normal: Vec<Shortcut>,
    pub insert: Vec<Shortcut>,
    pub visual: Vec<Shortcut>,
}
pub fn parse_shortcuts_to_key_handler() -> ShortcutParsingOutput {
    if fs::exists(CONFIG_PATH).is_err() {
        Logger::default_log("Couldn't find config file!".to_string());
    }

    let content = fs::read_to_string(CONFIG_PATH).expect("Couldn't find config file!");

    Logger::default_log("parse_shortcuts_to_key_handler".to_string());
    let tokens = tokenize(
        content,
        vec![
            TokenKind::WhiteSpace,
            TokenKind::Tab,
            TokenKind::Comment,
            TokenKind::NextLine,
        ],
    );

    Logger::default_log(format!("tokens: {:?}", tokens));
    let mut parser = Parser::new(tokens);

    let mut output = ShortcutParsingOutput {
        normal: Vec::new(),
        insert: Vec::new(),
        visual: Vec::new(),
    };
    while parser.current_token_kind() != &TokenKind::EndOfFile {
        let parsed_layer_output = parse_layer(&mut parser);

        match parsed_layer_output.0.as_str() {
            "visual" => output.visual = parsed_layer_output.1,
            "normal" => output.normal = parsed_layer_output.1,
            "insert" => output.insert = parsed_layer_output.1,

            default => {
                Logger::default_log(format!("layer name {:?} is not right", default));
                panic!();
            }
        };
    }

    return output;
}

fn parse_layer(parser: &mut Parser) -> (String, Vec<Shortcut>) {
    let layer_name = parser.expect(&TokenKind::Identifier).value.clone();
    let mut shortcuts = Vec::new();

    //move past {
    parser.expect(&TokenKind::OpenCurly);

    while parser.current_token_kind() != &TokenKind::CloseCurly {
        parse_single_shortcut(parser, &mut shortcuts);
    }

    parser.expect(&TokenKind::CloseCurly);
    return (layer_name, shortcuts);
}

fn parse_single_shortcut(parser: &mut Parser, shortcuts: &mut Vec<Shortcut>) {
    Logger::default_log("parse_single_shortcut".to_string());
    let line = parser.current_token().line;
    let keystrokes_string = parser.expect(&TokenKind::String).value.to_string();

    parser.expect(&TokenKind::Colon);
    let mut actions = Vec::new();
    parse_actions(parser, line, &mut actions);
    let actions = shortcuts.push(Shortcut::new_parse_keystrokes(&keystrokes_string, actions));

    parser.expect(&TokenKind::SemiColon);
}

fn parse_actions(parser: &mut Parser, line: u16, actions: &mut Vec<Action>) {
    while parser.current_token_kind() != &TokenKind::SemiColon {
        let name = parser.expect(&TokenKind::Identifier).value.to_owned();
        parser.expect(&TokenKind::OpenParen);
        let mut values = Vec::new();
        while parser.current_token_kind() != &TokenKind::CloseParen {
            if parser.current_token_kind() == &TokenKind::Comma {
                parser.advance();
                continue;
            }

            values.push(
                parser
                    .expect_vec(
                        [
                            &TokenKind::String,
                            &TokenKind::Number,
                            &TokenKind::Identifier,
                        ]
                        .to_vec(),
                    )
                    .value
                    .to_owned(),
            );
        }

        parser.expect(&TokenKind::CloseParen);

        actions.push(parser::complete_parsing_action(name, values, line));
    }
}
