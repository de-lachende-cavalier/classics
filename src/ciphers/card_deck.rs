use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

pub struct Deck {
    layout: Vec<u32>, // represent the cards by their bridge suits
                      // A joker = 53
                      // B joker = 54
}

impl Deck {
    pub fn new() -> Self {
        let init_state = (1..=54).collect::<Vec<u32>>();

        Deck { layout: init_state }
    }

    /// Generates a single keystream value.
    ///
    /// This function must be repeated for each plaintext/ciphertext character.
    fn get_output_card(&mut self, seed: &str) -> u32 {
        self.key_deck(seed);
        self.swap_A_joker();
        self.swap_B_joker();
        self.triple_cut();
        self.count_cut();

        self.layout[self.layout[0] as usize]
    }

    /// Keys the deck.
    ///
    /// The keying is done in a different way than the one specified by Schneier (because
    /// my way is much simpler and probably makes for a much more secure randomization).
    fn key_deck(&mut self, seed: &str) {
        let mut rng: Pcg64 = Seeder::from(seed).make_rng();
        // not using a CSPRNG (to make it more realistic)
        // Schneier suggests shuffling at list six times
        for _ in 0..6 {
            self.layout.shuffle(&mut rng);
        }
    }

    /// First step of keystream preparation.
    ///
    /// We find the A joker (53) and swap it with the card beneath it.
    /// If the joker is at the bottom we swap it with the first card in the deck.
    #[allow(non_snake_case)]
    fn swap_A_joker(&mut self) {
        for (i, v) in self.layout.iter().enumerate() {
            if *v == 53 {
                if i == 53 {
                    // joker A at bottom
                    self.layout.swap(i, 0);
                } else {
                    // joker A somewhere in the middle
                    self.layout.swap(i, i + 1);
                }

                break;
            }
        }
    }

    /// Second step of keystream preparation.
    ///
    /// We find the B joker (53) and swap it with the card that is two cards beneath it.
    /// If the joker is the bottom card, move it below the second card of the deck.
    /// If the joker is the second to last card, move it below the top card of the deck.
    #[allow(non_snake_case)]
    fn swap_B_joker(&mut self) {
        for (i, v) in self.layout.iter().enumerate() {
            if *v == 54 {
                if i == 53 {
                    // joker B at bottom
                    self.layout.swap(i, 2);
                } else if i == 52 {
                    // joker B secont to last
                    self.layout.swap(i, 1);
                } else {
                    // joker B somewhere in the middle
                    self.layout.swap(i, i + 2);
                }

                break;
            }
        }
    }

    /// Performs a triple cut.
    ///
    /// Performing a triple cut means swapping the cards above the first joker (in order) with the
    /// cards below the second joker, while leaving the cards between the two jokers (joker
    /// included) in the same state.
    ///
    /// If our deck is represented as 'A J1 M J2 B', with A := cards above the first joker, J1
    ///                                                   M := cards between the two jokers
    ///                                                   B := cards below the second joker, J2
    /// Then, after a triple cut, the deck will appear as such: 'B J1 M J2 A'.
    fn triple_cut(&mut self) {
        let fj_idx; // fj = first joker
        let sj_idx; // sj = second joker

        let old_layout = &self.layout;

        fj_idx = Deck::find_first_joker_index(old_layout);
        let (above_first, rest) = old_layout.split_at(fj_idx);

        // XXX ugly but necessary (for now)
        let mut r_v = rest.to_vec();
        let joker = r_v.remove(0);

        sj_idx = Deck::find_first_joker_index(&r_v);
        let (mid, below_second) = r_v.split_at(sj_idx + 1);
        // XXX

        let mut new_layout: Vec<u32> = Vec::new();

        // swap the cards above the first joker with the ones below the second joker
        new_layout.append(&mut Vec::from(below_second));
        new_layout.push(joker);
        new_layout.append(&mut Vec::from(mid));
        new_layout.append(&mut Vec::from(above_first));

        // sanity check
        assert_eq!(new_layout.len(), 54);

        self.layout = new_layout;
    }

    // TODO => look into a way to abstract the actual process of the cut (from "let cur_layout..."
    // onwards (it's very similar to above))
    /// Performs a count cut.
    ///
    /// Performing a count cut means looking at the value of the bottom card (using the bridge
    /// order of suits), then counting down from the top card a number of cards equivalent to the
    /// bottom card's value and finally cutting after the card we arrived at, leaving the bottom
    /// card at the bottom. Put in another way, supposing the value of the bottom card is b, it
    /// means removing b cards from the top of the deck and putting them on the bottom (not after
    /// the bottom card though).
    ///
    /// If we represent our deck as 'c1, c2, c3, c4, ..., c53, 4'  with 4 being the value of the
    /// bottom card
    /// Then, after a count cut, the layout of the deck will be 'c5, c6, ..., c53, c1, ..., c4, 4'
    fn count_cut(&mut self) {
        // if the bottom card is the joker there's no need to cut
        if *self.layout.last().unwrap() == 53 || *self.layout.last().unwrap() == 54 {
            return;
        }

        let bottom_card = self.layout.pop().unwrap();

        let cur_layout = &self.layout;
        let (above_card, rest) = cur_layout.split_at(bottom_card as usize); // we have to cut AFTER the card

        let mut new_layout: Vec<u32> = Vec::new();

        new_layout.append(&mut Vec::from(rest));
        new_layout.append(&mut Vec::from(above_card));
        new_layout.push(bottom_card); // the bottom card must stay at the bottom

        // sanity check
        assert_eq!(new_layout.len(), 54);

        self.layout = new_layout;
    }

    /// Finds the index corresponding to the first occurence of the joker
    /// given a certain deck, regardless of whether the joker is A or B.
    fn find_first_joker_index(deck: &Vec<u32>) -> usize {
        let mut joker_idx: usize = 0;

        for (i, v) in deck.iter().enumerate() {
            // these are the two possible values of the jokers
            if *v == 53 || *v == 54 {
                joker_idx = i;
                break;
            }
        }

        joker_idx
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
    fn test_keying() {
        let mut layouts: Vec<Vec<u32>> = Vec::new();
        let seeds = vec!["one", "two", "testing"];

        for s in seeds.iter() {
            let mut deck = Deck::new();
            deck.key_deck(s);

            assert!(is_proper_deck(&deck));

            layouts.push(deck.layout);
        }

        for (s, l) in seeds.iter().zip(layouts) {
            let mut deck = Deck::new();
            deck.key_deck(s);

            assert_eq!(l, deck.layout);
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_swap_A_joker() {
        let mut deck = Deck::new();

        deck.layout.swap(52, 53);
        assert_eq!(deck.layout[53], 53);

        // joker A at bottom
        deck.swap_A_joker();
        assert_eq!(deck.layout[0], 53);
        assert_eq!(deck.layout[53], 1);

        // joker A at top
        deck.swap_A_joker();
        assert_eq!(deck.layout[1], 53);
        assert_eq!(deck.layout[0], 2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_swap_B_joker() {
        let mut deck = Deck::new();

        // joker B at bottom
        deck.swap_B_joker();
        assert_eq!(deck.layout[2], 54);
        assert_eq!(deck.layout[53], 3);

        // joker B second to last
        deck.layout.swap(2, 52);
        assert_eq!(deck.layout[52], 54);

        deck.swap_B_joker();
        assert_eq!(deck.layout[1], 54);
        assert_eq!(deck.layout[52], 2);

        // joker B at top
        deck.swap_B_joker();
        assert_eq!(deck.layout[3], 54);
        assert_eq!(deck.layout[1], 4);
    }

    #[test]
    fn test_triple_cut_above_below() {
        let mut deck = Deck::new();
        let old_layout = deck.layout.clone();

        deck.layout.swap(3, 53);

        assert_eq!(deck.layout[3], 54);
        assert_eq!(deck.layout[53], 4);
        assert_eq!(deck.layout[52], 53); // layout: [1, 2, 3, 54, 5, 6 ..., 53, 4]

        deck.triple_cut();

        assert_ne!(old_layout, deck.layout);
        assert!(is_proper_deck(&deck));

        let expected: Vec<u32> = vec![
            4, 54, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
            26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
            48, 49, 50, 51, 52, 53, 1, 2, 3,
        ];

        assert_eq!(expected, deck.layout);
    }

    #[test]
    fn test_triple_cut_single_above() {
        let mut deck = Deck::new();
        let old_layout = deck.layout.clone();

        deck.layout.swap(10, 53);
        deck.layout.swap(52, 53); // layout: [1, 2, 3 ..., 10, 54, 12, ..., 52, 11, 53]

        assert_eq!(deck.layout[10], 54);
        assert_eq!(deck.layout[52], 11);
        assert_eq!(deck.layout[53], 53);

        deck.triple_cut();

        assert_ne!(old_layout, deck.layout);
        assert!(is_proper_deck(&deck));

        let expected: Vec<u32> = vec![
            54, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
            33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 11, 53,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        ];

        assert_eq!(expected, deck.layout);
    }

    #[test]
    fn test_triple_cut_back_to_back_jokers() {
        let mut deck = Deck::new();
        let old_layout = deck.layout.clone();

        deck.layout.swap(17, 53);
        deck.layout.swap(18, 52); // layout: [1, 2, 3 ..., 17, 54, 53, ..., 52, 19, 18]

        assert_eq!(deck.layout[17], 54);
        assert_eq!(deck.layout[53], 18);
        assert_eq!(deck.layout[18], 53);
        assert_eq!(deck.layout[52], 19);

        deck.triple_cut();

        assert_ne!(old_layout, deck.layout);
        assert!(is_proper_deck(&deck));

        let expected: Vec<u32> = vec![
            20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
            42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 19, 18, 54, 53, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            10, 11, 12, 13, 14, 15, 16, 17,
        ];

        assert_eq!(expected, deck.layout);
    }

    #[test]
    fn test_count_cut() {
        let mut deck = Deck::new();
        let old_layout = deck.layout.clone();

        deck.layout.swap(5, 53); // layout: [1, 2, 3, 4, 5, 54, 7, 8, ..., 53, 6]

        assert_eq!(deck.layout[5], 54);
        assert_eq!(deck.layout[53], 6);

        deck.count_cut();

        assert_ne!(old_layout, deck.layout);
        assert!(is_proper_deck(&deck));

        let expected: Vec<u32> = vec![
            7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
            29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
            51, 52, 53, 1, 2, 3, 4, 5, 54, 6,
        ];

        assert_eq!(expected, deck.layout);
    }

    #[test]
    fn test_count_cut_joker_bottom() {
        let mut deck = Deck::new();

        // swap a bunch of cards but leave the jokers where they are
        deck.layout.swap(5, 7);
        deck.layout.swap(1, 19);
        deck.layout.swap(33, 49);

        let old_layout = deck.layout.clone();

        deck.count_cut();

        assert!(is_proper_deck(&deck));
        assert_eq!(deck.layout, old_layout);
    }
}
