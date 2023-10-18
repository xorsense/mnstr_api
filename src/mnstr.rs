use nanoid::nanoid;

pub struct Mnstr {
    shortcode: String,
    image: Vec<u8>,
}

impl Mnstr {
    fn new() -> Mnstr {
        let shortcode = nanoid!(6);
        Mnstr{shortcode: shortcode, image: vec!()}
    }

    fn from(bytes: &[u8]) -> Mnstr {
        let shortcode = nanoid!(6);
        Mnstr{shortcode: shortcode, image: bytes.to_vec()}
    }
}