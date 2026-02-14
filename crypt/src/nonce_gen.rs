use getrandom::getrandom;

pub fn gen_nonce() -> Result<[u8; 12], getrandom::Error> {
    let mut rnd: [u8; 12] = [0u8; 12];
    getrandom(&mut rnd)?;
    Ok(rnd)
}
