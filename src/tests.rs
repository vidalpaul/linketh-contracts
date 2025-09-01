#[cfg(test)]
mod tests {
    use super::*;
    use stylus_sdk::{testing::*, prelude::Erase};
    use alloc::string::ToString;
    use alloc::vec;
    use alloy_primitives::Address;
    use crate::Linketh;

    #[test]
    fn test_init_ens_registry() {
        let vm = TestVM::default();
        let mut contract = Linketh::from(&vm);
        
        // Test initialization with ENS registry address
        let registry_addr = Address::from([1u8; 20]);
        contract.init(registry_addr);
        
        assert_eq!(contract.ens_registry.get(), registry_addr);
    }

    #[test]
    fn test_profile_storage_operations() {
        let vm = TestVM::default();
        let mut contract = Linketh::from(&vm);
        
        // Set up test data
        let owner = Address::from([1u8; 20]);
        
        // Test direct storage operations
        let mut profile = contract.profiles.setter(owner);
        profile.ens.set_str("test.eth");
        profile.cid.set_str("QmTest123");
        profile.display_name.set_str("Test User");
        profile.avatar_cid.set_str("QmAvatar456");
        
        // Retrieve and verify
        let (ens, cid, display, avatar) = contract.get_profile(owner);
        assert_eq!(ens, "test.eth");
        assert_eq!(cid, "QmTest123");
        assert_eq!(display, "Test User");
        assert_eq!(avatar, "QmAvatar456");
    }

    #[test]
    fn test_profile_field_updates() {
        let vm = TestVM::default();
        let mut contract = Linketh::from(&vm);
        
        let owner = Address::from([1u8; 20]);
        
        // Create initial profile
        let mut profile = contract.profiles.setter(owner);
        profile.ens.set_str("test.eth");
        profile.cid.set_str("QmOld");
        profile.display_name.set_str("Old Name");
        profile.avatar_cid.set_str("QmOldAvatar");
        
        // Update individual fields
        profile.cid.set_str("QmNew");
        profile.display_name.set_str("New Name");
        profile.avatar_cid.set_str("QmNewAvatar");
        
        // Verify updates
        let (ens, cid, display, avatar) = contract.get_profile(owner);
        assert_eq!(ens, "test.eth");
        assert_eq!(cid, "QmNew");
        assert_eq!(display, "New Name");
        assert_eq!(avatar, "QmNewAvatar");
    }

    #[test]
    fn test_quick_links_storage() {
        let vm = TestVM::default();
        let mut contract = Linketh::from(&vm);
        
        let owner = Address::from([1u8; 20]);
        
        // Create quick links directly through storage
        let mut quick_links = contract.quick_links.setter(owner);
        
        // Add first link
        let mut link1 = quick_links.grow();
        link1.title.set_str("Twitter");
        link1.url.set_str("https://twitter.com/user");
        
        // Add second link
        let mut link2 = quick_links.grow();
        link2.title.set_str("GitHub");
        link2.url.set_str("https://github.com/user");
        
        // Retrieve and verify
        let retrieved_links = contract.get_quick_links(owner);
        assert_eq!(retrieved_links.len(), 2);
        assert_eq!(retrieved_links[0], ("Twitter".to_string(), "https://twitter.com/user".to_string()));
        assert_eq!(retrieved_links[1], ("GitHub".to_string(), "https://github.com/user".to_string()));
    }

    #[test]
    fn test_profile_field_clearing() {
        let vm = TestVM::default();
        let mut contract = Linketh::from(&vm);
        
        let owner = Address::from([1u8; 20]);
        
        // Create profile
        let mut profile = contract.profiles.setter(owner);
        profile.ens.set_str("test.eth");
        profile.cid.set_str("QmTest");
        profile.display_name.set_str("Test User");
        profile.avatar_cid.set_str("QmAvatar");
        
        // Clear fields using erase
        profile.ens.erase();
        profile.cid.erase();
        profile.display_name.erase();
        profile.avatar_cid.erase();
        
        // Verify fields are cleared
        let (ens, cid, display, avatar) = contract.get_profile(owner);
        assert_eq!(ens, "");
        assert_eq!(cid, "");
        assert_eq!(display, "");
        assert_eq!(avatar, "");
    }

    #[test]
    fn test_multiple_users_independence() {
        let vm = TestVM::default();
        let mut contract = Linketh::from(&vm);
        
        let user1 = Address::from([1u8; 20]);
        let user2 = Address::from([2u8; 20]);
        
        // Create profile for user1
        let mut profile1 = contract.profiles.setter(user1);
        profile1.ens.set_str("user1.eth");
        profile1.cid.set_str("QmUser1");
        profile1.display_name.set_str("User One");
        profile1.avatar_cid.set_str("QmAvatar1");
        
        // Create profile for user2
        let mut profile2 = contract.profiles.setter(user2);
        profile2.ens.set_str("user2.eth");
        profile2.cid.set_str("QmUser2");
        profile2.display_name.set_str("User Two");
        profile2.avatar_cid.set_str("QmAvatar2");
        
        // Verify both profiles exist independently
        let (ens1, cid1, _, _) = contract.get_profile(user1);
        let (ens2, cid2, _, _) = contract.get_profile(user2);
        
        assert_eq!(ens1, "user1.eth");
        assert_eq!(cid1, "QmUser1");
        assert_eq!(ens2, "user2.eth");
        assert_eq!(cid2, "QmUser2");
    }

    #[test]
    fn test_empty_profile_retrieval() {
        let vm = TestVM::default();
        let contract = Linketh::from(&vm);
        
        let non_existent_user = Address::from([99u8; 20]);
        
        // Get profile for non-existent user
        let (ens, cid, display, avatar) = contract.get_profile(non_existent_user);
        
        // Should return empty strings
        assert_eq!(ens, "");
        assert_eq!(cid, "");
        assert_eq!(display, "");
        assert_eq!(avatar, "");
        
        // Quick links should be empty
        let links = contract.get_quick_links(non_existent_user);
        assert_eq!(links.len(), 0);
    }

    #[test]
    fn test_ens_registry_initialization() {
        let vm = TestVM::default();
        let contract = Linketh::from(&vm);
        
        // Check that ENS registry starts as zero address
        assert!(contract.ens_registry.get().is_zero());
    }

    #[test]
    fn test_quick_links_empty_filtering() {
        let vm = TestVM::default();
        let mut contract = Linketh::from(&vm);
        
        let owner = Address::from([1u8; 20]);
        
        // Create quick links with one empty title
        let mut quick_links = contract.quick_links.setter(owner);
        
        // Add valid link
        let mut link1 = quick_links.grow();
        link1.title.set_str("GitHub");
        link1.url.set_str("https://github.com/user");
        
        // Add empty link (should be filtered out)
        let mut _link2 = quick_links.grow();
        // Don't set title and url, leaving them empty
        
        // Retrieve and verify only non-empty links are returned
        let retrieved_links = contract.get_quick_links(owner);
        assert_eq!(retrieved_links.len(), 1);
        assert_eq!(retrieved_links[0], ("GitHub".to_string(), "https://github.com/user".to_string()));
    }

    #[test]
    fn test_storage_string_operations() {
        let vm = TestVM::default();
        let mut contract = Linketh::from(&vm);
        
        let owner = Address::from([1u8; 20]);
        
        // Test various string operations
        let mut profile = contract.profiles.setter(owner);
        
        // Set and get operations
        profile.ens.set_str("example.eth");
        assert_eq!(profile.ens.get_string(), "example.eth");
        
        // Empty string
        profile.cid.set_str("");
        assert_eq!(profile.cid.get_string(), "");
        
        // Long string
        let long_string = "QmVeryLongIPFSHashThatCouldPotentiallyBeUsedForStoringLargeAmountsOfData";
        profile.display_name.set_str(long_string);
        assert_eq!(profile.display_name.get_string(), long_string);
        
        // Special characters
        profile.avatar_cid.set_str("QmAvatar-123_with.special/chars");
        assert_eq!(profile.avatar_cid.get_string(), "QmAvatar-123_with.special/chars");
    }

    #[test]
    fn test_vec_operations() {
        let vm = TestVM::default();
        let mut contract = Linketh::from(&vm);
        
        let owner = Address::from([1u8; 20]);
        
        let mut quick_links = contract.quick_links.setter(owner);
        
        // Test growing and shrinking
        assert_eq!(quick_links.len(), 0);
        
        let mut link1 = quick_links.grow();
        link1.title.set_str("Link1");
        link1.url.set_str("url1");
        assert_eq!(quick_links.len(), 1);
        
        let mut link2 = quick_links.grow();
        link2.title.set_str("Link2");
        link2.url.set_str("url2");
        assert_eq!(quick_links.len(), 2);
        
        // Shrink
        quick_links.shrink();
        assert_eq!(quick_links.len(), 1);
        
        // Verify remaining link
        let remaining_link = quick_links.get(0).unwrap();
        assert_eq!(remaining_link.title.get_string(), "Link1");
        assert_eq!(remaining_link.url.get_string(), "url1");
    }
}