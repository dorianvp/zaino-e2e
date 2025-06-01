use testcontainers::{ContainerAsync, GenericImage};

pub type DynContainer<'a> = ContainerAsync<GenericImage>;
