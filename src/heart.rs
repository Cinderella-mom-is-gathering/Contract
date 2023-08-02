use crate::*;

#[near_bindgen]
impl Contract {
    pub fn num_heart(&self, token_id: TokenId) -> u64 {
        self.heart.get(&token_id).unwrap().len()
    }

    pub fn is_heart_pressed(&self, token_id: TokenId) -> bool {
        self.heart
            .get(&token_id)
            .unwrap()
            .contains(&env::predecessor_account_id())
    }

    pub fn press_heart(&mut self, token_id: TokenId) {
        assert!(
            self.heart
                .get(&token_id)
                .unwrap()
                .contains(&env::predecessor_account_id()),
            "already pressed!"
        );
        self.heart
            .get(&token_id)
            .unwrap()
            .insert(&env::predecessor_account_id());
    }

    pub fn unpress_heart(&mut self, token_id:TokenId){
        assert!(
            !self.heart
                .get(&token_id)
                .unwrap()
                .contains(&env::predecessor_account_id()),
            "already unpressed!"
        );
        self.heart
            .get(&token_id)
            .unwrap()
            .remove(&env::predecessor_account_id());
    }
}
