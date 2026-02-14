use getrandom::getrandom;

pub fn gen_nonce() -> Result<[u8; 32], getrandom::Error> {
    let mut rnd: [u8; 32] = [0u8; 32];
    getrandom(&mut rnd)?;
    Ok(rnd)
}
