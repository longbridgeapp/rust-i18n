use once_cell::sync::Lazy;
use siphasher::sip128::SipHasher13;

/// The prefix of auto-generated literal translation key
pub const TR_KEY_PREFIX: &str = "tr_";

// The hasher for generate the literal translation key
static TR_KEY_HASHER: Lazy<SipHasher13> = Lazy::new(SipHasher13::new);

pub trait TrKeyNumeric: std::fmt::Display {
    fn tr_key_numeric(&self) -> String {
        format!("{}N_{}", TR_KEY_PREFIX, self)
    }
}

/// A trait for generating translation key from a value.
pub trait TrKey {
    fn tr_key(&self) -> String;
}

macro_rules! impl_tr_key_for_numeric {
    ($typ:ty) => {
        impl TrKeyNumeric for $typ {}
        impl TrKey for $typ {
            #[inline]
            fn tr_key(&self) -> String {
                self.tr_key_numeric()
            }
        }
    };
}

macro_rules! impl_tr_key_for_signed_numeric {
    ($typ:ty) => {
        impl TrKeyNumeric for $typ {}
        impl TrKey for $typ {
            #[inline]
            fn tr_key(&self) -> String {
                (*self as u128).tr_key_numeric()
            }
        }
    };
}

impl_tr_key_for_numeric!(u8);
impl_tr_key_for_numeric!(u16);
impl_tr_key_for_numeric!(u32);
impl_tr_key_for_numeric!(u64);
impl_tr_key_for_numeric!(u128);
impl_tr_key_for_numeric!(usize);
impl_tr_key_for_signed_numeric!(i8);
impl_tr_key_for_signed_numeric!(i16);
impl_tr_key_for_signed_numeric!(i32);
impl_tr_key_for_signed_numeric!(i64);
impl_tr_key_for_signed_numeric!(i128);
impl_tr_key_for_signed_numeric!(isize);

impl TrKey for [u8] {
    #[inline]
    fn tr_key(&self) -> String {
        let hash = TR_KEY_HASHER.hash(self).as_u128();
        format!("{}{}", TR_KEY_PREFIX, base62::encode(hash))
    }
}

impl TrKey for str {
    #[inline]
    fn tr_key(&self) -> String {
        self.as_bytes().tr_key()
    }
}

impl TrKey for &str {
    #[inline]
    fn tr_key(&self) -> String {
        self.as_bytes().tr_key()
    }
}

impl TrKey for String {
    #[inline]
    fn tr_key(&self) -> String {
        self.as_bytes().tr_key()
    }
}

impl TrKey for &String {
    #[inline]
    fn tr_key(&self) -> String {
        self.as_bytes().tr_key()
    }
}

impl<'a> TrKey for std::borrow::Cow<'a, str> {
    #[inline]
    fn tr_key(&self) -> String {
        self.as_bytes().tr_key()
    }
}

impl<'a> TrKey for &std::borrow::Cow<'a, str> {
    #[inline]
    fn tr_key(&self) -> String {
        self.as_bytes().tr_key()
    }
}
