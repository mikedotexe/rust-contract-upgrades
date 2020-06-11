use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, collections::Vector};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/*
  Good links

  What is the syntax to match on a reference to an enum?
  - https://stackoverflow.com/a/36592628/711863

  Possibly consider adding "ref" in places
  https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/ref.html

*/

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Version1 {
    pub name: String,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Version2 {
    // add favorite_color in addition to Version 1's "name" variable
    pub favorite_color: String,
    pub favorite_musician: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum Version {
    V1(Version1),
    V2(Version2),
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
        }
    }
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // https://github.com/near/near-sdk-rs/blob/master/near-sdk/src/collections/vector.rs
    versions: Vector<Version>,
    // Consider adding other data structures that will never be changed/removed and scale significantly here
    // (Under the assumption that perhaps the match on set_* calls may be at risk of bloating system)
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(name: String) -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        let initial_version = Version1 {
            name
        };
        let mut contract = Contract {
            versions: Vector::new(b"versions".to_vec()),
            // versions: vec![Version::V1(initial_version)],
        };
        contract.versions.push(&Version::V1(initial_version));
        contract
    }

    pub fn get_current_version(&self) -> String {
        // We'll assume the "current version" is the last item in the Vec
        self.versions.get(self.versions.len() - 1).unwrap().get_version()
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
            Version::V1(Version1{name}) => Version1 {
                name: name.to_string()
            },
            _ => {
                env::panic(b"Error upgrading to version 2 when retrieving first version.")
            }
        };
        version_one.name
    }

    // Custom setter ("name" exists in Version1)
    pub fn set_name(&mut self, new_name: String) {
        self.only_owner_predecessor();

        let mut version_one = match self.versions.get(0).unwrap() {
            Version::V1(Version1{name}) => Version1 {
                name: name.to_string()
            },
            _ => {
                env::panic(b"Error upgrading to version 2 when retrieving first version.")
            }
        };
        version_one.name = new_name;
        // actually write it to storage
        self.versions.replace(0, &Version::V1(version_one));
    }

    // Helper function checking for owner
    fn only_owner_predecessor(&mut self) {
        assert_eq!(env::predecessor_account_id(), env::current_account_id(), "Only contract owner can sign transactions for this method.");
    }

    /* Version 2 work */

    /// Write a (transient) custom upgrade script here.
    /// After this function is executed it can be deleted and the contract redeployed if desired.
    pub fn add_v2_with_color(&mut self, favorite_color: String) {
        self.only_owner_predecessor();

        let v2 = Version2 {
            favorite_color,
            favorite_musician: self.empty_string() // Info we haven't collected yet
        };
        self.versions.push(&Version::V2(v2));
    }

    // Custom getter ("favorite_color" exists in Version2)
    pub fn get_favorite_color(&self) -> String {
        let version_two = match self.versions.get(1).unwrap() {
            // use .. here because we don't care about the value of any variables after
            Version::V2(Version2{favorite_color, ..}) => Version2 {
                favorite_color: favorite_color.to_string(),
                favorite_musician: self.empty_string()
            },
            _ => {
                env::panic(b"Error getting favorite color")
            }
        };
        version_two.favorite_color
    }

    // Custom setter ("favorite_color" exists in Version2)
    pub fn set_favorite_color(&mut self, new_color: String) {
        self.only_owner_predecessor();
        let mut version_two = match self.versions.get(1).unwrap() {
            Version::V2(Version2{favorite_color, favorite_musician}) => Version2 {
                favorite_color: favorite_color.to_string(),
                favorite_musician: favorite_musician.to_string()
            },
            _ => {
                env::panic(b"Error changing favorite color")
            }
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
                favorite_color: self.empty_string(),
                favorite_musician: favorite_musician.to_string()
            },
            _ => {
                env::panic(b"Error getting favorite musician")
            }
        };
        version_two.favorite_musician
    }

    // Custom setter ("favorite_musician" exists in Version2)
    // Trying to "ref" here, not sure if it's helpful
    pub fn set_favorite_musician(&mut self, new_musician: String) {
        self.only_owner_predecessor();
        let mut version_two = match self.versions.get(1).unwrap() {
            Version::V2(Version2{ref favorite_color, ref favorite_musician}) => Version2 {
                favorite_color: favorite_color.to_string(),
                favorite_musician: favorite_musician.to_string()
            },
            _ => {
                env::panic(b"Error changing favorite musician")
            }
        };
        version_two.favorite_musician = new_musician;
        self.versions.replace(1, &Version::V2(version_two));
    }

    pub fn set_all(&mut self, new_name: String, new_color: String, new_musician: String) {
        self.set_name(new_name);
        self.set_favorite_color(new_color);
        self.set_favorite_musician(new_musician);
    }

    pub fn get_all(&self) -> (String, String, String) {
        let name = self.get_name();
        let color = self.get_favorite_color();
        let musician = self.get_favorite_musician();
        // return tuple
        (name, color, musician)
    }

    fn empty_string(&self) -> String {
        "".to_string()
    }
}
