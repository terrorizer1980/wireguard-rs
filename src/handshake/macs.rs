use std::time::Instant;

use blake2::Blake2s;
use subtle::ConstantTimeEq;
use x25519_dalek::PublicKey;

use super::messages::{CookieReply, MacsFooter};

const LABEL_MAC1: &[u8] = b"mac1----";
const LABEL_COOKIE: &[u8] = b"cookie--";

const SIZE_COOKIE: usize = 16;
const SIZE_MAC: usize = 16; // blake2s-mac128

macro_rules! HASH {
    ( $($input:expr),* ) => {{
        use blake2::Digest;
        let mut hsh = Blake2s::new();
        $(
            hsh.input($input);
        )*
        hsh.result()
    }};
}

macro_rules! MAC {
    ( $key:expr, $($input:expr),* ) => {{
        use blake2::VarBlake2s;
        use digest::Input;
        use digest::VariableOutput;
        let mut tag = [0u8; SIZE_MAC];
        let mut mac = VarBlake2s::new_keyed($key, SIZE_MAC);
        $(
            mac.input($input);
        )*
        mac.variable_result(|buf| tag.copy_from_slice(buf));
        tag
    }};
}

pub struct Generator {
    mac1_key: [u8; 32],
    cookie_value: [u8; 16],
    cookie_birth: Option<Instant>, // when was the cookie set?
}

impl Generator {
    pub fn new(pk: PublicKey) -> Generator {
        Generator {
            mac1_key: HASH!(LABEL_MAC1, pk.as_bytes()).into(),
            cookie_value: [0u8; SIZE_COOKIE],
            cookie_birth: None,
        }
    }

    fn mac1(&self, msg: &[u8]) -> [u8; SIZE_MAC] {
        MAC!(&self.mac1_key, msg)
    }

    fn mac2(&self, msg: &[u8]) -> [u8; SIZE_MAC] {
        MAC!(&self.cookie_value, msg)
    }

    pub fn set_cookie(&mut self, cookie: &[u8; SIZE_COOKIE]) {
        self.cookie_birth = Some(Instant::now());
        self.cookie_value = *cookie;
    }

    pub fn generate(&self, inner: &[u8], macs : &mut MacsFooter) {

    }
}

pub struct Validator {}

impl Validator {}