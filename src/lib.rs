pub struct Sender<T> {}

pub struct Receiver<T> {}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {}

