use writ::data::set::*;

#[test]
fn initialize_deck() {
    let deck = Deck::new();

    assert_eq!(deck.len(), 81)
}

#[test]
fn check_first_card() {
    let deck = Deck::new();

    let card_a = deck.cards.first();
    let card_b = first_card();

    assert_eq!(card_a, Some(&card_b))
}

#[test]
fn draw_from_new_deck() {
    let mut deck = Deck::new();
    let initial_len = deck.len();

    let card_a = deck.draw();
    let card_b = last_card();


    assert_eq!(deck.len(), initial_len - 1);
    assert_eq!(card_a, card_b);
}

#[test]
fn draw_from_empty_deck() {
    let mut deck = Deck {
        cards: vec![]
    };
    let initial_len = deck.len();

    let card_a = deck.draw();
    let card_b = Card::blank();


    assert_eq!(initial_len, 0);
    assert_eq!(deck.len(), 0);
    assert_eq!(card_a, card_b);
}

#[test]
fn initialize_set() {
    let set = Set::new(Difficulty::Normal);
    let (board_width, board_height) = set.get_board_size();

    assert_eq!(set.deck.len(), 81 - board_width * board_height);
    assert_eq!(set.board.len(), board_width * board_height);
    assert_eq!(set.score, 0);
    assert_eq!(set.selected.len(), 0);
}

#[test]
fn select_card() {
    let mut set = Set::new(Difficulty::Normal);

    set.select_card(0);

    assert_eq!(set.selected.len(), 1);
    assert_eq!(set.selected.get(0), Some(&0));
}

#[test]
fn select_3_cards() {
    let mut set = Set::new(Difficulty::Normal);

    set.select_card(0);
    set.select_card(1);
    set.select_card(2);

    assert_eq!(set.selected.len(), 3);
    assert_eq!(set.selected.get(0), Some(&0));
    assert_eq!(set.selected.get(1), Some(&1));
    assert_eq!(set.selected.get(2), Some(&2));
}

#[test]
fn select_4_cards() {
    let mut set = Set::new(Difficulty::Normal);

    set.select_card(0);
    set.select_card(1);
    set.select_card(2);
    set.select_card(3);

    assert_eq!(set.selected.len(), 3);
    assert_eq!(set.selected.get(0), Some(&1));
    assert_eq!(set.selected.get(1), Some(&2));
    assert_eq!(set.selected.get(2), Some(&3));
}

#[test]
fn select_duplicate_card() {
    let mut set = Set::new(Difficulty::Normal);

    set.select_card(0);
    set.select_card(1);
    set.select_card(2);
    set.select_card(2);

    assert_eq!(set.selected.len(), 3);
    assert_eq!(set.selected.get(0), Some(&0));
    assert_eq!(set.selected.get(1), Some(&1));
    assert_eq!(set.selected.get(2), Some(&2));
}

#[test]
fn attributes_valid_1() {
    let card_1 = Card::new(
        Color::Blue,
        Texture::Hash,
        Pattern::Diamond,
        3
    );
    let card_2 = Card::new(
        Color::Purple,
        Texture::Hash,
        Pattern::Diamond,
        3
    );
    let card_3 = Card::new(
        Color::Gold,
        Texture::Hash,
        Pattern::Diamond,
        3
    );

    let set = set_board(vec![card_1, card_2, card_3]);

    assert_eq!(set.selection_valid(), true)
}

#[test]
fn attributes_valid_2() {
    let card_1 = Card::new(
        Color::Blue,
        Texture::Slash,
        Pattern::Square,
        2
    );
    let card_2 = Card::new(
        Color::Blue,
        Texture::Circle,
        Pattern::Cross,
        2
    );
    let card_3 = Card::new(
        Color::Blue,
        Texture::Hash,
        Pattern::Diamond,
        2
    );

    let set = set_board(vec![card_1, card_2, card_3]);

    assert_eq!(set.selection_valid(), true)
}

#[test]
fn attributes_valid_3() {
    let card_1 = Card::new(
        Color::Blue,
        Texture::Hash,
        Pattern::Square,
        3
    );
    let card_2 = Card::new(
        Color::Gold,
        Texture::Circle,
        Pattern::Diamond,
        2
    );
    let card_3 = Card::new(
        Color::Purple,
        Texture::Slash,
        Pattern::Cross,
        1
    );

    let set = set_board(vec![card_1, card_2, card_3]);

    assert_eq!(set.selection_valid(), true)
}

#[test]
fn attributes_invalid_1() {
    let card_1 = Card::new(
        Color::Blue,
        Texture::Hash,
        Pattern::Diamond,
        3
    );
    let card_2 = Card::new(
        Color::Purple,
        Texture::Hash,
        Pattern::Diamond,
        3
    );
    let card_3 = Card::new(
        Color::Purple,
        Texture::Hash,
        Pattern::Diamond,
        1
    );

    let set = set_board(vec![card_1, card_2, card_3]);

    assert_eq!(set.selection_valid(), false)
}

#[test]
fn attributes_invalid_2() {
    let card_1 = Card::new(
        Color::Blue,
        Texture::Slash,
        Pattern::Square,
        2
    );
    let card_2 = Card::new(
        Color::Blue,
        Texture::Circle,
        Pattern::Cross,
        2
    );
    let card_3 = Card::new(
        Color::Gold,
        Texture::Hash,
        Pattern::Diamond,
        2
    );

    let set = set_board(vec![card_1, card_2, card_3]);

    assert_eq!(set.selection_valid(), false)
}

#[test]
fn attributes_invalid_3() {
    let card_1 = Card::new(
        Color::Blue,
        Texture::Hash,
        Pattern::Square,
        3
    );
    let card_2 = Card::new(
        Color::Gold,
        Texture::Circle,
        Pattern::Square,
        2
    );
    let card_3 = Card::new(
        Color::Purple,
        Texture::Circle,
        Pattern::Cross,
        1
    );

    let set = set_board(vec![card_1, card_2, card_3]);

    assert_eq!(set.selection_valid(), false)
}

#[test]
fn uniqueness() {
    let mut distinct: Vec<Card> = vec![];
    let mut duplicates: Vec<Card> = vec![];

    let mut deck = Deck::new();
    deck.shuffle();

    for card in deck.cards.iter() {
        if distinct.contains(card) {
            duplicates.push(card.clone());
        } else {
            distinct.push(card.clone());
        }
    }

    assert_eq!(distinct.len(), 81);
    assert_eq!(duplicates.len(), 0);
}

fn set_board(board: Vec<Card>) -> Set {
    let mut set = Set::new(Difficulty::Normal);

    set.board = board;
    set.select_card(0);
    set.select_card(1);
    set.select_card(2);

    set
}

fn first_card() -> Card {
    Card::new(
        Color::Blue, 
        Texture::Circle, 
        Pattern::Cross, 
        1)
}


fn last_card() -> Card {
    Card::new(
        Color::Gold,
        Texture::Slash,
        Pattern::Square,
        3)
}
