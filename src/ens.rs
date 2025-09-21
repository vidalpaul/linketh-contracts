//! ENS (Ethereum Name Service) integration module

use alloc::vec::Vec;
use alloc::string::String;
use stylus_sdk::prelude::*;
use alloy_primitives::keccak256;

sol_interface! {
    interface ENS {
        function owner(bytes32 node) external returns (address);
    }
}

pub fn namehash(name: String) -> [u8;32] {
    let mut node = [0u8;32]; // root
    if name.is_empty() { 
        return node; 
    }

    let labels: Vec<&str> = name.split('.').rev().collect();
    for label in labels {
        let label_hash = keccak256(label.as_bytes());
        let mut combined = Vec::with_capacity(64);
        combined.extend_from_slice(&node);
        combined.extend_from_slice(label_hash.as_ref());
        node = keccak256(&combined).into();
    }
    node
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_namehash_empty_string() {
        let result = namehash("".to_string());
        let expected = [0u8; 32];
        assert_eq!(result, expected, "Empty string should return zero hash");
    }

    #[test]
    fn test_namehash_eth() {
        // Test for "eth" domain
        let result = namehash("eth".to_string());
        // Known hash for "eth" from ENS spec
        let expected = [
            0x93, 0xcd, 0xeb, 0x70, 0x8b, 0x75, 0x45, 0xdc,
            0x66, 0x8e, 0xb9, 0x28, 0x01, 0x76, 0x16, 0x9d,
            0x1c, 0x33, 0xcf, 0xd8, 0xed, 0x6f, 0x04, 0x69,
            0x0a, 0x0b, 0xcc, 0x88, 0xa9, 0x3f, 0xc4, 0xae
        ];
        assert_eq!(result, expected, "Hash for 'eth' doesn't match expected");
    }

    #[test]
    fn test_namehash_vitalik_eth() {
        // Test for "vitalik.eth" - verify it produces different hash from "eth"
        let vitalik_result = namehash("vitalik.eth".to_string());
        let eth_result = namehash("eth".to_string());
        
        // Should not be zero hash
        assert_ne!(vitalik_result, [0u8; 32], "Vitalik.eth should not produce zero hash");
        
        // Should be different from just "eth"
        assert_ne!(vitalik_result, eth_result, "vitalik.eth should differ from eth");
        
        // Should be deterministic
        let vitalik_result2 = namehash("vitalik.eth".to_string());
        assert_eq!(vitalik_result, vitalik_result2, "Same input should produce same hash");
    }

    #[test]
    fn test_namehash_subdomain() {
        // Test for multi-level subdomain "resolver.reverse.eth"
        let result = namehash("resolver.reverse.eth".to_string());
        // This tests that the function correctly handles multiple labels
        assert_ne!(result, [0u8; 32], "Subdomain should not return zero hash");
        
        // Verify that the hash is different from just "eth"
        let eth_hash = namehash("eth".to_string());
        assert_ne!(result, eth_hash, "Subdomain hash should differ from parent domain");
    }

    #[test]
    fn test_namehash_case_sensitivity() {
        // ENS namehash should be case-sensitive
        let lowercase = namehash("example.eth".to_string());
        let uppercase = namehash("EXAMPLE.eth".to_string());
        assert_ne!(lowercase, uppercase, "Namehash should be case-sensitive");
    }

    #[test]
    fn test_namehash_special_characters() {
        // Test with special characters (should still compute)
        let result = namehash("test-123.eth".to_string());
        assert_ne!(result, [0u8; 32], "Special characters should produce non-zero hash");
        
        // Test with numbers
        let result2 = namehash("123456.eth".to_string());
        assert_ne!(result2, [0u8; 32], "Numeric domains should produce non-zero hash");
    }

    #[test]
    fn test_namehash_consistency() {
        // Same input should always produce same output
        let name = "consistency.test.eth".to_string();
        let result1 = namehash(name.clone());
        let result2 = namehash(name.clone());
        assert_eq!(result1, result2, "Same input should produce same hash");
    }

    #[test]
    fn test_namehash_single_label() {
        // Test single label without TLD
        let result = namehash("localhost".to_string());
        assert_ne!(result, [0u8; 32], "Single label should produce non-zero hash");
    }

    #[test]
    fn test_namehash_many_subdomains() {
        // Test deeply nested subdomains
        let result = namehash("a.b.c.d.e.f.eth".to_string());
        assert_ne!(result, [0u8; 32], "Deep subdomain should produce non-zero hash");
        
        // Each additional subdomain should change the hash
        let result2 = namehash("b.c.d.e.f.eth".to_string());
        assert_ne!(result, result2, "Different subdomain levels should produce different hashes");
    }
}