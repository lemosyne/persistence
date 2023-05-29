use core::fmt::Debug;
use embedded_io::blocking::{Read, Seek, Write};

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
    /// Flags for creation.
    type Flags;
    /// Contains object information.
    type Info;
    /// Associated error type.
    type Error: Debug;
    /// The produced `Io` type.
    type Io<'a>: Read + Write + Seek
    where
        Self: 'a;

    /// Creates a new object.
    fn create(&mut self, objid: &Self::Id, flags: Self::Flags) -> Result<(), Self::Error>;

    /// Destroys an object.
    fn destroy(&mut self, objid: &Self::Id) -> Result<(), Self::Error>;

    /// Gets information about an object.
    fn info(&mut self, objid: &Self::Id) -> Result<Self::Info, Self::Error>;

    /// Returns an `Io` handle to read object with.
    fn read_handle(&mut self, objid: &Self::Id) -> Result<Self::Io<'_>, Self::Error>;

    /// Returns an `Io` handle to write an object with.
    fn write_handle(&mut self, objid: &Self::Id) -> Result<Self::Io<'_>, Self::Error>;

    /// Returns an `Io` handle to read and write an object with.
    fn rw_handle(&mut self, objid: &Self::Id) -> Result<Self::Io<'_>, Self::Error>;

    /// Shortens an object.
    fn truncate(&mut self, objid: &Self::Id, size: u64) -> Result<(), Self::Error>;
}
