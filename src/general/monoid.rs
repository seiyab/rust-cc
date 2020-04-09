pub trait Monoid: SemiGroup {
    fn zero() -> Self;
}

pub trait SemiGroup {
    fn plus(&self, another: &Self) -> Self;
}

pub enum FreeMonoid<T: SemiGroup> {
    Zero,
    Some(T),
}

impl <T: SemiGroup + Clone> SemiGroup for FreeMonoid<T> {
    fn plus(&self, another: &Self) -> Self {
        match (self, another) {
            (Self::Zero, Self::Zero) =>  Self::Zero,
            (Self::Zero, Self::Some(v)) => Self::Some(v.clone()),
            (Self::Some(u), Self::Zero) => Self::Some(u.clone()),
            (Self::Some(u), Self::Some(v)) => Self::Some(u.plus(v)),
        }
    }
}

impl <T: SemiGroup + Clone> Monoid for FreeMonoid<T> {
    fn zero() -> Self {
        Self::Zero
    }
}

impl <T: SemiGroup> FreeMonoid<T> {
    pub fn get(&self) -> Option<&T> {
        match self {
            Self::Zero => None,
            Self::Some(x) => Some(x),
        }
    }
}