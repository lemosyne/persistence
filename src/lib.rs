use core::fmt::Debug;
use embedded_io::{
    blocking::{Read, Write},
    Io,
};

/// A trait for persisting and loading objects using `Io`s.
pub trait Persist<Io>: Sized
where
    Io: Read + Write,
{
    /// Associated error type.
    type Error: Debug;

    /// Persists `self` to `sink`.
    fn persist(&mut self, sink: Io) -> Result<(), Self::Error>;

    /// Loads `Self` from `source`.
    fn load(source: Io) -> Result<Self, Self::Error>;
}

/// Types that can be used to generate handles to write data to persistent storage.
pub trait PersistentStorage {
    /// The identifier for the target of an `Io`.
    type Id;

    /// Flag supplied on `open()`.
    type Flag;

    /// Associated error type.
    type Error: Debug;

    /// The produced `Io` type.
    type Io<'a>: Io
    where
        Self: 'a;

    /// Produces a new `Io` that is backed by an arbitrary number of bytes.
    fn open<'a>(&'a mut self, id: Self::Id, flag: Self::Flag) -> Result<Self::Io<'_>, Self::Error>;
}
