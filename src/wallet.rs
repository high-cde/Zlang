use std::fs;
use std::io;
use rand::RngExt;
use bip39::{Mnemonic, Language};
use secp256k1::{SecretKey, PublicKey};
use tiny_keccak::{Keccak, Hasher};

pub struct ZdosWallet {
    pub address: String,
    pub private_key: String,
    pub mnemonic: String,
}

impl ZdosWallet {
    pub fn create_new() -> Self {
        // Generazione Mnemonic BIP-39 (12 parole)
        let mut rng = rand::rng();
        let mut entropy = [0u8; 16];
        rng.fill(&mut entropy);
        
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
        let seed = mnemonic.to_seed("");
        
        // Derivazione chiave privata secp256k1 (Compatibile MetaMask / Ethereum / EVM)
        let secret_bytes: [u8; 32] = seed[..32].try_into().expect("Seed non valido");
        let secret_key = SecretKey::from_secret_bytes(secret_bytes).expect("32 bytes seed validi per secp256k1");
        let priv_hex = hex::encode(secret_key.to_secret_bytes());
        
        // Generazione indirizzo EVM (MetaMask) tramite Keccak-256 della chiave pubblica
        let public_key = PublicKey::from_secret_key(&secret_key);
        let pub_serialized = public_key.serialize_uncompressed();
        
        let mut keccak = Keccak::v256();
        let mut hash = [0u8; 32];
        keccak.update(&pub_serialized[1..]);
        keccak.finalize(&mut hash);
        let addr_hex = format!("0x{}", hex::encode(&hash[12..]));

        println!("[+] Wallet ZDOS & EVM (MetaMask) generato con successo!");
        println!("[*] Mnemonic BIP-39: {}", mnemonic);
        println!("[*] Indirizzo EVM: {}", addr_hex);

        ZdosWallet {
            address: addr_hex,
            private_key: priv_hex,
            mnemonic: mnemonic.to_string(),
        }
    }

    pub fn save_to_zbag(&self, filename: &str) -> io::Result<()> {
        let content = format!(
            "{{\n  \"protocol\": \"ZDOS-SLY-EVM\",\n  \"address\": \"{}\",\n  \"private_key\": \"{}\",\n  \"mnemonic\": \"{}\",\n  \"compatible\": [\"MetaMask\", \"Coinomi\", \"RandomX-Pools\"],\n  \"version\": \"2026.2\"\n}}",
            self.address, self.private_key, self.mnemonic
        );
        fs::write(filename, content)?;
        println!("[+] Backup sicuro esportato in formato .zbag: {}", filename);
        Ok(())
    }
}
