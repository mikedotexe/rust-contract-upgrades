use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize};
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

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Version2 {
    // add "favorite_color" in addition to Version 1's "name" variable
    pub favorite_color: String,
    pub favorite_musician: String,
}

// Used in Version 3
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Account {
    first_name: String,
    last_name: String,
    pronoun: String,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Version3 {
    // add "account" which uses Version 1's "name"
    pub account: Account,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum Version {
    V1(Version1),
    V2(Version2),
    V3(Version3),
}

// Used in get_all()
#[derive(BorshDeserialize, BorshSerialize, Serialize, Debug)]
pub struct AllValues {
    name: String,
    favorite_color: String,
    favorite_musician: String,
}

impl Version {
    pub fn get_version(&self) -> String {
        match *self {
            // Note that V1, V2, VX… are simple increasing numbers
            Version::V1(_) => {
                "0.0.1".to_string()
            },
            Version::V2(_) => {
                "0.0.2".to_string()
            },
            Version::V3(_) => {
                "0.1.0".to_string()
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
            _ => env::panic(self._error_retrieving_version().as_bytes())
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
            _ => env::panic(self._error_retrieving_version().as_bytes())
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
            _ => env::panic(self._error_retrieving_version().as_bytes())
        };
        version_one.map.to_vec()
    }

    pub fn get_map_len(&self) -> u64 {
        let version_one = match self.versions.get(0).unwrap() {
            Version::V1(Version1{map, ..}) => Version1 {
                map: map,
                name: self._empty_string(),
            },
            _ => env::panic(self._error_retrieving_version().as_bytes())
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
            _ => env::panic(self._error_retrieving_version().as_bytes())
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
            _ => env::panic(self._error_retrieving_version().as_bytes())
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

    /* Start Version 2 work */

    /// Write a (transient) custom upgrade script here.
    /// After this function is executed it can be deleted and the contract redeployed if desired.
    pub fn add_v2_with_color(&mut self, favorite_color: String) {
        self._only_owner_predecessor();

        let v2 = Version2 {
            favorite_color,
            favorite_musician: self._empty_string() // Info we haven't collected yet
        };
        self.versions.push(&Version::V2(v2));
        // Update the index of the current version
        self.current_version_index = self.versions.len() - 1;
    }

    // Custom getter ("favorite_color" exists in Version2)
    pub fn get_favorite_color(&self) -> String {
        let version_two = match self.versions.get(1).unwrap() {
            // use .. here because we don't care about the value of any variables after
            Version::V2(Version2{favorite_color, ..}) => Version2 {
                favorite_color: favorite_color.to_string(),
                favorite_musician: self._empty_string()
            },
            _ => env::panic(self._error_retrieving_version().as_bytes())
        };
        version_two.favorite_color
    }

    // Custom setter ("favorite_color" exists in Version2)
    pub fn set_favorite_color(&mut self, new_color: String) {
        self._only_owner_predecessor();
        // TODO try this pattern
        // https://blog.rust-lang.org/2015/04/17/Enums-match-mutation-and-moves.html
        // or
        // https://stackoverflow.com/q/37267060/711863
        let mut version_two = match self.versions.get(1).unwrap() {
            Version::V2(Version2{favorite_color, favorite_musician}) => Version2 {
                favorite_color: favorite_color.to_string(),
                favorite_musician: favorite_musician.to_string()
            },
            _ => env::panic(self._error_retrieving_version().as_bytes())
        };
        version_two.favorite_color = new_color;
        self.versions.replace(1, &Version::V2(version_two));
    }

    // Custom getter ("favorite_musician" exists in Version2)
    pub fn get_favorite_musician(&self) -> String {
        let version_two = match self.versions.get(1).unwrap() {
            // use .. here because we don't care about the value of any variables after
            // note we can reorder the parameters even though favorite_color appears first in the declaration
            Version::V2(Version2{favorite_musician, ..}) => Version2 {
                favorite_color: self._empty_string(),
                favorite_musician: favorite_musician.to_string()
            },
            _ => env::panic(self._error_retrieving_version().as_bytes())
        };
        version_two.favorite_musician
    }

    // Custom setter ("favorite_musician" exists in Version2)
    // Trying to "ref" here, not sure if it's helpful
    pub fn set_favorite_musician(&mut self, new_musician: String) {
        self._only_owner_predecessor();
        let mut version_two = match self.versions.get(1).unwrap() {
            Version::V2(Version2{ref favorite_color, ref favorite_musician}) => Version2 {
                favorite_color: favorite_color.to_string(),
                favorite_musician: favorite_musician.to_string()
            },
            _ => env::panic(self._error_retrieving_version().as_bytes())
        };
        version_two.favorite_musician = new_musician;
        self.versions.replace(1, &Version::V2(version_two));
    }

    pub fn set_all(&mut self, new_name: String, new_color: String, new_musician: String) {
        self.set_name(new_name);
        self.set_favorite_color(new_color);
        self.set_favorite_musician(new_musician);
    }

    // pub fn get_all(&self) -> (String, String, String) {
    pub fn get_all(&self) -> AllValues {
        let name = self.get_name();
        let favorite_color = self.get_favorite_color();
        let favorite_musician = self.get_favorite_musician();

        AllValues {
            name,
            favorite_color,
            favorite_musician,
        }
    }

    fn _empty_string(&self) -> String {
        "".to_string()
    }

    /* End Version 2 work */

    /* Start Version 3 work */

    /// Write a (transient) custom upgrade script here.
    /// After this function is executed it can be deleted and the contract redeployed if desired.
    pub fn add_v3_and_migrate(&mut self) {
        self._only_owner_predecessor();

        let v1_name = self.get_name();
        let split_on_space: Vec<&str> = v1_name.split(' ').collect();
        let last_name = if split_on_space.len() > 1 {
            split_on_space[1].to_string()
        } else {
            self._empty_string()
        };

        let v3 = Version3 {
            account: Account {
                first_name: split_on_space[0].to_string(),
                last_name,
                pronoun: "".to_string()
            }
        };
        self.versions.push(&Version::V3(v3));
        self.current_version_index = self.versions.len() - 1;
    }

    /// Write a (transient) custom upgrade script here.
    /// After this function is executed it can be deleted and the contract redeployed if desired.
    pub fn remove_v1(&mut self) {
        self._only_owner_predecessor();

        // Note: this removes index 0 (Version 1, and swaps in the last item: Version 3
        self.versions.swap_remove(0);
        // Hence, we now set the index to 0
        self.current_version_index = 0;
    }

    /* End Version 3 work */
}