use derive_more::{Add, AddAssign, Div, DivAssign, From, Into, Mul, MulAssign, Sub, SubAssign};

macro_rules! float_impl {
    ($name:ident, $typ:ty) => {
        #[derive(
            Clone, Copy, Add, AddAssign, Div, DivAssign, From, Into, Mul, MulAssign, Sub, SubAssign,
        )]
        pub struct $name($typ);

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.total_cmp(&other.0)
            }
        }

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0.total_cmp(&other.0).is_eq()
            }
        }

        impl Eq for $name {}

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.to_bits().hash(state);
            }
        }
    };
}

//float_impl!(TotalCmpF16, f16);
float_impl!(TotalCmpF32, f32);
float_impl!(TotalCmpF64, f64);
//float_impl!(TotalCmpF128, f128);
