use core::cell::{Cell, Ref, RefCell};

use alloc::boxed::{Box, ThinBox};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::std::desync::Executor;

#[cfg(feature = "primitive_async_impl")]
pub type MainExecutor = super::primitive::PrimitiveExecutor;

// if you are reading this i sincerely apologize
pub static mut Executor: spin::Lazy<RefCell<Box<dyn Executor>>> = spin::Lazy::new(|| RefCell::new(Box::new({
    #[cfg(feature = "primitive_async_impl")]
    {
        super::primitive::PrimitiveExecutor::new()
    }
    #[cfg(feature = "stable_async_impl")]
    {
        super::stable::StableExecutor::new()
    }
})));