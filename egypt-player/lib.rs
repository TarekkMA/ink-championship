#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use player::{Player as TestPlayer, PlayerRef as TestPlayerRef};

#[ink::contract]
mod player {
    use common::*;
    use squink_splash::GameRef;
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    pub struct Player {
        game_contract: AccountId,
        dimensions: (u32, u32),
        empty_slots: Vec<(u32, u32)>,
    }

    impl Player {
        #[ink(constructor)]
        pub fn new(game_contract: AccountId, dimensions: (u32, u32)) -> Self {
            let mut empty_slots = Vec::new();
            for x in 0..dimensions.0 {
                for y in 0..dimensions.1 {
                    empty_slots.push((x, y));
                }
            }
            Self {
                game_contract,
                dimensions,
                empty_slots,
            }
        }

        /// This is the function that will be called during every game round.
        ///
        /// The function returns an `(x, y)` coordinate of the pixel which you
        /// want to color.
        ///
        /// # Notes
        ///
        /// The function signature `&mut self` is so that you can retain state
        /// in the contract's storage if you want to.
        ///
        /// The function can be named as you like, but it always needs to have
        /// a defined selector of `0`.
        #[ink(message, selector = 0)]
        pub fn your_turn(&mut self) -> Option<(u32, u32)> {
            if self.empty_slots.is_empty() {
                return None;
            }
            let mut turn: (u32, u32) = self.random_choice();

            while !self.check_field(turn.0, turn.1) {
                if self.empty_slots.is_empty() {
                    return None;
                }
                turn = self.random_choice();
            }

            return Some(turn);
        }

        #[ink(message)]
        pub fn reset_state(&mut self) {
            let mut empty_slots = Vec::new();
            for x in 0..self.dimensions.0 {
                for y in 0..self.dimensions.1 {
                    empty_slots.push((x, y));
                }
            }
            self.empty_slots = empty_slots;
        }

        fn random_choice(&mut self) -> (u32, u32) {
            let time = self.env().block_timestamp();
            // xor each byte of the timestamp to get a random number
            let random = time.to_le_bytes().iter().fold(0, |acc, &x| acc ^ x as u32);
            let index = random.rem_euclid(self.empty_slots.len() as u32);
            self.empty_slots.remove(index as usize)
        }

        fn check_field(&self, x: u32, y: u32) -> bool {
            let game: GameRef = ink::env::call::FromAccountId::from_account_id(self.game_contract);
            game.field(Field { x, y }).is_none()
        }
    }
}
