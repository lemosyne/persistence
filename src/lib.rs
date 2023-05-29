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

    /// Associated error type.
    type Error: Debug;

    /// The produced `Io` type.
    type Io<'a>: Read + Write + Seek
    where
        Self: 'a;

    /// Creates a new object.
    fn create(&mut self, objid: &Self::Id) -> Result<(), Self::Error>;

    /// Destroys an object.
    fn destroy(&mut self, objid: &Self::Id) -> Result<(), Self::Error>;

    /// Returns an `Io` handle to read object with.
    fn read_handle(&mut self, objid: &Self::Id) -> Result<Self::Io<'_>, Self::Error>;

    /// Returns an `Io` handle to write an object with.
    fn write_handle(&mut self, objid: &Self::Id) -> Result<Self::Io<'_>, Self::Error>;

    /// Returns an `Io` handle to read and write an object with.
    fn rw_handle(&mut self, objid: &Self::Id) -> Result<Self::Io<'_>, Self::Error>;

    /// Returns the size, in bytes, of an object.
    fn size(&mut self, objid: &Self::Id) -> Result<u64, Self::Error>;

    /// Shortens an object.
    fn truncate(&mut self, objid: &Self::Id, size: u64) -> Result<(), Self::Error>;
}

#[cfg(feature = "std")]
pub mod standard {
    use crate::PersistentStorage;
    use embedded_io::adapters::FromStd;
    use path_macro::path;
    use serde::{Deserialize, Serialize};
    use std::{
        fs::{self, File},
        io,
        path::{Path, PathBuf},
    };

    #[derive(Serialize, Deserialize)]
    pub struct StdObjectStore {
        root: PathBuf,
    }

    impl StdObjectStore {
        pub fn new<P: AsRef<Path>>(root: P) -> io::Result<Self> {
            fs::create_dir_all(&root)?;
            Ok(Self {
                root: root.as_ref().into(),
            })
        }

        fn object_path(&self, objid: &u64) -> PathBuf {
            path![self.root / format!("{objid}")]
        }
    }

    impl PersistentStorage for StdObjectStore {
        type Id = u64;
        type Error = io::Error;
        type Io<'a> = FromStd<File>;

        fn create(&mut self, objid: &Self::Id) -> Result<(), Self::Error> {
            File::create(self.object_path(objid))?;
            Ok(())
        }

        fn destroy(&mut self, objid: &Self::Id) -> Result<(), Self::Error> {
            fs::remove_file(self.object_path(objid))
        }

        fn read_handle(&mut self, objid: &Self::Id) -> Result<Self::Io<'_>, Self::Error> {
            Ok(FromStd::new(
                File::options().read(true).open(self.object_path(objid))?,
            ))
        }

        fn write_handle(&mut self, objid: &Self::Id) -> Result<Self::Io<'_>, Self::Error> {
            Ok(FromStd::new(
                File::options().write(true).open(self.object_path(objid))?,
            ))
        }

        fn rw_handle(&mut self, objid: &Self::Id) -> Result<Self::Io<'_>, Self::Error> {
            Ok(FromStd::new(
                File::options()
                    .read(true)
                    .write(true)
                    .open(self.object_path(objid))?,
            ))
        }

        fn size(&mut self, objid: &Self::Id) -> Result<u64, Self::Error> {
            Ok(fs::metadata(self.object_path(objid))?.len())
        }

        fn truncate(&mut self, objid: &Self::Id, size: u64) -> Result<(), Self::Error> {
            File::options()
                .write(true)
                .open(self.object_path(objid))?
                .set_len(size)
        }
    }
}
