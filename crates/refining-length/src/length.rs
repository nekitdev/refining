//! Traits for types that have length defined for their values.

use core::ffi::CStr;

/// Represents types that have length defined for their values.
pub trait HasLength {
    /// Returns the value length.
    fn length(&self) -> usize;
}

// core

impl HasLength for str {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> HasLength for [T] {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T: HasLength + ?Sized> HasLength for &T {
    fn length(&self) -> usize {
        (*self).length()
    }
}

impl HasLength for CStr {
    fn length(&self) -> usize {
        self.to_bytes().length()
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
mod std_or_alloc {
    cfg_select! {
        feature = "std" => {
            use std::{
                borrow::{Cow, ToOwned},
                collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque},
                ffi::CString,
                rc::Rc,
                sync::Arc,
            };
        }
        feature = "alloc" => {
            use alloc::{
                borrow::{Cow, ToOwned},
                boxed::Box,
                collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque},
                ffi::CString,
                rc::Rc,
                string::String,
                sync::Arc,
                vec::Vec,
            };
        }
    }

    use super::HasLength;

    // prelude

    impl<T: HasLength + ?Sized> HasLength for Box<T> {
        fn length(&self) -> usize {
            self.as_ref().length()
        }
    }

    impl HasLength for String {
        fn length(&self) -> usize {
            self.len()
        }
    }

    impl<T> HasLength for Vec<T> {
        fn length(&self) -> usize {
            self.len()
        }
    }

    // C strings

    impl HasLength for CString {
        fn length(&self) -> usize {
            self.to_bytes().length()
        }
    }

    // clone-on-write

    impl<T: ToOwned + HasLength + ?Sized> HasLength for Cow<'_, T> {
        fn length(&self) -> usize {
            self.as_ref().length()
        }
    }

    // reference-counted

    impl<T: HasLength + ?Sized> HasLength for Rc<T> {
        fn length(&self) -> usize {
            self.as_ref().length()
        }
    }

    impl<T: HasLength + ?Sized> HasLength for Arc<T> {
        fn length(&self) -> usize {
            self.as_ref().length()
        }
    }

    // collections

    impl<K, V> HasLength for BTreeMap<K, V> {
        fn length(&self) -> usize {
            self.len()
        }
    }

    impl<T> HasLength for BTreeSet<T> {
        fn length(&self) -> usize {
            self.len()
        }
    }

    impl<T> HasLength for BinaryHeap<T> {
        fn length(&self) -> usize {
            self.len()
        }
    }

    impl<T> HasLength for LinkedList<T> {
        fn length(&self) -> usize {
            self.len()
        }
    }

    impl<T> HasLength for VecDeque<T> {
        fn length(&self) -> usize {
            self.len()
        }
    }
}

#[cfg(feature = "std")]
mod std_only {
    use std::{
        collections::{HashMap, HashSet},
        ffi::{OsStr, OsString},
        path::{Path, PathBuf},
    };

    use super::HasLength;

    // collections

    impl<K, V, S> HasLength for HashMap<K, V, S> {
        fn length(&self) -> usize {
            self.len()
        }
    }

    impl<T, S> HasLength for HashSet<T, S> {
        fn length(&self) -> usize {
            self.len()
        }
    }

    // OS strings

    impl HasLength for OsStr {
        fn length(&self) -> usize {
            self.len()
        }
    }

    impl HasLength for OsString {
        fn length(&self) -> usize {
            self.len()
        }
    }

    // paths (via underlying strings)

    impl HasLength for Path {
        fn length(&self) -> usize {
            self.as_os_str().length()
        }
    }

    impl HasLength for PathBuf {
        fn length(&self) -> usize {
            self.as_os_str().length()
        }
    }
}
