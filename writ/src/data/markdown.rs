use std::{char};
use crate::utils::{seq::*};

#[derive(Clone, PartialEq)]
pub struct MarkdownContent {
    pub content: Seq<Block>,
}

#[derive(Clone, PartialEq)]
pub struct Block {
    block_content: Seq<Text>,
    pub block_type: BlockType
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
    pub text_type: TextType
}

#[derive(Clone, Debug, PartialEq)]
pub enum TextType {
    Bold,
    BoldItalic,
    Code,
    Image(String),
    Italic,
    LineBreak,
    Link(String),
    Normal,
    Struck,
    Whitespace
}

impl MarkdownContent {
    pub fn from_string(s: &String) -> Self {
        Self {
            content: Block::parse_string(s)
        }
    }

    pub fn parse_example() -> Self {
        let sample = String::from("Parse example
            \n*Italic*
            \n**Bold**
            \n***Bold Italic***
            \n~Struck~
            \n_Italic_
            \n__Bold__
            \n___Bold Italic___
            \n- Item *1*
            \n- Item ~2~
            \n- Item _3_
            \n[link title](link source)
            \n![img title](img source)
            \n> Block Quote
            \n> Block Quote
            \n> Block Quote
            \nVarious *Text* ~Example~ Line
            \nSample **text** sample ___text___ sample
            \n1. Item A
            \n2. Item B
            \n3. Item C
            \n===
            \n# Test
            \n## *Test*
            \n### ~Test~
            \n#### Test
            \n##### Test
            \n###### Test");

        Self {
            content: Block::parse_string(&sample)
        }
    }
}

impl Block {
    pub fn content(&self) -> Seq<Text> {
        self.block_content.clone()
    }

    pub fn parse_string(content: &String) -> Seq<Block> {
        let lines = content
            .split('\n')
            .map(|l| l.to_string())
            .collect();

        Self::aggregate(Seq::from_vec(lines))
    }

    fn aggregate(lines: Seq<String>) -> Seq<Block> {
        Self::aggregate_rec(Seq::new(), Self::empty(), lines)
    }

    fn aggregate_rec(
        acc: Seq<Block>,
        curr: Block,
        rest: Seq<String>) -> Seq<Block> {
        if let Some(line) = rest.first() {
            let (curr_new, next) = Self::from_line(&curr, &line);

            if let Some(block) = next {
                return Self::aggregate_rec(
                    acc.append_item(curr_new), 
                    block,
                    rest.skip(1))
            } else {
                return Self::aggregate_rec(
                    acc.append_item(curr_new.clone()), 
                    curr_new.clone(), 
                    rest.skip(1))
            }
        }

        acc
    }

    pub fn from_line<'a>(
        curr: &'a Block,
        line: &String) -> (Block, Option<Block>) {
        use BlockType::*;

        let (curr_new, next, content) =  match (Seq::to_char_seq(line).to_slice(), curr.block_type.clone()) {
            (['#', '#', '#', '#', '#', '#', ' ', text @ ..], _) => (curr, Some(Header(6)), Text::from_chars(text)),
            (['#', '#', '#', '#', '#', ' ', text @ ..], _) => (curr, Some(Header(5)), Text::from_chars(text)),
            (['#', '#', '#', '#', ' ', text @ ..], _) => (curr, Some(Header(4)), Text::from_chars(text)),
            (['#', '#', '#', ' ', text @ ..], _) => (curr, Some(Header(3)), Text::from_chars(text)),
            (['#', '#', ' ', text @ ..], _) => (curr, Some(Header(2)), Text::from_chars(text)),
            (['#', ' ', text @ ..], _) => (curr, Some(Header(1)), Text::from_chars(text)),

            (['-', ' ', text @ ..], UnorderedList(_)) => (&curr.append_line_break(), None, Text::from_chars(text)),
            (['-', ' ', text @ ..], _) => (curr, Some(UnorderedList(1)), Text::from_chars(text)),

            (['+', ' ', text @ ..], OrderedList(_)) => (&curr.append_line_break(), None, Text::from_chars(text)),
            (['+', ' ', text @ ..], _) => (curr, Some(OrderedList(1)), Text::from_chars(text)),

            (['`', '`', '`', text @ .., '`', '`', '`'], _) => (curr, Some(CodeSnippet), Text::from_chars(text)),

            (['`', '`', '`', text @ ..], CodeBlock) => (&curr.append_line_break(), Some(CodeBlock), Text::from_chars(text)),
            (['`', '`', '`', text @ ..], _) => (curr, Some(CodeBlock), Text::from_chars(text)),
            ([text @ .., '`', '`', '`'], CodeBlock) => (&curr.append_line_break(), None, Text::from_chars(text)),

            ([text @ ..], CodeBlock) => (&curr.append_line_break(), Some(CodeBlock), Text::from_chars(text)),

            (['>', ' ', text @ ..], BlockQuote) => (&curr.append_line_break(), None, Text::from_chars(text)),
            (['>', ' ', text @ ..], _) => (curr, Some(BlockQuote), Text::from_chars(text)),


            (['=', '=', '=', text @ ..], _) => (curr, Some(HorizontalRule), Text::from_chars(text)),

            ([text @ ..], _) => (curr, Some(Paragraph), Text::from_chars(text))
        };


        match next {
            Some(block_type) => (curr_new.clone(), Some(Block { block_type, block_content: content })),
            None => (curr_new.with(content), None)
        }
    }

    pub fn line_break() -> Self {
        Self {
            block_type: BlockType::LineBreak,
            block_content: Seq::new()
        }
    }

    pub fn empty() -> Self {
        Self {
            block_type: BlockType::Whitespace,
            block_content: Seq::new()
        }
    }

    pub fn with(&self, content: Seq<Text>) -> Self {
        Self {
            block_type: self.block_type.clone(),
            block_content: self.block_content.append(content)
        }
    }

    fn append_line_break(&self) -> Self {
        let block_type = match self.block_type {
            BlockType::OrderedList(n) => BlockType::OrderedList(n + 1),
            BlockType::UnorderedList(n) => BlockType::UnorderedList(n + 1),
            _ => self.block_type.clone()
        };

        Self {
            block_type,
            block_content: self.block_content.append_item(Text::line_break())
        }
    }
}

impl Text {
    pub fn text_content(&self) -> String {
        self.text_content.clone()
    }

    pub fn split_list_content(text: Seq<Text>) -> Seq<Seq<Text>> {
        text
            .iter()
            .fold((Seq::<Seq<Text>>::new(), Seq::<Text>::new()), |(acc, curr), next| match next.text_type {
                TextType::LineBreak => (acc.append_item(curr), Seq::new()),
                _ => (acc, curr.append_item(next.clone()))
            }).0
                
    }

    pub fn parse_chars(_chars: &[char]) -> Seq<Text> {
        Seq::new()
    }

    pub fn from_str(_s: &str) -> Seq<Text> {
        Seq::new()
    }

    pub fn from_chars<'a>(
        chars: &'a [char],
    ) -> Seq<Text> {
        Self::from_chars_rec(chars, Self::empty(), "")
    }

    pub fn from_chars_rec<'a>(
        acc: &[char],
        _curr: Self,
        _rest: &str
    ) -> Seq<Text> {
        let s = String::from_iter(acc.iter());

        Seq::single(Text { 
            text_type: TextType::Normal,
            text_content: s
        })

        /*
        match rest.chars().next() {
            Some(c) => match c {
                '*' | '_' | '`' | '~' | '!' | '[' if prefix.is_none() => split_curr(c),

                '*' | '_' if check_curr(c, 1) => append_text(Italic(trim(c))),
                '*' | '_' if check_curr(c, 2) => append_text(Bold(trim(c))),
                '*' | '_' if check_curr(c, 3) => append_text(BoldItalic(trim(c))),

                '`' if check_curr('`', 1) => append_text(Code(trim('`'))),
                '~' if check_curr('~', 1) => append_text(Struck(trim('~'))),

                ')' if check_prefix('[') => append_text(Text::link(curr)),
                ')' if check_prefix('!') => append_text(Text::image(curr)),

                '\n' => append_curr(),
                _ => append_char(c),
            },
            None => append_curr(),
        }
        */
        //Seq::new();
    }

    pub fn line_break() -> Self {
        Self {
            text_type: TextType::LineBreak,
            text_content: String::new()
        }
    }

    pub fn empty() -> Self {
        Self {
            text_type: TextType::Whitespace,
            text_content: String::new()
        }
    } 

    fn _check_curr(curr: &[char], c: char, count: i32) -> bool {
        let (prefix, suffix, found) = curr
            .iter()
            .map(|x| *x)
            .fold((0, 1, false), |acc, x| match (x == c, acc.2) {
                (true, false) => (acc.0 + 1, acc.1, acc.2),
                (true, true) => (acc.0, acc.1 + 1, acc.2),
                (false, false) => (acc.0, acc.1, true),
                (_, _) => (acc.0, acc.1, acc.2),
            });

        prefix == count && suffix == count && found
    }
}
