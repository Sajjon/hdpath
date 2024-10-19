#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum KeySpace {
    Unsecurified { is_hardened: bool },
    Securified,
}
