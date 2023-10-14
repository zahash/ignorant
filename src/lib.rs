pub trait Ignore {
    fn ignore(self);
}

impl<E> Ignore for Result<(), E> {
    fn ignore(self) {}
}
