use std::{char, fs::File, io::Read, path::PathBuf};

use dioxus::{logger::tracing::info, prelude::*};

use super::{
    menu::{MenuItem, MenuMaker},
    seq::Seq,
};

pub struct Markdown {
    pub content: Seq<Paragraph>,
}

#[derive(Clone, Debug)]
pub enum Paragraph {
    Header(usize, Seq<Text>),
    CodeSnippet(Seq<Text>),
    CodeBlock(Seq<Seq<Text>>),
    OrderedList(Seq<Seq<Text>>),
    UnorderedList(Seq<Seq<Text>>),
    BlockQuote(Seq<Seq<Text>>),
    Para(Seq<Text>),
    HorizontalRule,
    LineBreak,
}

#[derive(Clone, Debug)]
pub enum Text {
    Italic(String),
    Bold(String),
    BoldItalic(String),
    Struck(String),
    Code(String),
    Link(String, String),
    Image(String, String),
    Normal(String),
}

#[allow(dead_code)]
impl Paragraph {
    pub fn get_paragraphs(markdown: &String) -> Seq<Paragraph> {
        let lines = markdown.split('\n').map(|l| l.to_string()).collect();

        Paragraph::accumulate(Seq::from_vec(lines))
    }

    fn accumulate(lines: Seq<String>) -> Seq<Paragraph> {
        Self::accumulate_rec(Seq::new(), Seq::new(), lines)
    }

    fn accumulate_rec(acc: Seq<Paragraph>, curr: Seq<String>, rest: Seq<String>) -> Seq<Paragraph> {
        use Paragraph::*;

        let append_para = |p: Paragraph| {
            Self::accumulate_rec(acc.clone().append_item(p), Seq::new(), rest.skip(1))
        };

        let append_line =
            |s: String| Self::accumulate_rec(acc.clone(), curr.append_item(s), rest.skip(1));

        let check_next = |c: char| {
            rest.iter()
                .nth(1)
                .is_some_and(|line| match Seq::to_char_seq(line).to_slice() {
                    [s, ' ', ..] if *s == c => true,
                    _ => false,
                })
        };

        let check_ol = || {
            rest.iter()
                .nth(1)
                .is_some_and(|line| match Seq::to_char_seq(line).to_slice() {
                    ['1'..'9', '1'..'9' | '.', ..] => true,
                    _ => false,
                })
        };

        match rest.first() {
            Some(line) => match Seq::to_char_seq(line).to_slice() {
                ['#', '#', '#', '#', '#', '#', ' ', text @ ..] => {
                    append_para(Header(6, Text::from_chars(text)))
                }
                ['#', '#', '#', '#', '#', ' ', text @ ..] => {
                    append_para(Header(5, Text::from_chars(text)))
                }
                ['#', '#', '#', '#', ' ', text @ ..] => {
                    append_para(Header(4, Text::from_chars(text)))
                }
                ['#', '#', '#', ' ', text @ ..] => append_para(Header(3, Text::from_chars(text))),
                ['#', '#', ' ', text @ ..] => append_para(Header(2, Text::from_chars(text))),
                ['#', ' ', text @ ..] => append_para(Header(1, Text::from_chars(text))),

                ['-', ' ', text @ ..] if check_next('-') => append_line(String::from_iter(text)),
                ['-', ' ', text @ ..] => append_para(UnorderedList(Text::from_seq(
                    curr.append_item(String::from_iter(text.iter())),
                ))),

                ['1'..'9', '.', text @ ..] if check_ol() => append_line(String::from_iter(text)),
                ['1'..'9', '.', text @ ..] => append_para(OrderedList(Text::from_seq(
                    curr.append_item(String::from_iter(text)),
                ))),

                ['`', '`', '`', text @ ..] if text.ends_with(&['`', '`', '`']) => {
                    append_para(CodeSnippet(Text::from_chars(&text[..text.len() - 3])))
                }

                ['`', '`', '`', text @ ..] if curr.len() == 0 => {
                    append_line(String::from_iter(text))
                }

                [text @ .., '`', '`', '`'] if curr.len() > 0 => append_para(CodeBlock(
                    Text::from_seq(curr.append_item(String::from_iter(text))),
                )),

                ['>', ' ', text @ ..] if check_next('>') => append_line(String::from_iter(text)),
                ['>', ' ', text @ ..] => append_para(BlockQuote(Text::from_seq(
                    curr.append_item(String::from_iter(text)),
                ))),

                ['=', '=', '=', ..] => append_para(HorizontalRule),

                [text @ ..] if text.len() == 0 => append_para(LineBreak),

                [text @ ..] if curr.len() > 0 => append_line(String::from_iter(text)),

                _ => append_para(Para(Text::from_string(line))),
            },
            None => match curr.len() > 0 {
                true => acc.append_item(Para(Text::from_string(&curr.to_string()))),
                false => acc,
            },
        }
    }
}

#[allow(dead_code)]
impl Text {
    pub fn from_seq(seq: Seq<String>) -> Seq<Seq<Text>> {
        let text: Vec<Seq<Text>> = seq.iter().map(|s| Self::from_string(s)).collect();

        Seq::from_vec(text)
    }

    pub fn from_chars(chars: &[char]) -> Seq<Text> {
        let s = String::from_iter(chars.iter());

        Self::from_string(&s)
    }

    pub fn from_string(line: &str) -> Seq<Text> {
        Self::to_text_rec(Seq::new(), Seq::new(), line, None)
    }

    pub fn get_text(&self) -> &str {
        use Text::*;

        match self {
            Normal(text) => text,
            Bold(text) => text,
            Italic(text) => text,
            BoldItalic(text) => text,
            Code(text) => text,
            Struck(text) => text,
            Link(text, _) => text,
            Image(text, _) => text,
        }
    }

    fn to_text_rec(acc: Seq<Text>, curr: Seq<char>, rest: &str, prefix: Option<char>) -> Seq<Text> {
        use Text::*;

        let trim = |c: char| curr.to_string().replace(c, "");

        let append_text =
            |text: Text| Self::to_text_rec(acc.append_item(text), Seq::new(), &rest[1..], None);

        let append_char =
            |c: char| Self::to_text_rec(acc.clone(), curr.append_item(c), &rest[1..], prefix);

        let append_curr = || match curr.len() {
            0 => acc.clone(),
            _ => acc.append_item(Normal(curr.to_string())),
        };

        let split_curr = |c: char| match curr.len() {
            0 => Self::to_text_rec(acc.clone(), Seq::single(c), &rest[1..], Some(c)),
            _ => Self::to_text_rec(
                acc.append_item(Normal(curr.to_string())),
                Seq::single(c),
                &rest[1..],
                Some(c),
            ),
        };

        let check_curr = |c: char, count: i32| {
            let (prefix, suffix, found) =
                curr.iter()
                    .map(|x| *x)
                    .fold((0, 1, false), |acc, x| match (x == c, acc.2) {
                        (true, false) => (acc.0 + 1, acc.1, acc.2),
                        (true, true) => (acc.0, acc.1 + 1, acc.2),
                        (false, false) => (acc.0, acc.1, true),
                        (_, _) => (acc.0, acc.1, acc.2),
                    });
            return prefix == count && suffix == count && found;
        };

        let check_prefix = |c: char| prefix.is_some_and(|p| p == c);

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
    }

    fn link(text: Seq<char>) -> Text {
        let line = text.to_string();

        let (title, source) = match line.strip_prefix('[').and_then(|s| s.split_once('(')) {
            Some((t, s)) => (t, s),
            None => ("", ""),
        };

        Text::Link(title.replace("]", ""), source.to_string())
    }

    fn image(text: Seq<char>) -> Text {
        let line = text.to_string();
        let (title, source) = match line.strip_prefix("![").and_then(|s| s.split_once('(')) {
            Some((t, s)) => (t, s),
            None => ("", ""),
        };

        Text::Image(title.replace("]", ""), source.to_string())
    }
}

#[allow(dead_code)]
impl Markdown {
    pub fn new(content: Seq<Paragraph>) -> Self {
        Self { content }
    }

    pub fn render_paragraph(paragraph: Paragraph) -> Element {
        use Paragraph::*;

        match paragraph {
            Header(size, text) => Self::render_header(size, text),
            CodeSnippet(text) => {
                rsx! { code { class: "md-code-snippet", {Self::render_texts(text)} } }
            }
            CodeBlock(block) => {
                rsx! { code { class: "md-code-block", {Self::render_block(block)} } }
            }
            OrderedList(list) => {
                rsx! { ol { class: "md-ol", {Self::render_list(list)} } }
            }
            UnorderedList(list) => {
                rsx! { ul { class: "md-ul", {Self::render_list(list)} } }
            }
            BlockQuote(block) => {
                rsx! { blockquote { class: "md-block-quoute", {Self::render_block(block)} } }
            }
            Para(texts) => {
                rsx! { p { class: "md-para", {Self::render_texts(texts)} } }
            }
            HorizontalRule => {
                rsx! { hr { class: "md-hr" } }
            }
            LineBreak => {
                rsx! { br {} }
            }
        }
    }

    fn render_texts(texts: Seq<Text>) -> Element {
        rsx! {
            for t in texts.iter() {
                {Self::render_text(t.clone())}
            }
        }
    }

    fn render_text(text: Text) -> Element {
        use Text::*;

        match text {
            Italic(text) => rsx! { em { class: "md-italic", {text} } },
            Bold(text) => rsx! { strong { class: "md-bold", {text} } },
            BoldItalic(text) => rsx! { em { strong { class: "md-bold-italic", {text} } } },
            Struck(text) => rsx! { s { class: "md-struck", {text} } },
            Code(text) => rsx! { code { class: "md-code", {text} } },
            Image(src, alt) => {
                rsx! { img { class: "md-img", src: src, alt: alt } }
            }
            Link(desc, href) => {
                rsx! { a { class: "md-link", href: href, {desc} } }
            }
            Normal(text) => rsx! { span { class: "md-text", {text} } },
        }
    }

    fn render_header(size: usize, text: Seq<Text>) -> Element {
        let id = text
            .iter()
            .map(Text::get_text)
            .collect::<Vec<_>>()
            .join("_")
            .replace(" ", "_");

        match size {
            1 => rsx! { h1 { class: "md-title", id: id, {Self::render_texts(text.clone())} } },
            2 => rsx! { h2 { class: "md-title", id: id, {Self::render_texts(text.clone())} } },
            3 => rsx! { h3 { class: "md-title", id: id, {Self::render_texts(text.clone())} } },
            4 => rsx! { h4 { class: "md-title", id: id, {Self::render_texts(text.clone())} } },
            5 => rsx! { h5 { class: "md-title", id: id, {Self::render_texts(text.clone())} } },
            6 => rsx! { h6 { class: "md-title", id: id, {Self::render_texts(text.clone())} } },
            _ => rsx! { p { class: "md-title", id: id, {Self::render_texts(text.clone())} } },
        }
    }

    fn render_list(items: Seq<Seq<Text>>) -> Element {
        rsx! {
            for item in items.iter() {
                li { class: "md-li", {Self::render_texts(item.clone())} }
            }
        }
    }

    fn render_block(items: Seq<Seq<Text>>) -> Element {
        rsx! {
            for item in items.iter() {
                span { {Self::render_texts(item.clone())} }
                br { }
            }
        }
    }

    pub fn example() -> Self {
        use Paragraph::*;
        use Text::*;

        let text = String::from("Test");

        let sample_text = Normal(text);

        let h1 = Header(1, Seq::from_vec(vec![sample_text.clone()]));
        let h2 = Header(2, Seq::from_vec(vec![sample_text.clone()]));
        let h3 = Header(3, Seq::from_vec(vec![sample_text.clone()]));
        let h4 = Header(4, Seq::from_vec(vec![sample_text.clone()]));
        let h5 = Header(5, Seq::from_vec(vec![sample_text.clone()]));
        let h6 = Header(6, Seq::from_vec(vec![sample_text.clone()]));

        let p = Para(Seq::from_vec(vec![sample_text.clone()]));

        let content = Seq::from_vec(vec![h1, h2, h3, h4, h5, h6, p]);

        Self { content }
    }

    pub fn parse_example() -> Self {
        let sample = "Parse example
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
            \n###### Test";

        let lines: Vec<String> = sample
            .split('\n')
            .filter(|l| l.trim().len() > 0)
            .map(|l| l.to_string())
            .collect();

        let content = Paragraph::accumulate(Seq::from_vec(lines));

        Self { content }
    }
}

impl MenuMaker for Markdown {
    fn to_menu(&self) -> Vec<MenuItem> {
        self.content
            .iter()
            .filter(|p| match p {
                Paragraph::Header(size, _) if *size == 2 => true,
                _ => false,
            })
            .map(|h| match h {
                Paragraph::Header(_, text) => MenuItem::from_markdown(text),
                _ => MenuItem::empty(),
            })
            .collect::<Vec<_>>()
    }
}
