use writ::{constants::ProgrammingLanguage, data::{BlockType::CodeBlock, *}, utils::seq::Seq};

#[test]
fn empty_content() {
    let md = MarkdownContent::from_str("");
    let content = Seq::single(Block::whitespace());

    let empty = MarkdownContent { content };

    assert_eq!(md, empty)
}

#[test]
fn headers() {
    let md_1 = MarkdownContent::from_str("# Test");
    let md_2 = MarkdownContent::from_str("## Test");
    let md_3 = MarkdownContent::from_str("### Test");
    let md_4 = MarkdownContent::from_str("#### Test");
    let md_5 = MarkdownContent::from_str("##### Test");
    let md_6 = MarkdownContent::from_str("###### Test");
    let md_7 = MarkdownContent::from_str("####### Test");

    let h1 = Seq::single(Block::new("Test", BlockType::Header(1)));
    let h2 = Seq::single(Block::new("Test", BlockType::Header(2)));
    let h3 = Seq::single(Block::new("Test", BlockType::Header(3)));
    let h4 = Seq::single(Block::new("Test", BlockType::Header(4)));
    let h5 = Seq::single(Block::new("Test", BlockType::Header(5)));
    let h6 = Seq::single(Block::new("Test", BlockType::Header(6)));
    let h7 = Seq::single(Block::new("####### Test", BlockType::Paragraph));

    assert_eq!(md_1, MarkdownContent { content: h1 });
    assert_eq!(md_2, MarkdownContent { content: h2 });
    assert_eq!(md_3, MarkdownContent { content: h3 });
    assert_eq!(md_4, MarkdownContent { content: h4 });
    assert_eq!(md_5, MarkdownContent { content: h5 });
    assert_eq!(md_6, MarkdownContent { content: h6 });
    assert_eq!(md_7, MarkdownContent { content: h7 });
}

#[test]
fn unordered_list() {
    let md_1 = MarkdownContent::from_str("- Test1");
    let md_2 = MarkdownContent::from_str("- Test1\n- Test2");

    let ul_1 = Seq::single(Block::new("Test1", BlockType::UnorderedList(1)));
    let ul_2 = Seq::single(Block::with_text(
        Seq::from_vec(vec![
            Text::new("Test1", TextType::Normal),
            Text::new("", TextType::LineBreak),
            Text::new("Test2", TextType::Normal)]), 
        BlockType::UnorderedList(2)));

    assert_eq!(md_1, MarkdownContent { content: ul_1 });
    assert_eq!(md_2, MarkdownContent { content: ul_2 });
}

#[test]
fn ordered_list_plus() {
    let md_1 = MarkdownContent::from_str("+ Test1");
    let md_2 = MarkdownContent::from_str("+ Test1\n+ Test2");

    let ol_1 = Seq::single(Block::new("Test1", BlockType::OrderedList(1)));
    let ol_2 = Seq::single(Block::with_text(
        Seq::from_vec(vec![
            Text::new("Test1", TextType::Normal),
            Text::new("", TextType::LineBreak),
            Text::new("Test2", TextType::Normal)]), 
        BlockType::OrderedList(2)));

    assert_eq!(md_1, MarkdownContent { content: ol_1 });
    assert_eq!(md_2, MarkdownContent { content: ol_2 });
}

#[test]
fn ordered_list_num() {
    let md_1 = MarkdownContent::from_str("1. Test1");
    let md_2 = MarkdownContent::from_str("1. Test1\n2. Test2");
    let md_3 = MarkdownContent::from_str("11. Test1\n22. Test2");

    let ol_1 = Seq::single(Block::new("Test1", BlockType::OrderedList(1)));
    let ol_2 = Seq::single(Block::with_text(
        Seq::from_vec(vec![
            Text::new("Test1", TextType::Normal),
            Text::line_break(),
            Text::new("Test2", TextType::Normal)]), 
        BlockType::OrderedList(2)));

    let ol_2_mock = MarkdownContent { content: ol_2 };

    assert_eq!(md_1, MarkdownContent { content: ol_1 });
    assert_eq!(&md_2, &ol_2_mock);
    assert_eq!(&md_3, &ol_2_mock);
}

#[test]
fn code_block() {
    let md_1 = MarkdownContent::from_str("```\nTest\n```");
    let md_2 = MarkdownContent::from_str("```Rust\nTest\n```");

    let cb_1 = Seq::single(Block::with_text(
        Seq::from_vec(vec![
            Text::new("Test", TextType::Normal),
            Text::line_break()]), 
        BlockType::CodeBlock(ProgrammingLanguage::None)));
    let cb_2 = Seq::single(Block::with_text(
        Seq::from_vec(vec![
            Text::new("Test", TextType::Normal),
            Text::line_break()]), 
        BlockType::CodeBlock(ProgrammingLanguage::Rust)));

    assert_eq!(md_1, MarkdownContent { content: cb_1 });
    assert_eq!(md_2, MarkdownContent { content: cb_2 });
}
