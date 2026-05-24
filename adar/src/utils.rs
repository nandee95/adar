#[cfg(all(feature = "async-st", feature = "alloc"))]
pub trait MaybeSend {}

#[cfg(all(feature = "async-st", feature = "alloc"))]
impl<T> MaybeSend for T {}

#[cfg(all(not(feature = "async-st"), feature = "alloc"))]
pub trait MaybeSend: Send {}

#[cfg(all(not(feature = "async-st"), feature = "alloc"))]
impl<T: Send> MaybeSend for T {}

pub trait UnitType {
    fn unit() -> Self;
}
impl UnitType for () {
    fn unit() -> Self {}
}
