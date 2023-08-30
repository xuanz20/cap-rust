use core::cell::RefCell;

use intrusive_collections::UnsafeRef;

pub use self::{untyped::Untyepd, task::Task};

mod untyped;
mod task;
mod utils;

pub enum CapType {
    Untyped(Untyepd),
    Empty,
    Task(Task),
}

pub struct Capability {
    data: CapType,
    next: Option<CapPtr>,
    pre: Option<CapPtr>,
}

pub type CapPtr = UnsafeRef<RefCell<Capability>>;