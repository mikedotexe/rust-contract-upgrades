use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, collections::Map, AccountId};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub name: String,
    pub map: Map<AccountId, String>,
    pub favorite_color: String,
}

impl Default for Contract {
    fn default() -> Self {
        env::panic(b"Contract must be initialized before usage with 'new' function call.")
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(name: String) -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            name,
            map: Map::new(b"m".to_vec()),
            favorite_color: "".to_string()
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, new_name: String) {
        self._only_owner_predecessor();

        self.name = new_name;
    }

    pub fn add_to_map(&mut self, account: AccountId, desc: String) {
        self.map.insert(&account, &desc);
    }

    pub fn get_map(&self) -> Vec<(String, String)> {
        self.map.to_vec()
    }

    pub fn get_map_len(&self) -> u64 {
        self.map.len()
    }

    pub fn bloat_map(&mut self) {
        self._only_owner_predecessor();
        for x in 0..190 {
            let key = format!("reasonably long time of string that'll take up some storage {}-{}", x, self.map.len());
            let val = "well shucks I guess this should be longer than a short word but nothing comes to mind.".to_string();
            self.map.insert(&key, &val);
        }
    }

    // Helper function checking for owner
    fn _only_owner_predecessor(&mut self) {
        assert_eq!(env::predecessor_account_id(), env::current_account_id(), "Only contract owner can sign transactions for this method.");
    }
}
