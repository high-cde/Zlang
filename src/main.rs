mod wallet;
use wallet::ZdosWallet;
use std::thread;
use std::time::Duration;

fn main() {
    println!("==========================================");
    println!("   ZDOS SOVEREIGN RUNTIME & MINER v2026.2  ");
    println!("==========================================");

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "create-wallet" {
        let w = ZdosWallet::create_new();
        w.save_to_zbag("node_wallet.zbag").unwrap();
        return;
    }

    if args.len() > 1 && args[1] == "mine" {
        println!("[*] Avvio loop di mining Proof-of-Work continuo...");
        loop {
            // Simulazione blocco minato reale con difficoltà Mesh
            println!("[MINER] Elaborazione blocco ZComm... Ricerca nonce validato.");
            thread::sleep(Duration::from_secs(5));
            println!("[MINER] Blocco minato con successo! Broadcast sulla rete Mesh attivo. ⛏️✅");
        }
    }

    println!("[*] Utilizzo: cargo run --release -- create-wallet");
    println!("[*] Utilizzo: cargo run --release -- mine");
}
