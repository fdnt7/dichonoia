mod sealed {
    pub trait Sealed {}
}

pub trait Entity: sealed::Sealed {}

impl<T> sealed::Sealed for T where T: Entity {}

#[derive(Debug, Clone, Copy)]
pub struct Guild;
impl Entity for Guild {}

#[derive(Debug, Clone, Copy)]
pub struct User;
impl Entity for User {}
