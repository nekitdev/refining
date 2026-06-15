//! Predicates based on emptiness.

use core::{ffi::CStr, fmt, marker::PhantomData};

use refining_core::{logical::Not, predicate::Predicate, types::StaticStr};

/// Represents types that have emptiness-checking capabilities.
pub trait HasEmpty {
    /// Checks whether the value is empty.
    fn empty(&self) -> bool;
}

/// The `empty value` literal.
pub const VALUE: StaticStr = "empty value";

/// The `empty` literal.
pub const EMPTY: StaticStr = "empty";

/// Checks whether the value is empty.
pub struct Empty {
    private: PhantomData<()>,
}

impl<T: HasEmpty + ?Sized> Predicate<T> for Empty {
    fn check(value: &T) -> bool {
        value.empty()
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(VALUE)
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(EMPTY)
    }
}

/// Checks whether the value is non-empty.
pub type NonEmpty = Not<Empty>;

// core

impl HasEmpty for str {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> HasEmpty for [T] {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T: HasEmpty + ?Sized> HasEmpty for &T {
    fn empty(&self) -> bool {
        (*self).empty()
    }
}

impl HasEmpty for CStr {
    fn empty(&self) -> bool {
        self.is_empty()
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

    use super::HasEmpty;

    // prelude

    impl<T: HasEmpty + ?Sized> HasEmpty for Box<T> {
        fn empty(&self) -> bool {
            self.as_ref().empty()
        }
    }

    impl HasEmpty for String {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    impl<T> HasEmpty for Vec<T> {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    // C strings

    impl HasEmpty for CString {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    // clone-on-write

    impl<T: ToOwned + HasEmpty + ?Sized> HasEmpty for Cow<'_, T> {
        fn empty(&self) -> bool {
            self.as_ref().empty()
        }
    }

    // reference-counted

    impl<T: HasEmpty + ?Sized> HasEmpty for Rc<T> {
        fn empty(&self) -> bool {
            self.as_ref().empty()
        }
    }

    impl<T: HasEmpty + ?Sized> HasEmpty for Arc<T> {
        fn empty(&self) -> bool {
            self.as_ref().empty()
        }
    }

    // collections

    impl<K, V> HasEmpty for BTreeMap<K, V> {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    impl<T> HasEmpty for BTreeSet<T> {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    impl<T> HasEmpty for BinaryHeap<T> {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    impl<T> HasEmpty for LinkedList<T> {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    impl<T> HasEmpty for VecDeque<T> {
        fn empty(&self) -> bool {
            self.is_empty()
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

    use super::HasEmpty;

    // collections

    impl<K, V, S> HasEmpty for HashMap<K, V, S> {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    impl<T, S> HasEmpty for HashSet<T, S> {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    // OS strings

    impl HasEmpty for OsStr {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    impl HasEmpty for OsString {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    // paths (via underlying strings)

    impl HasEmpty for Path {
        fn empty(&self) -> bool {
            self.as_os_str().empty()
        }
    }

    impl HasEmpty for PathBuf {
        fn empty(&self) -> bool {
            self.as_os_str().empty()
        }
    }
}
