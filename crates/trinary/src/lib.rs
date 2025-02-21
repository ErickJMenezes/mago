use serde::Deserialize;
use serde::Serialize;
use strum::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord, Display)]
#[repr(i8)]
pub enum Trinary {
    True = 1,
    Maybe = 0,
    False = -1,
}

impl Trinary {
    #[inline(always)]
    pub fn is_true(self) -> bool {
        self == Trinary::True
    }

    #[inline(always)]
    pub fn maybe_true(self) -> bool {
        self == Trinary::True || self == Trinary::Maybe
    }

    #[inline(always)]
    pub fn is_false(self) -> bool {
        self == Trinary::False
    }

    #[inline(always)]
    pub fn maybe_false(self) -> bool {
        self == Trinary::False || self == Trinary::Maybe
    }

    #[inline(always)]
    pub fn is_maybe(self) -> bool {
        self == Trinary::Maybe
    }

    #[inline(always)]
    pub fn and(self, other: Trinary) -> Trinary {
        self & other
    }

    #[inline(always)]
    pub fn or(self, other: Trinary) -> Trinary {
        self | other
    }

    #[inline(always)]
    pub fn xor(self, other: Trinary) -> Trinary {
        self ^ other
    }

    #[inline(always)]
    pub fn negate(self) -> Trinary {
        !self
    }
}

impl From<bool> for Trinary {
    fn from(value: bool) -> Self {
        if value { Trinary::True } else { Trinary::False }
    }
}

impl From<Option<bool>> for Trinary {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Trinary::Maybe,
        }
    }
}

impl From<Option<Trinary>> for Trinary {
    fn from(value: Option<Trinary>) -> Self {
        value.unwrap_or(Trinary::Maybe)
    }
}

impl FromIterator<Trinary> for Trinary {
    fn from_iter<I: IntoIterator<Item = Trinary>>(iter: I) -> Self {
        let mut result = Trinary::True;
        for value in iter {
            result &= value;
        }

        result
    }
}

impl TryInto<bool> for Trinary {
    type Error = ();

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Trinary::True => Ok(true),
            Trinary::False => Ok(false),
            Trinary::Maybe => Err(()),
        }
    }
}

impl core::ops::Not for Trinary {
    type Output = Trinary;

    fn not(self) -> Self::Output {
        match self {
            Trinary::True => Trinary::False,
            Trinary::False => Trinary::True,
            Trinary::Maybe => Trinary::Maybe,
        }
    }
}

impl core::ops::BitAnd for Trinary {
    type Output = Trinary;

    fn bitand(self, other: Self) -> Self::Output {
        match (self, other) {
            (Trinary::True, Trinary::True) => Trinary::True,
            (Trinary::False, _) | (_, Trinary::False) => Trinary::False,
            _ => Trinary::Maybe,
        }
    }
}

impl core::ops::BitOr for Trinary {
    type Output = Trinary;

    fn bitor(self, other: Self) -> Self::Output {
        match (self, other) {
            (Trinary::False, Trinary::False) => Trinary::False,
            (Trinary::True, _) | (_, Trinary::True) => Trinary::True,
            _ => Trinary::Maybe,
        }
    }
}

impl core::ops::BitXor for Trinary {
    type Output = Trinary;

    fn bitxor(self, other: Self) -> Self::Output {
        match (self, other) {
            (Trinary::True, Trinary::True) => Trinary::False,
            (Trinary::False, Trinary::False) => Trinary::False,
            _ => Trinary::Maybe,
        }
    }
}

impl core::ops::BitAndAssign for Trinary {
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}

impl core::ops::BitOrAssign for Trinary {
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}

impl core::ops::BitXorAssign for Trinary {
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}

impl core::ops::BitAnd<Option<bool>> for Trinary {
    type Output = Trinary;

    fn bitand(self, other: Option<bool>) -> Self::Output {
        self & Trinary::from(other)
    }
}

impl core::ops::BitOr<Option<bool>> for Trinary {
    type Output = Trinary;

    fn bitor(self, other: Option<bool>) -> Self::Output {
        self | Trinary::from(other)
    }
}

impl core::ops::BitXor<Option<bool>> for Trinary {
    type Output = Trinary;

    fn bitxor(self, other: Option<bool>) -> Self::Output {
        self ^ Trinary::from(other)
    }
}

impl core::ops::BitAnd<bool> for Trinary {
    type Output = Trinary;

    fn bitand(self, other: bool) -> Self::Output {
        self & Trinary::from(other)
    }
}

impl core::ops::BitOr<bool> for Trinary {
    type Output = Trinary;

    fn bitor(self, other: bool) -> Self::Output {
        self | Trinary::from(other)
    }
}

impl core::ops::BitXor<bool> for Trinary {
    type Output = Trinary;

    fn bitxor(self, other: bool) -> Self::Output {
        self ^ Trinary::from(other)
    }
}

impl core::ops::BitAndAssign<Option<bool>> for Trinary {
    fn bitand_assign(&mut self, other: Option<bool>) {
        *self = *self & other;
    }
}

impl core::ops::BitOrAssign<Option<bool>> for Trinary {
    fn bitor_assign(&mut self, other: Option<bool>) {
        *self = *self | other;
    }
}

impl core::ops::BitAndAssign<bool> for Trinary {
    fn bitand_assign(&mut self, other: bool) {
        *self = *self & other;
    }
}

impl core::ops::BitOrAssign<bool> for Trinary {
    fn bitor_assign(&mut self, other: bool) {
        *self = *self | other;
    }
}
