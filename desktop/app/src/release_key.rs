// SPDX-License-Identifier: GPL-3.0-or-later
//! The key this build's updates are signed with.
//!
//! Each release carries `SHA256SUMS.txt` and `SHA256SUMS.txt.sig`, a raw
//! 64-byte Ed25519 signature of the sums file made with the private half of
//! [`PUBLIC_KEY`] (`openssl pkeyutl -sign -rawin`, in the release workflow's
//! signing job). The updater installs nothing whose sums don't carry a valid
//! signature, so a release asset swapped on GitHub, or a build pushed by
//! someone who can publish releases but doesn't hold the key, is refused.

use ed25519_dalek::{Signature, VerifyingKey};

/// zaiddia/NectarLink-ES's update signing key (Ed25519, raw 32 bytes).
pub const PUBLIC_KEY: [u8; 32] = [
    0xd1, 0x77, 0xff, 0x31, 0x67, 0xb7, 0xc0, 0xa7, 0x39, 0xe6, 0xb9, 0xa3, 0x79, 0x7a, 0x9c, 0x35, 0x5a, 0x6e,
    0x3f, 0x8f, 0x35, 0x78, 0xc1, 0x86, 0xf8, 0x64, 0x5e, 0x2b, 0xdb, 0xa7, 0xfd, 0xa7,
];

/// Name of the signature that goes with a release file.
pub fn signature_name(file: &str) -> String {
    format!("{file}.sig")
}

/// Whether `signature` (raw 64 bytes) is [`PUBLIC_KEY`]'s signature of `data`.
pub fn verify(data: &[u8], signature: &[u8]) -> bool {
    verify_with(&PUBLIC_KEY, data, signature)
}

fn verify_with(key: &[u8; 32], data: &[u8], signature: &[u8]) -> bool {
    let Ok(key) = VerifyingKey::from_bytes(key) else { return false };
    let Ok(bytes) = <[u8; 64]>::try_from(signature) else { return false };
    key.verify_strict(data, &Signature::from_bytes(&bytes)).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUMS: &[u8] = b"abc123  Nectarlink-9.9.9-x64-setup.exe\n";

    fn openssl_signature() -> Vec<u8> {
        let hex = "004ca0f7d5cc66412ac1ba843233c3c250edf59f1c5c684628f2a1254db25f27\
                   9e19d710fc50d44ad69db135ad09137b1e5191b5eafe4b7efe00c7bde9d02a07";
        (0..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap()).collect()
    }

    #[test]
    fn accepts_a_signature_made_by_openssl_with_the_release_key() {
        assert!(verify(SUMS, &openssl_signature()));
    }

    #[test]
    fn refuses_changed_data_bad_signatures_and_other_keys() {
        let sig = openssl_signature();
        assert!(!verify(b"abc124  Nectarlink-9.9.9-x64-setup.exe\n", &sig));
        let mut flipped = sig.clone();
        flipped[10] ^= 1;
        assert!(!verify(SUMS, &flipped));
        assert!(!verify(SUMS, &sig[..63]));
        assert!(!verify(SUMS, b""));
        let mut other = PUBLIC_KEY;
        other[0] ^= 0x55;
        assert!(!verify_with(&other, SUMS, &sig));
    }

    #[test]
    fn signature_sits_next_to_its_file() {
        assert_eq!(signature_name("SHA256SUMS.txt"), "SHA256SUMS.txt.sig");
    }
}
