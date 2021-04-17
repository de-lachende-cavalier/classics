use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

pub struct Deck {
    layout: Vec<u32>, // represent the cards by their bridge suits (1..=54, with 53 and 54 representing the two jokers)
                      // first joker = 53
                      // second joker = 54
}

impl Deck {
    pub fn new() -> Self {
        let init_state = (1..=54).collect::<Vec<u32>>();

        Deck { layout: init_state }
    }

    pub fn shuffle(&mut self, seed: &str) {
        let mut rng: Pcg64 = Seeder::from(seed).make_rng();
        // not using a CSPRNG (to make it more realistic)
        // Schneier suggests shuffling at list six times
        for _ in 0..6 {
            self.layout.shuffle(&mut rng);
        }
    }

    pub fn triple_cut(&mut self) {
        let mut fj_idx = 99; // fj = first joker
        let mut sj_idx = 99; // sj = second joker

        // look for the first joker
        // look for the second joker (the one after the first, ignore the values)
        for (i, card) in self.layout.iter().enumerate() {
            if *card == 53 || *card == 54 {
                if fj_idx == 99 {
                    // this is the first joker
                    fj_idx = i;
                } else if sj_idx == 99 {
                    // this is the second joker
                    sj_idx = i;
                }
            }

            if fj_idx != 99 && sj_idx != 99 {
                // no point in wasting instructions
                break;
            }
        }

        let cur_layout = &self.layout;
        let (above_first, rest) = cur_layout.split_at(fj_idx);
        let (mid, below_second) = rest.split_at(sj_idx - 2);

        let mut new_layout: Vec<u32> = Vec::new();

        // swap the cards above the first joker with the ones below the second joker
        new_layout.append(&mut Vec::from(below_second));
        new_layout.append(&mut Vec::from(mid));
        new_layout.append(&mut Vec::from(above_first));

        // sanity check
        assert_eq!(cur_layout.len(), new_layout.len());

        self.layout = new_layout;
    }

    // TODO => look into a way to abstract the actual process of the cut (from "let cur_layout..."
    // onwards (it's very similar to above)
    pub fn count_cut(&mut self) {
        let bottom_card = self.layout.pop().unwrap();
        // if the bottom card is the joker there's no need to cut
        if bottom_card == 53 || bottom_card == 54 {
            return;
        }

        let cur_layout = &self.layout;
        let (above_card, rest) = cur_layout.split_at(bottom_card as usize - 1); // -1 cause i popped the bottom card

        let mut new_layout: Vec<u32> = Vec::new();

        new_layout.append(&mut Vec::from(rest));
        new_layout.append(&mut Vec::from(above_card));
        new_layout.push(bottom_card); // the bottom card must stay at the bottom

        // sanity check
        assert_eq!(cur_layout.len(), new_layout.len());

        self.layout = new_layout;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // checks whether the deck is as we expect
    // aka, if it contains all the cards 1..=54
    fn is_proper_deck(deck: &Deck) -> bool {
        let mut s = HashSet::new();

        for n in 1..=54 {
            s.insert(deck.layout.contains(&n));
        }

        // a single false means that a value is missing
        // => the deck is not "proper"
        if s.contains(&false) {
            false
        } else {
            true
        }
    }

    #[test]
    fn test_shuffle() {
        let mut layouts: Vec<Vec<u32>> = Vec::new();
        let seeds = vec!["one", "two", "testing"];

        for s in seeds.iter() {
            let mut deck = Deck::new();
            deck.shuffle(s);

            assert!(is_proper_deck(&deck));

            layouts.push(deck.layout);
        }

        for (s, l) in seeds.iter().zip(layouts) {
            let mut deck = Deck::new();
            deck.shuffle(s);

            assert_eq!(l, deck.layout);
        }
    }

    #[test]
    fn test_triple_cut() {
        let mut deck = Deck::new();
        let old_layout = deck.layout.clone();

        deck.layout.swap(3, 53); // put the second joker in position 3 (swap it with 4)
        assert_eq!(deck.layout[3], 54);
        assert_eq!(deck.layout[53], 4);
        assert_eq!(deck.layout[52], 53); // the first joker is in the second to last position
        // layout: [1, 2, 3, 54, 5, 6 ..., 53, 4]

        deck.triple_cut(); // layout: [4, 54, 5, 6, 7 ..., 53, 1, 2, 3]

        assert_ne!(old_layout, deck.layout);
        assert!(is_proper_deck(&deck));

        let mut expected: Vec<u32> = Vec::new();
        expected.push(4); // layout: [4]
        expected.push(54); // layout: [4, 54]
        // try to recreate the expected situation manually
        let mut first = (5..=53).collect::<Vec<u32>>(); // [4, 54, 5, 6, 7 ..., 53]
        let mut second = (1..=3).collect::<Vec<u32>>(); // [1, 2, 3]

        expected.append(&mut first);
        expected.append(&mut second);

        // XXX test passes but it highlighted a bunch of mistakes in my code
        // => add some more tests to see if i fixed them for good
        assert_eq!(expected, deck.layout);
    }
}
