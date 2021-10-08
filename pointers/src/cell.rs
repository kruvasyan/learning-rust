use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}


// provided by UnsafeCell
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell { value: UnsafeCell::new(value) }
    }

    pub fn set(&self, value: T) {
        let pointer = self.value.get();
        // SAFETY: no one else concurrently mutating self.value because !Sync.
        // No need for invalidating any reference, because never give out.
        unsafe { *pointer = value }
    }

    pub fn get(&self) -> T where T: Copy {
        // SAFETY: no-one else is modifying this value, since only this thread
        // can mutate because !Sync, and it is executing this function instead.
        unsafe { *self.value.get() }
    }
}
