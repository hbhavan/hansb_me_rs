use std::{char, fs::File, io::Read};

use dioxus::prelude::*;
use writ::data::*;
use writ::utils::seq::*;

use super::Render;

impl Render for Markdown {
    fn render(&self) -> Element {
        rsx! {
            for paragraph in self.content.iter() {
                {render_paragraph(paragraph.clone())}
            }
        }
    }
}

fn render_paragraph(paragraph: Paragraph) -> Element {
    use Paragraph::*;

    match paragraph {
        Header(size, text) => render_header(size, text),
        CodeSnippet(text) => {
            rsx! {
                code { class: "md-code-snippet", {render_texts(text)} }
            }
        }
        CodeBlock(block) => {
            rsx! {
                code { class: "md-code-block", {render_block(block)} }
            }
        }
        OrderedList(list) => {
            rsx! {
                ol { class: "md-ol", {render_list(list)} }
            }
        }
        UnorderedList(list) => {
            rsx! {
                ul { class: "md-ul", {render_list(list)} }
            }
        }
        BlockQuote(block) => {
            rsx! {
                blockquote { class: "md-block-quoute", {render_block(block)} }
            }
        }
        Para(texts) => {
            rsx! {
                p { class: "md-para", {render_texts(texts)} }
            }
        }
        HorizontalRule => {
            rsx! {
                hr { class: "md-hr" }
            }
        }
        LineBreak => {
            rsx! {
                br {}
            }
        }
    }
}

fn render_texts(texts: Seq<Text>) -> Element {
    rsx! {
        for t in texts.iter() {
            {render_text(t.clone())}
        }
    }
}

fn render_text(text: Text) -> Element {
    use Text::*;

    match text {
        Italic(text) => rsx! {
            em { class: "md-italic", {text} }
        },
        Bold(text) => rsx! {
            strong { class: "md-bold", {text} }
        },
        BoldItalic(text) => rsx! {
            em {
                strong { class: "md-bold-italic", {text} }
            }
        },
        Struck(text) => rsx! {
            s { class: "md-struck", {text} }
        },
        Code(text) => rsx! {
            code { class: "md-code", {text} }
        },
        Image(src, alt) => {
            rsx! {
                img { class: "md-img", src: render_image(src), alt }
            }
        }
        Link(desc, href) => {
            rsx! {
                a { class: "md-link", href, {desc} }
            }
        }
        Normal(text) => rsx! {
            span { class: "md-text", {text} }
        },
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
        1 => rsx! {
            h1 { class: "md-title", id, {render_texts(text.clone())} }
        },
        2 => rsx! {
            h2 { class: "md-title", id, {render_texts(text.clone())} }
        },
        3 => rsx! {
            h3 { class: "md-title", id, {render_texts(text.clone())} }
        },
        4 => rsx! {
            h4 { class: "md-title", id, {render_texts(text.clone())} }
        },
        5 => rsx! {
            h5 { class: "md-title", id, {render_texts(text.clone())} }
        },
        6 => rsx! {
            h6 { class: "md-title", id, {render_texts(text.clone())} }
        },
        _ => rsx! {
            p { class: "md-title", id, {render_texts(text.clone())} }
        },
    }
}

fn render_list(items: Seq<Seq<Text>>) -> Element {
    rsx! {
        for item in items.iter() {
            li { class: "md-li", {render_texts(item.clone())} }
        }
    }
}

fn render_block(items: Seq<Seq<Text>>) -> Element {
    rsx! {
        for item in items.iter() {
            span { {render_texts(item.clone())} }
            br {}
        }
    }
}

fn render_image(src: String) -> String {
    let mut buf = String::from("");

    let res = match File::open(src).and_then(|mut f| f.read_to_string(&mut buf)) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e),
    };

    match res {
        Ok(r) => r,
        Err(e) => String::from(e.to_string()),
    }
}
