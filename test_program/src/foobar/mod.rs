// double export of a symbol occurs here

pub use funny_derive::Funny;

mod funny;
pub use self::funny::Funny;