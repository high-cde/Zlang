/// La firma e la persistenza su una chain esterna non sono implementate.
pub fn sign_transaction_zchain(_payload: &str) -> Result<(), &'static str> {
    Err("zchain esterna non implementata: nessuna firma o scrittura è stata eseguita")
}
