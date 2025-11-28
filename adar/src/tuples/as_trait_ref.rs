use core::ops::{Deref, DerefMut};

pub trait AsTraitRef<T: ?Sized>: Sized {
    fn as_trait_ref(&self) -> &T;
}

pub trait AsTraitRefMut<T: ?Sized>: Sized {
    fn as_trait_mut(&mut self) -> &T;
}

macro_rules! impl_as_trait_ref {
    ($trait:path) => {
        impl<T> AsTraitRef<dyn $trait> for T
        where
            T: Sized + $trait + 'static,
        {
            fn as_trait_ref(&self) -> &(dyn $trait + 'static) {
                self
            }
        }
        impl<T> AsTraitRefMut<dyn $trait> for T
        where
            T: Sized + $trait + 'static,
        {
            fn as_trait_mut(&mut self) -> &(dyn $trait + 'static) {
                self
            }
        }
    };
}

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

impl<T, U> AsTraitRef<dyn Deref<Target = U>> for T
where
    T: Deref<Target = U> + 'static,
{
    fn as_trait_ref(&self) -> &(dyn Deref<Target = U> + 'static) {
        self
    }
}

impl<T, U> AsTraitRefMut<dyn Deref<Target = U>> for T
where
    T: Deref<Target = U> + 'static,
{
    fn as_trait_mut(&mut self) -> &(dyn Deref<Target = U> + 'static) {
        self
    }
}

impl<T, U> AsTraitRef<dyn DerefMut<Target = U>> for T
where
    T: DerefMut<Target = U> + 'static,
{
    fn as_trait_ref(&self) -> &(dyn DerefMut<Target = U> + 'static) {
        self
    }
}

impl<T, U> AsTraitRefMut<dyn DerefMut<Target = U>> for T
where
    T: DerefMut<Target = U> + 'static,
{
    fn as_trait_mut(&mut self) -> &(dyn DerefMut<Target = U> + 'static) {
        self
    }
}
