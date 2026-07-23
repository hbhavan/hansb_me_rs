use dioxus::prelude::*;
use writ::constants::ProgrammingLanguage;
use writ::data::*;
use writ::utils::seq::*;

use super::Render;

impl Render for MarkdownContent {
    fn render(&self) -> Element {
        rsx! {
            for block in self.content.iter() {
                {render_block(block)}
            }
        }
    }
}

fn render_block<'a>(block: &'a Block) -> Element {
    use BlockType::*;
    
    let content = block.content();

    match &block.block_type {
        BlockQuote =>  rsx! {
            blockquote { class: "md-block-quote", {render_content(&content)} }
        },
        CodeBlock(lang) => rsx! {
            code { class: vec!["md-code-block", get_lang_class(lang).as_str()].join(" "),
                {render_content(&content)}
            }
        },
        CodeSnippet => rsx! {
            code { class: "md-code-snippet", {render_content(&content)} }
        },
        Header(size) => render_header(*size, content),
        HorizontalRule => rsx! {
            hr { class: "md-hr" }
        },
        LineBreak => rsx! {
            br {}
        },
        OrderedList(_) => rsx! {
            ol { class: "md-ol", {render_list(content)} }
        },
        Paragraph => rsx! {
            p { class: "md-para", {render_content(&content)} }
        },
        UnorderedList(_) => rsx! {
            ul { class: "md-ol", {render_list(content)} }
        },
        Whitespace | Empty => rsx! {}
    }
}

fn render_content<'a>(text: &'a Seq<Text>) -> Element {
    rsx! {
        for t in text.iter() {
            {render_text(t)}
        }
    }
}

fn render_list(text: Seq<Text>) -> Element {
    let list_items = Text::split_list_content(text);

    rsx! {
        for list_item in list_items.iter() {
            li { class: "md-li", {render_content(list_item)} }
        }
    }
}

fn render_header(size: usize, text: Seq<Text>) -> Element {
    let id = text
        .iter()
        .map(|x| x.text_content())
        .collect::<Vec<_>>()
        .join("_")
        .replace(" ", "_");

    match size {
        1 => rsx! {
            h1 { class: "md-title", id, {render_content(&text)} }
        },
        2 => rsx! {
            h2 { class: "md-title", id, {render_content(&text)} }
        },
        3 => rsx! {
            h3 { class: "md-title", id, {render_content(&text)} }
        },
        4 => rsx! {
            h4 { class: "md-title", id, {render_content(&text)} }
        },
        5 => rsx! {
            h5 { class: "md-title", id, {render_content(&text)} }
        },
        6 => rsx! {
            h6 { class: "md-title", id, {render_content(&text)} }
        },
        _ => rsx! {
            p { class: "md-title", id, {render_content(&text)} }
        },
    }
}

fn render_text<'a>(text: &'a Text) -> Element {
    use TextType::*;

    let content = text.text_content();

    match text.text_type {
        Bold => rsx! {
            strong { class: "md-bold", {content} }
        },
        BoldItalic => rsx! {
            em {
                strong { class: "md-bold-italic", {content} }
            }
        },
        Code => rsx! {
            code { class: "md-code", {content} }
        },
        Image(_) => rsx! {
            img { src: "", alt: content }
        },
        Italic => rsx! {
            em { class: "md-italic", {content} }
        },
        LineBreak => rsx! {
            br {}
        },
        Link(_) => rsx! {
            a { class: "md-link", href: content }
        },
        Normal => rsx! {
            span { class: "md-text", {content} }
        },
        Struck => rsx! {
            s { class: "md-struck", {content} }
        },
        Whitespace => rsx! {}
    }
}

fn get_lang_class<'a>(language: &'a ProgrammingLanguage) -> String {
    let alias = language.get_alias();

    format!("langauge-{alias}")
}
