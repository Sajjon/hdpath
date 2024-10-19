use std::borrow::Cow;

use crate::prelude::*;

use bip39::Mnemonic;
use crypto::{
    keys::slip10::{self as IotaSlip10, Hardened as IotaSlip10PathComponent, Slip10},
    signatures::ed25519 as IotaSlip10Ed25519,
};
use zeroize::Zeroizing;

pub struct BIP39Seed([u8; 64]);

pub struct Ed25519PublicKey(IotaSlip10Ed25519::PublicKey);
impl Ed25519PublicKey {
    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }
    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }
}
pub struct Ed25519PrivateKey(IotaSlip10Ed25519::SecretKey);
impl Ed25519PrivateKey {
    pub fn public_key(&self) -> Ed25519PublicKey {
        Ed25519PublicKey(self.0.public_key())
    }
    pub fn to_bytes(&self) -> Zeroizing<[u8; 32]> {
        self.0.to_bytes()
    }
    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }
}

impl HDPath {
    fn hardened_chain(&self) -> Vec<IotaSlip10PathComponent> {
        self.components()
            .iter()
            .map(|c| c.into_global_key_space())
            .map(|v| IotaSlip10PathComponent::try_from(v).expect("Should work"))
            .collect::<Vec<IotaSlip10PathComponent>>()
    }
}

impl BIP39Seed {
    fn derive_slip10_private_key<K, I>(&self, chain: I) -> IotaSlip10::Slip10<K>
    where
        K: IotaSlip10::IsSecretKey + IotaSlip10::WithSegment<<I as Iterator>::Item>,
        I: Iterator,
        <I as Iterator>::Item: IotaSlip10::Segment,
    {
        let iota_seed = IotaSlip10::Seed::from_bytes(&self.0);
        iota_seed.derive(chain)
    }

    fn derive_ed25519_private_key(
        &self,
        path: &HDPath,
    ) -> Slip10<crypto::signatures::ed25519::SecretKey> {
        self.derive_slip10_private_key::<IotaSlip10Ed25519::SecretKey, _>(
            path.hardened_chain().into_iter(),
        )
    }

    pub fn derive_private_key(&self, hd_path: impl Into<HDPath>) -> Ed25519PrivateKey {
        let key = self.derive_ed25519_private_key(&hd_path.into());
        let inner = key.secret_key();
        Ed25519PrivateKey(inner)
    }
}

pub trait ToSeed {
    fn to_bip39_seed<'a, P: Into<Cow<'a, str>>>(&self, passphrase: P) -> BIP39Seed;
}
impl ToSeed for Mnemonic {
    fn to_bip39_seed<'a, P: Into<Cow<'a, str>>>(&self, passphrase: P) -> BIP39Seed {
        BIP39Seed(self.to_seed(passphrase))
    }
}

pub struct FactorSourceIDFromHash;
impl FactorSourceIDFromHash {
    pub fn from_mnemonic_with_passphrase(
        mnemonic: impl AsRef<str>,
        passphrase: impl AsRef<str>,
    ) -> String {
        let mnemonic = Mnemonic::from_str(mnemonic.as_ref()).unwrap();
        let seed = mnemonic.to_bip39_seed(passphrase.as_ref());
        let private_key = seed.derive_private_key(CAP26GetIDPath);
        let public_key_bytes = private_key.public_key().to_bytes();
        let hash = blake2b_256_hash(public_key_bytes);
        hex::encode(hash)
    }
}

#[cfg(test)]
mod tests {
    use bip39::Mnemonic;

    use super::*;

    fn test_with_passphrase<P>(
        mnemonic: impl AsRef<str>,
        passphrase: impl AsRef<str>,
        path: impl AsRef<str>,
        assert: impl Fn(&Ed25519PrivateKey, &Ed25519PublicKey),
    ) where
        P: FromStr + Into<HDPath>,

        P::Err: std::fmt::Debug,
    {
        let mnemonic = Mnemonic::from_str(mnemonic.as_ref()).unwrap();
        let seed = mnemonic.to_bip39_seed(passphrase.as_ref());
        let path = P::from_str(path.as_ref()).unwrap();
        let path: HDPath = path.into();
        let private_key = seed.derive_private_key(path);
        assert(&private_key, &private_key.public_key());
    }

    fn test<P>(
        mnemonic: impl AsRef<str>,
        path: impl AsRef<str>,
        assert: impl Fn(&Ed25519PrivateKey, &Ed25519PublicKey),
    ) where
        P: FromStr + Into<HDPath>,
        P::Err: std::fmt::Debug,
    {
        test_with_passphrase::<P>(mnemonic, "", path, assert);
    }

    fn test_assert_key_hexes<P>(
        mnemonic: impl AsRef<str>,
        path: impl AsRef<str>,
        private_key_hex: impl AsRef<str>,
        public_key_hex: impl AsRef<str>,
    ) where
        P: FromStr + Into<HDPath>,
        P::Err: std::fmt::Debug,
    {
        test::<P>(mnemonic, path, |private_key, public_key| {
            assert_eq!(private_key.to_hex(), private_key_hex.as_ref());
            assert_eq!(public_key.to_hex(), public_key_hex.as_ref());
        });
    }

    #[test]
    fn derivation_kisharnet_account() {
        test_assert_key_hexes::<CAP26AccountPath>(
            "equip will roof matter pink blind book anxiety banner elbow sun young",
            "m/44H/1022H/12H/525H/1460H/0H",
            "13e971fb16cb2c816d6b9f12176e9b8ab9af1831d006114d344d119ab2715506",
            "451152a1cef7be603205086d4ebac0a0b78fda2ff4684b9dea5ca9ef003d4e7d",
        );
    }

    #[test]
    fn derivation_mainnet_account() {
        test_assert_key_hexes::<CAP26AccountPath>(
            "device phone sign source sample device sample device sample device sample device sample device sample device sample device phone sign source sample device swim", 
            "m/44H/1022H/1H/525H/1460H/0H", 
                "88ec4649da764965f862510dbe53d551a3fc2da49e1ef1f383d9d17006773bee",
                "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36",
        );
    }

    #[test]
    fn derivation_get_id() {
        let mnemonic_sample_device = "device phone sign source sample device sample device sample device sample device sample device sample device sample device phone sign source sample device swim";
        let id_from_hash =
            FactorSourceIDFromHash::from_mnemonic_with_passphrase(mnemonic_sample_device, "");
        let expected = "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a";
        assert_eq!(expected, id_from_hash)
    }
}
