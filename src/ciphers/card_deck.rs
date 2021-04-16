// XXX useful: https://stackoverflow.com/questions/48490049/how-do-i-choose-a-random-value-from-an-enum
enum Suits {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

enum Values {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    JokerA,
    JokerB,
}

struct Card(Values, Suits);

pub struct Deck {
    layout: String,
}

impl Deck {
    fn new() -> Self {
        todo!();
    }
}
