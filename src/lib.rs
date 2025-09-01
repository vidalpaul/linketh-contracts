//!
//! vidalpaul@arb.soul.wasm
//!
//! The following contract implements a Linktree-like DID profile for my (https://github.com/vidalpaul) personal usage.
//!
//! Note: ...
//!
// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

use stylus_sdk::{
    prelude::*,
    storage::{StorageMap, StorageString},
    msg, evm,
};
use sha3::{Digest, Keccak256};

// --- QuickLink and Profile Structures ---
#[derive(SolidityAbi, Clone)]
pub struct QuickLink {
    pub title: StorageString,
    pub url: StorageString,
}

#[derive(SolidityAbi, Clone)]
pub struct Profile {
    pub ens: StorageString,
    pub cid: StorageString,
    pub display_name: StorageString,
    pub avatar_cid: StorageString,
}

// --- Storage ---
#[storage]
pub struct Linketh {
    profiles: StorageMap<Address, Profile>,
    quick_links: StorageMap<Address, [QuickLink; 5]>,
}

// --- ENS Interface ---
sol_interface! {
    interface ENS {
        fn owner(node: [u8;32]) -> Address;
    }
}

// --- Stylus contract implementation ---
#[public]
impl Linktree {

    // Create profile
    pub fn create_profile(&mut self, ens_name: String, cid: String, display: String, avatar: String) {
        let sender = msg::sender();
        let namehash = ens_namehash(ens_name.clone());
        let ens_owner = ENS::owner(namehash);
        assert!(ens_owner == sender, "You do not own this ENS");

        let mut p = self.profiles.setter(sender);
        p.ens.set(ens_name);
        p.cid.set(cid);
        p.display_name.set(display);
        p.avatar_cid.set(avatar);

        evm::log("ProfileCreated", (sender,));
    }

    // Update profile
    pub fn update_profile(&mut self, cid: Option<String>, display: Option<String>, avatar: Option<String>) {
        let sender = msg::sender();
        let mut p = self.profiles.getter(sender).clone();
        if let Some(c) = cid { p.cid.set(c); }
        if let Some(d) = display { p.display_name.set(d); }
        if let Some(a) = avatar { p.avatar_cid.set(a); }
        self.profiles.setter(sender).copy_from(&p);
        evm::log("ProfileUpdated", (sender,));
    }

    // Delete profile
    pub fn delete_profile(&mut self) {
        let sender = msg::sender();
        self.profiles.remove(sender);
        self.quick_links.remove(sender);
        evm::log("ProfileDeleted", (sender,));
    }

    // Set quick links (max 5)
    pub fn set_quick_links(&mut self, links: Vec<(String,String)>) {
        let sender = msg::sender();
        assert!(links.len() <= 5, "Max 5 quick links");

        let mut arr = [QuickLink{title: StorageString::from(""), url: StorageString::from("")}; 5];
        for (i, (t,u)) in links.into_iter().enumerate() {
            arr[i].title.set(t);
            arr[i].url.set(u);
        }

        self.quick_links.setter(sender).copy_from(&arr);
        evm::log("QuickLinksUpdated", (sender,));
    }

    // Get profile data
    pub fn get_profile(&self, owner: Address) -> (String, String, String, String) {
        let p = self.profiles.getter(owner);
        (p.ens.get(), p.cid.get(), p.display_name.get(), p.avatar_cid.get())
    }

    // Get quick links
    pub fn get_quick_links(&self, owner: Address) -> Vec<(String, String)> {
        let arr = self.quick_links.getter(owner);
        arr.iter()
           .filter(|q| q.title.len() > 0)
           .map(|q| (q.title.get(), q.url.get()))
           .collect()
    }

    // Transfer profile to new owner
    pub fn transfer_profile(&mut self, new_owner: Address) {
        let sender = msg::sender();
        let old_p = self.profiles.getter(sender).clone();
        self.profiles.setter(new_owner).copy_from(&old_p);

        let old_q = self.quick_links.getter(sender).clone();
        self.quick_links.setter(new_owner).copy_from(&old_q);
    }
}

// --- ENS Namehash function ---
pub fn ens_namehash(name: String) -> [u8;32] {
    let mut node = [0u8;32]; // initialize to 0 (root)
    if name.is_empty() { return node; }

    let labels: Vec<&str> = name.split('.').rev().collect();
    for label in labels {
        let mut hasher = Keccak256::new();
        let label_hash = Keccak256::digest(label.as_bytes());
        hasher.update(&node);
        hasher.update(&label_hash);
        node.copy_from_slice(&hasher.finalize());
    }
    node
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_profile() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Profile::from(&vm);

        /*
        assert_eq!(U256::ZERO, contract.number());

        contract.increment();
        assert_eq!(U256::from(1), contract.number());

        contract.add_number(U256::from(3));
        assert_eq!(U256::from(4), contract.number());

        contract.mul_number(U256::from(2));
        assert_eq!(U256::from(8), contract.number());

        contract.set_number(U256::from(100));
        assert_eq!(U256::from(100), contract.number());

        // Override the msg value for future contract method invocations.
        vm.set_value(U256::from(2));

        contract.add_from_msg_value();
        assert_eq!(U256::from(102), contract.number());
        */
    }
}