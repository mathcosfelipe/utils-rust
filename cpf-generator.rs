use rand::Rng;

pub fn generate() -> String{
    let mut rng = rand::thread_rng();
    const CHARSET: &[u8] = b"0123456789";

    let mut vec: Vec<char> = (0..9)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    vec.push(CHARSET[validate_first_digit(&vec)] as char);
    vec.push(CHARSET[validate_second_digit(&vec)] as char);

    return vec.into_iter().collect();
}
