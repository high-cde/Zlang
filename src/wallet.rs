use sha2::{Sha256, Digest};

pub struct ZBagWallet {
    pub address: String,
    pub balance: u64,
    pub nonce: u64,
}

impl ZBagWallet {
    pub fn new(address: &str, initial_balance: u64) -> Self {
        Self {
            address: address.to_string(),
            balance: initial_balance,
            nonce: 0,
        }
    }

    pub fn sign_transaction(&self, recipient: &str, amount: u64) -> String {
        let payload = format!("{}:{}:{}:{}", self.address, recipient, amount, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn generate_zbag_file(&self, recipient: &str, amount: u64) -> String {
        let sig = self.sign_transaction(recipient, amount);
        format!(
            "# === ZDOS NATIVE ZBAG WALlet TRANSACTION ===\n\
             sender = \"{}\"\n\
             recipient = \"{}\"\n\
             amount = {}\n\
             nonce = {}\n\
             signature = \"{}\"\n",
            self.address, recipient, amount, self.nonce, sig
        )
    }
}
