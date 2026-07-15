use std::collections::VecDeque;

pub struct Set {
    pub deck: Deck,
    pub board: Vec<Card>,
    pub selected: VecDeque<usize>,
    pub score: u8,
    pub difficulty: Difficulty,
    game_state: GameState
}

pub struct Deck {
    pub cards: Vec<Card>
}

#[derive(Clone, Debug, PartialEq)]
pub struct Card {
    pub color: Color,
    pub texture: Texture,
    pub pattern: Pattern,
    pub quantity: u8
}

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Blue,
    Purple,
    Gold,
    Blank
}

#[derive(Clone, Debug, PartialEq)]
pub enum Texture {
    Circle,
    Hash,
    Slash,
    Blank
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    Cross,
    Diamond,
    Square,
    Blank
}

#[derive(Clone, PartialEq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard
}


#[derive(Clone)]
pub enum GameState {
    Ongoing(String),
    Concluded,
    Forfeit
}

impl Set {
    pub fn new(difficulty: Difficulty) -> Self {
        let mut deck = Deck::new();
        let mut board = vec![];
        let (board_width, board_height) = difficulty.get_board_size();

        deck.shuffle();

        for _ in 0..(board_width * board_height) {
            board.push(deck.draw());
        }

        Set {
            deck,
            selected: VecDeque::with_capacity(3),
            board,
            score: 0,
            difficulty,
            game_state: GameState::Ongoing(String::from("New game started"))
        }
    }

    pub fn select_card(&mut self, index: usize) {
        if !self.selected.contains(&index) && 
        let Some(card) = self.board.get(index) &&
        !card.is_blank() {
            self.selected.push_back(index)
        }

        if self.selected.len() > 3 {
            self.selected.pop_front();
        }
    }

    pub fn selection_valid(&self) -> bool {
        let cards: Vec<Card> = self.selected
            .iter()
            .map(|i| match self.board.get(*i) {
                Some(card) => card.clone(),
                None => Card::blank()
            })
            .collect();

        match cards.iter().as_slice() {
            [card_1, card_2, card_3] => Card::valid(card_1, card_2, card_3),
            _ => false
        }
    }

    pub fn submit(&mut self) {
        if self.selection_valid() {
            for i in self.selected.iter() {
                if let Some(card) = self.board.get_mut(*i) {
                    *card = self.deck.draw();
                } 
            }

            self.selected = VecDeque::with_capacity(2);
            self.score = self.score + 1;
        } else {
            self.game_state = GameState::Ongoing(String::from("Invalid submission"))
        }

        if self.board.iter().all(|c| c.is_blank()) {
            self.game_state = GameState::Concluded;
        }
    }

    pub fn forfeit(&mut self) {
        self.game_state = GameState::Forfeit
    }

    pub fn get_board_size(&self) -> (usize, usize) {
        self.difficulty.get_board_size()
    }

    pub fn game_state(&self) -> GameState {
        self.game_state.clone()
    }

    pub fn desc(&self) -> String {
        let deck_len = self.deck.len();
        let score = self.score;
        let selected: String = self.selected.iter().map(|i| i.to_string()).collect();

        format!("Remaining: {deck_len:<3}Score: {score:<3} Selected: {selected:<16}")
    }
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = vec![];

        for color in Color::colors() {
            for texture in Texture::textures() {
                for pattern in Pattern::patterns() {
                    for quantity in 1..=3 {
                        cards.push(Card::new(
                            color.clone(),
                            texture.clone(),
                            pattern.clone(),
                            quantity.clone()
                        ));
                    }
                }
            }
        }

        Deck { cards }
    }

    pub fn shuffled() -> Self {
        let mut deck = Deck::new();

        deck.shuffle();

        deck
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn shuffle(&mut self) {
        let len = self.cards.len();

        for _ in 0..1000 {
            let r_1 = rand::random_range(0..len);
            let r_2 = rand::random_range(0..len);

            self.cards.swap(r_1, r_2);
        }
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop().unwrap_or(Card::blank())
    }
}

impl Card {
    pub fn new(
        color: Color,
        texture: Texture,
        pattern: Pattern,
        quantity: u8
    ) -> Self {
        Card {
            color,
            texture,
            pattern,
            quantity
        }
    }

    pub fn valid(card_1: &Card, card_2: &Card, card_3: &Card) -> bool {
        !card_1.is_blank() && !card_2.is_blank() && !card_3.is_blank() &&
        attributes_valid(&card_1.color, &card_2.color, &card_3.color) &&
        attributes_valid(&card_1.texture, &card_2.texture, &card_3.texture) &&
        attributes_valid(&card_1.pattern, &card_2.pattern, &card_3.pattern) &&
        attributes_valid(&card_1.quantity, &card_2.quantity, &card_3.quantity)
    }

    pub fn to_string(&self) -> String {
        let s = (0..3).into_iter().map(|i| 
            format!("{row}\n", row = self.get_row(i)));

        String::from_iter(s)
    }

    pub fn get_row(&self, i: usize) -> String {
        self.pattern
            .get_pattern()
            .get(i)
            .map(|p| {
                let chars: Vec<char> = p
                    .iter()
                    .map(|x| match x {
                        1 => self.texture.clone(),
                        _ => Texture::Blank
                    }.get_texture())
                    .collect();

                let row = String::from_iter(chars);
                match self.quantity {
                    1 => format!("    {row}    \n"),
                    2 => format!("  {row} {row}  \n"),
                    3 => format!("{row} {row} {row}\n"),
                    _ => Card::blank_row()
                }
            })
            .unwrap_or(Card::blank_row())
    }

    pub fn desc(&self) -> String {
        let color = format!("{col:?}", col = self.color);
        let texture = format!("{txt:?}", txt = self.texture);
        let pattern = format!("{pat:?}", pat = self.pattern);
        let quantity = format!("{qnt}", qnt = self.quantity);

        format!("{color:<10}{texture:<10}{pattern:<10}{quantity}")
    }

    pub fn blank() -> Self {
        Card {
            color: Color::Blank,
            texture: Texture::Blank,
            pattern: Pattern::Blank,
            quantity: 0
        }
    }

    pub fn is_blank(&self) -> bool {
        self == &Card::blank()
    }

    pub fn blank_row() -> String {
        String::from("           \n")
    }
}

impl Color {
    pub fn get_color(&self) -> String {
        match self {
            Self::Blue => String::from("set-blue"),
            Self::Purple => String::from("set-purple"),
            Self::Gold => String::from("set-gold"),
            Self::Blank => String::from("set-blank")
        }
    }

    fn colors() -> Vec<Color> {
        vec![Color::Blue, Color::Purple, Color::Gold]
    }
}

impl Texture {
    pub fn get_texture(&self) -> char {
        match self {
            Self::Hash => '#',
            Self::Slash => '/',
            Self::Circle => 'O',
            Self::Blank => ' '
        }
    }

    fn textures() -> Vec<Texture> {
        vec![Texture::Circle, Texture::Hash, Texture::Slash]
    }
}

impl Pattern {
    pub fn get_pattern(&self) -> [[u8; 3]; 3] {
        match self {
            Self::Cross => Self::cross(),
            Self::Diamond => Self::diamond(),
            Self::Square => Self::square(),
            Self::Blank => Self::blank()
        }
    }

    fn cross() -> [[u8; 3]; 3] {
        [
            [1, 0, 1],
            [0, 1, 0],
            [1, 0, 1]
        ]
    }

    fn diamond() -> [[u8; 3]; 3] {
        [
            [0, 1, 0],
            [1, 1, 1],
            [0, 1, 0]
        ]
    }

    fn square() -> [[u8; 3]; 3] {
        [
            [1, 1, 1],
            [1, 1, 1],
            [1, 1, 1]
        ]
    }

    fn blank() -> [[u8; 3]; 3] {
        [
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0]
        ]
    }

    fn patterns() -> Vec<Pattern> {
        vec![Pattern::Cross, Pattern::Diamond, Pattern:: Square]
    }
}

impl Difficulty {
    pub fn get_difficulties() -> Vec<Difficulty> {
        vec![Difficulty::Easy, Difficulty::Normal, Difficulty::Hard]
    }

    pub fn to_string<'a>(&self) -> &'a str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Normal => "Normal",
            Difficulty::Hard => "Hard"
        }
    }

    pub fn get_board_size(&self) -> (usize, usize) {
        match self {
            Difficulty::Easy => (9, 9),
            Difficulty::Normal => (4, 5),
            Difficulty::Hard => (3, 4)
        }
    }
}

fn attributes_valid<T: PartialEq>(attr_1: &T, attr_2: &T, attr_3: &T) -> bool {
        (attr_1 == attr_2 && attr_2 == attr_3) ||
        (attr_1 != attr_2 && attr_2 != attr_3 && attr_1 != attr_3)
}
