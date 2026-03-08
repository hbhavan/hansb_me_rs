use std::{char, fs::File, io::Read};
use crate::utils::{seq::*, menu::*};

#[derive(Clone, PartialEq)]
pub struct MarkdownDown {
    pub content: Seq<Block>,
}

#[derive(Clone, PartialEq)]
pub struct Block {
    block_content: Seq<Text>,
    block_type: BlockType
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockType {
    BlockQuote,
    CodeBlock,
    CodeSnippet,
    Header(usize),
    HorizontalRule,
    OrderedList(usize),
    LineBreak,
    Paragraph,
    UnorderedList(usize),
    Whitespace
}

#[derive(Clone, PartialEq)]
pub struct Text {
    text_content: String,
    text_type: TextType
}

#[derive(Clone, Debug, PartialEq)]
pub enum TextType {
    Bold,
    BoldItalic,
    Code,
    Image(String),
    Italic,
    Link(String),
    Normal,
    Struck,
    Whitespace
}

impl MarkdownDown {

}

impl Block {
    pub fn parse_string(content: &String) -> Seq<Block> {
        let lines = content
            .split('\n')
            .map(|l| l.to_string())
            .collect();

        Self::aggregate(Seq::from_vec(lines))
    }

    fn aggregate(lines: Seq<String>) -> Seq<Block> {
        Self::aggregate_rec(Seq::new(), Seq::new(), lines)
    }

    fn aggregate_rec(acc: Seq<Block>, curr: Seq<String>, rest: Seq<String>) -> Seq<Block> {

    }
}

impl Text {
    
}

impl BlockType {
    pub fn from_line(
        line: &String,
        curr: BlockType,
        block_content: Seq<Text>) -> (Self, String) {
        use BlockType::*;

        let chars = Seq::to_char_seq(line).to_slice();

        let (block_type, content) =  match (chars, curr) {
            (['#', '#', '#', '#', '#', '#', ' ', text @ ..], _) => (Header(6), text),
            (['#', '#', '#', '#', '#', ' ', text @ ..], _) => (Header(5), text),
            (['#', '#', '#', '#', ' ', text @ ..], _) => (Header(4), text),
            (['#', '#', '#', ' ', text @ ..], _) => (Header(3), text),
            (['#', '#', ' ', text @ ..], _) => (Header(2), text),
            (['#', ' ', text @ ..], _) => (Header(1), text),

            (['-', ' ', text @ ..], UnorderedList(n)) => (UnorderedList(n + 1), text),
            (['-', ' ', text @ ..], _) => (UnorderedList(1), text),

            (['+', ' ', text @ ..], OrderedList(n)) => (OrderedList(n + 1), text),
            (['+', ' ', text @ ..], _) => (OrderedList(1), text),

            (['`', '`', '`', text @ ..], CodeBlock) => (Paragraph, text),
            (['`', '`', '`', text @ ..], _) => (CodeBlock, text),
            ([text @ .., '`', '`', '`'], CodeBlock) => (CodeBlock, text),

            ([text @ ..], CodeBlock) => (CodeBlock, text),

            (['=', '=', '-', text @ ..], _) => (HorizontalRule, text),
        };


        (block_type, content.iter().collect())
    }
}

impl TextType {
    pub fn from_str() {
    }
}
