use crate::cell::Cell;
use std::marker::PhantomData;
use std::ptr::NonNull;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });
        let raw = Box::into_raw(inner);
        // SAFETY: Box does not give us a null pointer.
        Rc {
            inner: unsafe { NonNull::new_unchecked(raw) } ,
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box is only deallocated when the last Rc goes away.
        // We have an Rc, therefore the Box has not need deallocated, so deref is fine.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        // SAFETY: self.inner is a Box is only deallocated when the last Rc goes away.
        // We have an Rc, therefore the Box has not need deallocated, so close is fine.
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc { inner: self.inner, _marker: PhantomData }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        // SAFETY: self.inner is a Box is only deallocated when the last Rc goes away.
        // So here we check refcount for correct deallocating.
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            // SAFETY: we are the _only_ Rc left, and we being dropped.
            // Therefore, after us, there will be no Rcs, and no references to T.
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
            inner.refcount.set(0);
        } else {
            // there are other Rcs, so don't drop the Box!
            inner.refcount.set(c - 1);
        }
    }
}
