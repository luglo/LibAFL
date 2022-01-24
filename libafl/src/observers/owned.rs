//! A dynamic collection of owned observers

use core::{any::Any, fmt::Debug};

use crate::{
    bolts::{
        anymap::{pack_type_id, AsAny},
        tuples::MatchName,
    },
    observers::{Observer, ObserversTuple},
    Error,
};

////////// Warning, unsafe as hell, this bypass the standard library ///////////

extern "rust-intrinsic" {
    fn type_id<T: ?Sized>() -> u64;
}

unsafe fn downcast_ref_unsafe<T>(any: &dyn Any) -> &T {
    &*(any as *const dyn Any as *const T)
}

unsafe fn downcast_mut_unsafe<T>(any: &mut dyn Any) -> &mut T {
    &mut *(any as *mut dyn Any as *mut T)
}

////////////////////////////////////////////////////////////////////////////////

/// Combine `Observer` and `AsAny`
pub trait AnyObserver<I: 'static + Debug, S: 'static + Debug>: Observer<I, S> + AsAny {}

crate::create_anymap_for_trait!(
    observers_anymap,
    super,
    AnyObserver<I: 'static + Debug, S: 'static + Debug>,
    derive(Debug)
);
pub use observers_anymap::{AnyMap as ObserversAnyMap, NamedAnyMap as NamedObserversAnyMap};

/// An owned list of `Observer` trait objects
#[derive(Debug, Default)]
pub struct ObserversOwnedMap<I: 'static + Debug, S: 'static + Debug> {
    /// The named trait objects map
    pub map: NamedObserversAnyMap<I, S>,
}

impl<I: 'static + Debug, S: 'static + Debug> ObserversTuple<I, S> for ObserversOwnedMap<I, S> {
    fn pre_exec_all(&mut self, state: &mut S, input: &I) -> Result<(), Error> {
        self.map
            .for_each_mut(&mut |_, ob| ob.pre_exec(state, input))
    }

    fn post_exec_all(&mut self, state: &mut S, input: &I) -> Result<(), Error> {
        self.map
            .for_each_mut(&mut |_, ob| ob.post_exec(state, input))
    }
}

impl<I: 'static + Debug, S: 'static + Debug> MatchName for ObserversOwnedMap<I, S> {
    fn match_name<T>(&self, name: &str) -> Option<&T> {
        unsafe {
            let t = pack_type_id(type_id::<T>());
            self.map
                .by_typeid(name, &t)
                .map(|x| downcast_ref_unsafe(x.as_any()))
        }
    }
    fn match_name_mut<T>(&mut self, name: &str) -> Option<&mut T> {
        unsafe {
            let t = pack_type_id(type_id::<T>());
            self.map
                .by_typeid_mut(name, &t)
                .map(|x| downcast_mut_unsafe(x.as_any_mut()))
        }
    }
}
