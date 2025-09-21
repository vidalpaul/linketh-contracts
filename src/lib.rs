//!
//! linketh-contracts (wasm)
//!
//! The following WASM contract implements a Linktree-like DID profiles.
//!
//! Note: ...
//!
// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;

use stylus_sdk::{
    prelude::*,
    storage::{StorageString, StorageVec},
    alloy_sol_types::sol,
};
use alloy_primitives::Address;

mod ens;

#[cfg(test)]
mod tests;

// --- Event Definitions ---
sol! {
    event ProfileCreated(address indexed owner, string ens, string cid, string display_name, string avatar_cid);
    event ProfileUpdated(address indexed owner, string cid, string display_name, string avatar_cid);
    event ProfileDeleted(address indexed owner);
    event QuickLinksUpdated(address indexed owner);
}

// --- QuickLink and Profile Structures ---
sol_storage! {
    pub struct QuickLink {
        StorageString title;
        StorageString url;
    }
}

sol_storage! {
    pub struct Profile {
        StorageString ens;
        StorageString cid;
        StorageString display_name;
        StorageString avatar_cid;
    }
}

// --- Storage ---
sol_storage! {
    #[entrypoint]
    pub struct Linketh {
        address ens_registry;
        mapping(address => Profile) profiles;
        mapping(address => StorageVec<QuickLink>) quick_links;
    }
}

// --- Contract Implementation ---
#[public]
impl Linketh {

    // Initialize ENS registry address
    pub fn init(&mut self, ens_registry_address: Address) {
        self.ens_registry.set(ens_registry_address);
    }

    // Create profile
    pub fn create_profile(&mut self, ens_name: String, cid: String, display: String, avatar: String) {
        let sender = self.vm().msg_sender();
        let node = ens::namehash(ens_name.clone());
        let registry_addr = self.ens_registry.get();
        assert!(!registry_addr.is_zero(), "ENS registry not initialized");
        
        let ens_contract = ens::ENS::new(registry_addr);
        let ens_owner = ens_contract.owner(&mut *self, node.into()).unwrap();
        assert!(ens_owner == sender, "You do not own this ENS");

        let mut p = self.profiles.setter(sender);
        p.ens.set_str(&ens_name);
        p.cid.set_str(&cid);
        p.display_name.set_str(&display);
        p.avatar_cid.set_str(&avatar);

        log(self.vm(), ProfileCreated {
            owner: sender,
            ens: ens_name,
            cid,
            display_name: display,
            avatar_cid: avatar,
        });
    }

    // Update profile
    pub fn update_profile(&mut self, cid: String, display: String, avatar: String) {
        let sender = self.vm().msg_sender();
        let mut p = self.profiles.setter(sender);
        if !cid.is_empty() { p.cid.set_str(&cid); }
        if !display.is_empty() { p.display_name.set_str(&display); }
        if !avatar.is_empty() { p.avatar_cid.set_str(&avatar); }

        log(self.vm(), ProfileUpdated {
            owner: sender,
            cid,
            display_name: display,
            avatar_cid: avatar,
        });
    }

    // Delete profile
    pub fn delete_profile(&mut self) {
        let sender = self.vm().msg_sender();
        // Clear profile fields
        let mut p = self.profiles.setter(sender);
        p.ens.erase();
        p.cid.erase();
        p.display_name.erase();
        p.avatar_cid.erase();
        
        // Clear quick links
        let mut links = self.quick_links.setter(sender);
        for _ in 0..links.len() {
            links.shrink();
        }

        log(self.vm(), ProfileDeleted {
            owner: sender,
        });
    }

    // Set quick links
    pub fn set_quick_links(&mut self, links: Vec<(String,String)>) {
        let sender = self.vm().msg_sender();
        assert!(links.len() <= 5, "Max 5 quick links");

        let mut quick_links_vec = self.quick_links.setter(sender);
        // Clear existing links by shrinking to 0
        for _ in 0..quick_links_vec.len() {
            quick_links_vec.shrink();
        }
        
        for (t, u) in links.into_iter() {
            let mut link = quick_links_vec.grow();
            link.title.set_str(&t);
            link.url.set_str(&u);
        }

        log(self.vm(), QuickLinksUpdated {
            owner: sender,
        });
    }

    // Get profile
    pub fn get_profile(&self, owner: Address) -> (String, String, String, String) {
        let p = self.profiles.getter(owner);
        (p.ens.get_string(), p.cid.get_string(), p.display_name.get_string(), p.avatar_cid.get_string())
    }

    // Get quick links
    pub fn get_quick_links(&self, owner: Address) -> Vec<(String, String)> {
        let links = self.quick_links.getter(owner);
        let mut result = Vec::new();
        for i in 0..links.len() {
            let link = links.get(i).unwrap();
            let title = link.title.get_string();
            if !title.is_empty() {
                result.push((title, link.url.get_string()));
            }
        }
        result
    }

    // Transfer profile
    pub fn transfer_profile(&mut self, new_owner: Address) {
        let sender = self.vm().msg_sender();
        
        // Transfer profile data
        {
            let old_p = self.profiles.getter(sender);
            let ens_str = old_p.ens.get_string();
            let cid_str = old_p.cid.get_string();
            let display_str = old_p.display_name.get_string();
            let avatar_str = old_p.avatar_cid.get_string();
            
            let mut new_p = self.profiles.setter(new_owner);
            new_p.ens.set_str(&ens_str);
            new_p.cid.set_str(&cid_str);
            new_p.display_name.set_str(&display_str);
            new_p.avatar_cid.set_str(&avatar_str);
        }
        
        // Transfer quick links
        {
            let mut link_data = Vec::new();
            let old_links = self.quick_links.getter(sender);
            for i in 0..old_links.len() {
                let old_link = old_links.get(i).unwrap();
                link_data.push((old_link.title.get_string(), old_link.url.get_string()));
            }
            
            let mut new_links = self.quick_links.setter(new_owner);
            // Clear existing links
            for _ in 0..new_links.len() {
                new_links.shrink();
            }
            for (title, url) in link_data {
                let mut new_link = new_links.grow();
                new_link.title.set_str(&title);
                new_link.url.set_str(&url);
            }
        }
        
        // Clear sender's data
        {
            let mut p = self.profiles.setter(sender);
            p.ens.erase();
            p.cid.erase();
            p.display_name.erase();
            p.avatar_cid.erase();
        }
        
        {
            let mut links = self.quick_links.setter(sender);
            for _ in 0..links.len() {
                links.shrink();
            }
        }
    }
}
