use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use crate::{TBCSDeserialize, TBCSSerialize};

// ---------------------------------------------------------------------------
// Primitives
// ---------------------------------------------------------------------------

macro_rules! impl_bcs_primitive {
    ($($t:ty),* $(,)?) => {
        $(
            impl TBCSSerialize for $t {}
            impl<'de> TBCSDeserialize<'de> for $t {}
        )*
    };
}

impl_bcs_primitive!(
    (),
    bool,
    char,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64,
    String,
);

impl TBCSSerialize for str {}

impl TBCSSerialize for Path {}
impl TBCSSerialize for PathBuf {}
impl TBCSDeserialize<'_> for PathBuf {}

// ---------------------------------------------------------------------------
// References
// ---------------------------------------------------------------------------

impl<T: ?Sized + TBCSSerialize> TBCSSerialize for &T {}

// ---------------------------------------------------------------------------
// Standard wrappers / collections
// ---------------------------------------------------------------------------

impl<T: TBCSSerialize> TBCSSerialize for Option<T> {}
impl<'de, T: TBCSDeserialize<'de>> TBCSDeserialize<'de> for Option<T> {}

impl<T: TBCSSerialize> TBCSSerialize for Vec<T> {}
impl<'de, T: TBCSDeserialize<'de>> TBCSDeserialize<'de> for Vec<T> {}

impl<T: TBCSSerialize> TBCSSerialize for Box<T> {}
impl<'de, T: TBCSDeserialize<'de>> TBCSDeserialize<'de> for Box<T> {}

impl<T: TBCSSerialize> TBCSSerialize for [T] {}

// serde (this workspace) implements Serialize/Deserialize for arrays up to size 32,
// not via const generics.
macro_rules! impl_bcs_array {
    ($($n:expr),* $(,)?) => {
        $(
            impl<T: TBCSSerialize> TBCSSerialize for [T; $n] {}
            impl<'de, T: TBCSDeserialize<'de>> TBCSDeserialize<'de> for [T; $n] {}
        )*
    };
}

impl_bcs_array!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32
);

impl<T> TBCSSerialize for Cow<'_, T> where T: ?Sized + ToOwned + TBCSSerialize {}
impl<'de, T> TBCSDeserialize<'de> for Cow<'_, T>
where
    T: ?Sized + ToOwned,
    T::Owned: TBCSDeserialize<'de>,
{
}

impl<K: TBCSSerialize, V: TBCSSerialize> TBCSSerialize for BTreeMap<K, V> {}
impl<'de, K: TBCSDeserialize<'de> + Ord, V: TBCSDeserialize<'de>> TBCSDeserialize<'de>
    for BTreeMap<K, V>
{
}

impl<T: TBCSSerialize> TBCSSerialize for BTreeSet<T> {}
impl<'de, T: TBCSDeserialize<'de> + Ord> TBCSDeserialize<'de> for BTreeSet<T> {}

// HashMap is intentionally unsupported: BCS requires canonical key ordering, and
// HashMap iteration order is nondeterministic. Use BTreeMap for map payloads.

// ---------------------------------------------------------------------------
// Tuples (arity 1..=12)
// ---------------------------------------------------------------------------

macro_rules! impl_bcs_tuple {
    ($($name:ident),+) => {
        impl<$($name: TBCSSerialize),+> TBCSSerialize for ($($name,)+) {}
        impl<'de, $($name: TBCSDeserialize<'de>),+> TBCSDeserialize<'de> for ($($name,)+) {}
    };
}

impl_bcs_tuple!(A);
impl_bcs_tuple!(A, B);
impl_bcs_tuple!(A, B, C);
impl_bcs_tuple!(A, B, C, D);
impl_bcs_tuple!(A, B, C, D, E);
impl_bcs_tuple!(A, B, C, D, E, F);
impl_bcs_tuple!(A, B, C, D, E, F, G);
impl_bcs_tuple!(A, B, C, D, E, F, G, H);
impl_bcs_tuple!(A, B, C, D, E, F, G, H, I);
impl_bcs_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_bcs_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_bcs_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

#[cfg(feature = "decimal")]
impl crate::TBCSSerialize for rust_decimal::Decimal {}
#[cfg(feature = "decimal")]
impl<'de> crate::TBCSDeserialize<'de> for rust_decimal::Decimal {}

#[cfg(feature = "bigdecimal")]
impl crate::TBCSSerialize for bigdecimal::BigDecimal {}
#[cfg(feature = "bigdecimal")]
impl<'de> crate::TBCSDeserialize<'de> for bigdecimal::BigDecimal {}
