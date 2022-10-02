// Reference: https://hoverbear.org/blog/rust-state-machine-pattern/

use std::marker::PhantomData;

#[derive(Clone, PartialEq, Eq)]
pub struct Zero;

#[derive(Clone, PartialEq, Eq)]
pub struct Written;

#[derive(Clone, PartialEq, Eq)]
pub struct Stale;

#[derive(Clone, PartialEq, Eq)]
pub struct ResourceStateMachine<State> {
    state: PhantomData<State>,
}

impl ResourceStateMachine<Zero> {
    fn new() -> Self {
        Self { state: PhantomData }
    }
}

impl From<ResourceStateMachine<Zero>> for ResourceStateMachine<Written> {
    fn from(_machine: ResourceStateMachine<Zero>) -> Self {
        Self { state: PhantomData }
    }
}

impl From<ResourceStateMachine<Zero>> for ResourceStateMachine<Stale> {
    fn from(_machine: ResourceStateMachine<Zero>) -> Self {
        Self { state: PhantomData }
    }
}

impl From<ResourceStateMachine<Written>> for ResourceStateMachine<Stale> {
    fn from(_machine: ResourceStateMachine<Written>) -> Self {
        Self { state: PhantomData }
    }
}

impl From<ResourceStateMachine<Stale>> for ResourceStateMachine<Written> {
    fn from(_machine: ResourceStateMachine<Stale>) -> Self {
        Self { state: PhantomData }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ResourceStateMachineWrapper {
    Zero(ResourceStateMachine<Zero>),
    Written(ResourceStateMachine<Written>),
    Stale(ResourceStateMachine<Stale>),
}

impl ResourceStateMachineWrapper {
    pub fn new() -> Self {
        Self::Zero(ResourceStateMachine::<Zero>::new())
    }

    pub fn write(mut self) -> Self {
        self = match self {
            Self::Zero(machine) => Self::Written(machine.into()),
            wrapper @ Self::Written(_) => wrapper,
            Self::Stale(machine) => Self::Written(machine.into()),
        };
        self
    }

    pub fn clear(mut self) -> Self {
        self = match self {
            Self::Zero(machine) => Self::Stale(machine.into()),
            Self::Written(machine) => Self::Stale(machine.into()),
            wrapper @ Self::Stale(_) => wrapper,
        };
        self
    }

    pub fn is_zero(&self) -> bool {
        matches!(self, Self::Zero(_))
    }

    pub fn is_written(&self) -> bool {
        matches!(self, Self::Written(_))
    }

    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Zero(_) | Self::Written(_))
    }
}

#[cfg(test)]
mod tests;
