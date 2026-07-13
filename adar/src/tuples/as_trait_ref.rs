pub trait AsTraitRef<T: ?Sized> {
    fn as_trait_ref(value: &T) -> &Self;
}
pub trait AsTraitMut<T: ?Sized> {
    fn as_trait_mut(value: &mut T) -> &mut Self;
}

macro_rules! impl_as_trait_ref_impl {
    ($trait:path, $($generics:tt)*) => {
        impl<T, $($generics)*> AsTraitRef<T> for dyn $trait
        where
            T: $trait + 'static,
        {
            fn as_trait_ref(value: &T) -> &Self {
                value
            }
        }
        impl<T, $($generics)*> AsTraitMut<T> for dyn $trait
        where
            T: $trait + 'static,
        {
            fn as_trait_mut(value: &mut T) -> &mut Self {
                value
            }
        }
    };
}

macro_rules! impl_as_trait_ref {
    ($trait:path) => {
        impl_as_trait_ref_impl!($trait,);
    };
    ($trait:path, $($generics:tt)+) => {
        impl_as_trait_ref_impl!($trait, $($generics)+);
    };
}

impl_as_trait_ref!(Fn());
impl_as_trait_ref!(core::any::Any);
impl_as_trait_ref!(core::fmt::Debug);
impl_as_trait_ref!(core::fmt::Display);
impl_as_trait_ref!(core::error::Error);
impl_as_trait_ref!(core::fmt::Binary);
impl_as_trait_ref!(core::fmt::Octal);
impl_as_trait_ref!(core::fmt::LowerHex);
impl_as_trait_ref!(core::fmt::UpperHex);
impl_as_trait_ref!(core::fmt::Pointer);
impl_as_trait_ref!(core::fmt::LowerExp);
impl_as_trait_ref!(core::fmt::UpperExp);
impl_as_trait_ref!(core::convert::AsRef<[T]>);
impl_as_trait_ref!(core::convert::AsMut<[T]>);
impl_as_trait_ref!(core::borrow::Borrow<[T]>);
impl_as_trait_ref!(core::borrow::BorrowMut<[T]>);
#[cfg(feature = "std")]
impl_as_trait_ref!(std::io::Read);
#[cfg(feature = "std")]
impl_as_trait_ref!(std::io::Write);
#[cfg(feature = "std")]
impl_as_trait_ref!(std::io::BufRead);
#[cfg(feature = "std")]
impl_as_trait_ref!(std::io::Seek);
#[cfg(feature = "std")]
impl_as_trait_ref!(std::string::ToString);
impl_as_trait_ref!(core::future::Future<Output = OUTPUT>, OUTPUT);
impl_as_trait_ref!(core::ops::Deref<Target = TARGET>, TARGET);
impl_as_trait_ref!(core::ops::DerefMut<Target = TARGET>, TARGET);
