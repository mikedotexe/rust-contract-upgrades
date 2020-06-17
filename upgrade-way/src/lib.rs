use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, collections::Vector, collections::Map, AccountId};
use core::fmt;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Version1 {
    pub name: String,
    pub map: Map<AccountId, String>,
}

impl fmt::Debug for Version1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Version1")
            .field("name", &self.name)
            .field("map", &self.map.to_vec())
            .finish()
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum Version {
    V1(Version1),
}

impl Version {
    pub fn get_version(&self) -> String {
        match *self {
            // Note that V1, V2, VX… are simple increasing numbers
            Version::V1(_) => {
                "0.0.1".to_string()
            },
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // https://github.com/near/near-sdk-rs/blob/master/near-sdk/src/collections/vector.rs
    versions: Vector<Version>,
    current_version_index: u64,
    // Consider adding other data structures that will never be changed/removed and scale significantly here?
    // (Under the assumption that perhaps the match on set_* calls may be at risk of bloating system)
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
        let initial_version = Version1 {
            name,
            map: Map::new(b"m".to_vec()),
        };
        let mut contract = Contract {
            versions: Vector::new(b"versions".to_vec()),
            current_version_index: 0,
        };
        contract.versions.push(&Version::V1(initial_version));
        contract
    }

    // Since it's possible that Vector will use swap_remove, we keep track with current_version_index
    pub fn get_current_version(&self) -> String {
        self.versions.get(self.current_version_index).unwrap().get_version()
    }

    pub fn log_version_data(&self, index: u64) {
        if self.versions.is_empty() || self.versions.len() - 1 < index {
            env::panic(b"Invalid index, buddy.")
        }
        let version = self.versions.get(index).unwrap();
        env::log(format!("State info for version at index {}: {:?}", index, version).as_bytes());
    }

    // Custom getter ("name" exists in Version1)
    pub fn get_name(&self) -> String {
        let version_one = match self.versions.get(0).unwrap() {
            Version::V1(Version1{name, ..}) => Version1 {
                name: name.to_string(),
                map: Map::default(),
            },
        };
        version_one.name
    }

    // Custom setter ("name" exists in Version1)
    pub fn set_name(&mut self, new_name: String) {
        self._only_owner_predecessor();

        let mut version_one = match self.versions.get(0).unwrap() {
            Version::V1(Version1{name, map}) => Version1 {
                name: name.to_string(),
                map,
            },
        };
        version_one.name = new_name;
        // actually write it to storage
        self.versions.replace(0, &Version::V1(version_one));
    }

    // Custom getter ("map" exists in Version1)
    pub fn get_map(&self) -> Vec<(String, String)> {
        let version_one = match self.versions.get(0).unwrap() {
            Version::V1(Version1{map, ..}) => Version1 {
                map: map,
                name: self._empty_string(),
            },
        };
        version_one.map.to_vec()
    }

    pub fn get_map_len(&self) -> u64 {
        let version_one = match self.versions.get(0).unwrap() {
            Version::V1(Version1{map, ..}) => Version1 {
                map: map,
                name: self._empty_string(),
            },
        };
        version_one.map.len()
    }

    // Custom setter ("map" exists in Version1)
    pub fn add_to_map(&mut self, account: AccountId, desc: String) {
        self._only_owner_predecessor();

        let mut version_one = match self.versions.get(0).unwrap() {
            Version::V1(Version1{name, map}) => Version1 {
                name: name.to_string(),
                map,
            },
        };
        version_one.map.insert(&account, &desc);
        // actually write it to storage
        self.versions.replace(0, &Version::V1(version_one));
    }

    pub fn bloat_map(&mut self) {
        self._only_owner_predecessor();

        let mut version_one = match self.versions.get(0).unwrap() {
            Version::V1(Version1{name, map}) => Version1 {
                name: name.to_string(),
                map,
            },
        };
        for x in 0..190 {
            let key = format!("reasonably long time of string that'll take up some storage {}-{}", x, version_one.map.len());
            let val = "well shucks I guess this should be longer than a short word but nothing comes to mind.".to_string();
            version_one.map.insert(&key, &val);
        }
        // actually write it to storage
        self.versions.replace(0, &Version::V1(version_one));
    }

    // Helper function checking for owner
    fn _only_owner_predecessor(&mut self) {
        assert_eq!(env::predecessor_account_id(), env::current_account_id(), "Only contract owner can sign transactions for this method.");
    }

    // Helper function for common error message
    fn _error_retrieving_version(&self) -> String {
        "Error retrieving version.".to_string()
    }

    fn _empty_string(&self) -> String {
        "".to_string()
    }
}