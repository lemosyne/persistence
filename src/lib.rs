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

    /// Associated error type.
    type Error: Debug;

    /// The produced `Io` type.
    type Io<'a>: Io
    where
        Self: 'a;

    /// Produces a new `Io` that is backed by an arbitrary number of bytes.
    fn open<'a>(
        &'a mut self,
        id: Self::Id,
        access: StorageAccess,
    ) -> Result<Self::Io<'_>, Self::Error>;
}

/// An extremely basic hint for how storage will be accessed.
pub enum StorageAccess {
    Read,
    Write,
}

#[cfg(feature = "std")]
pub mod standard {
    use crate::{PersistentStorage, StorageAccess};
    use embedded_io::adapters::FromStd;
    use path_macro::path;
    use std::{
        fs::{self, File},
        io,
        path::{Path, PathBuf},
    };

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

        fn object_path(&self, objid: u64) -> PathBuf {
            path![self.root / format!("{objid}")]
        }
    }

    impl PersistentStorage for StdObjectStore {
        type Id = u64;
        type Io<'a> = FromStd<File>;
        type Error = io::Error;

        fn open<'a>(
            &'a mut self,
            objid: Self::Id,
            access: StorageAccess,
        ) -> Result<Self::Io<'_>, Self::Error> {
            Ok(FromStd::new(match access {
                StorageAccess::Read => File::options().read(true).open(self.object_path(objid)),
                StorageAccess::Write => File::options()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(self.object_path(objid)),
            }?))
        }
    }
}
