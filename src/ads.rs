use near_sdk::env::log_str;

use crate::*;

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonAd {
    token_id: TokenId,
    requester_id: AccountId,
    img: Vec<String>,
    description: String,
    cost: Balance,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Ad {
    pub img: Vec<String>,
    pub description: String,
    pub cost: Balance,
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn request_ad(&mut self, token_id: TokenId, description: String, img: Vec<String>) {
        if !self.requested_ads.contains_key(&token_id) {
            self.requested_ads.insert(
                token_id.clone(),
                store::UnorderedMap::new(
                    [
                        b"s".as_slice(),
                        &near_sdk::env::sha256_array(token_id.as_bytes()),
                    ]
                    .concat(),
                ),
            );
        }
        let ads = self.requested_ads.get_mut(&token_id).unwrap();
        Promise::new(env::current_account_id()).transfer(env::attached_deposit());
        let ad = Ad {
            img: img,
            description: description,
            cost: env::attached_deposit(),
        };
        if ads.contains_key(&env::predecessor_account_id()) {
            near_sdk::env::panic_str("already requested");
        }
        ads.insert(env::predecessor_account_id(), ad);
    }

    pub fn view_requested_ads(&self) -> Vec<JsonAd> {
        let mut ret = vec![];
        let account_id = env::predecessor_account_id();
        if self.tokens_per_owner.contains_key(&account_id) {
            for token_id in self.tokens_per_owner.get(&account_id).unwrap().iter() {
                if self.requested_ads.contains_key(&token_id) {
                    for ad in self.requested_ads.get(&token_id).unwrap().iter() {
                        ret.push(JsonAd {
                            token_id: token_id.clone(),
                            requester_id: ad.0.clone(),
                            description: ad.1.description.clone(),
                            img: ad.1.img.clone(),
                            cost: ad.1.cost.clone(),
                        });
                    }
                }
            }
        }
        ret
    }

    pub fn approve_ad(&mut self, token_id: TokenId, requester_id: AccountId) {
        assert!(self
            .tokens_per_owner
            .get(&env::predecessor_account_id())
            .unwrap()
            .contains(&token_id));
        log_str(&token_id);
        log_str(&requester_id.to_string());
        let ads = self.requested_ads.get_mut(&token_id).unwrap();
        let ad = ads.get_mut(&requester_id).unwrap().clone();

        self.approved_ad.insert(token_id.clone(), ad.img.clone());

        ads.remove(&requester_id);

        Promise::new(env::predecessor_account_id()).transfer(ad.cost);
    }

    pub fn deny_ad(&mut self, token_id: TokenId, requester_id: AccountId) {
        assert!(self
            .tokens_per_owner
            .get(&env::predecessor_account_id())
            .unwrap()
            .contains(&token_id));
        self.requested_ads
            .get_mut(&token_id)
            .unwrap()
            .remove(&requester_id);
    }

    pub fn get_ad(&self, token_id: TokenId) -> Vec<String> {
        if self.approved_ad.contains_key(&token_id) {
            self.approved_ad.get(&token_id).unwrap().clone()
        } else {
            vec![]
        }
    }
}
