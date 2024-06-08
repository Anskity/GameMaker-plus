pub trait Boxable {
    fn to_box(&self) -> Box<Self>
    where
        Self: Sized,
        Self: Clone,
    {
        Box::new(self.to_owned())
    }
}
