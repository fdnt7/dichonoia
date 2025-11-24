mod sealed {
    pub trait Sealed {}
}

pub trait Entity: sealed::Sealed {}

impl<T> sealed::Sealed for T where T: Entity {}

macro_rules! define_entities {
    ( $( $name:ident ),+ $(,)? ) => {
        $(
            #[derive(Debug, Clone, Copy)]
            pub struct $name;

            impl Entity for $name {}
        )+
    };
}

define_entities![Guild, User, Application];
