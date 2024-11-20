pub trait RetainMarked<R> {
    fn retain_marked<I>(&mut self, it: I)
    where
        I: Iterator<Item = R>;
}
