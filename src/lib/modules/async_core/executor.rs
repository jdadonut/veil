use core::cell::{Cell, Ref, RefCell};

use alloc::{boxed::{Box, ThinBox}, vec::Vec};
use lazy_static::lazy_static;
use spin::{Mutex, Lazy};

use crate::{std::desync::Executor, lib::veil_std::desync::Task};

#[cfg(feature = "primitive_async_impl")]
pub type MainExecutor = super::primitive::PrimitiveExecutor;


// if you are reading this i sincerely apologize
// pub static mut Executor: spin::Lazy<RefCell<Box<dyn Executor>>> = spin::Lazy::new(|| RefCell::new(Box::new({
//     #[cfg(feature = "primitive_async_impl")]
//     {
//         super::primitive::PrimitiveExecutor::new()
//     }
//     #[cfg(feature = "stable_async_impl")]
//     {
//         super::stable::StableExecutor::new()
//     }
// })));
#[cfg(feature = "primitive_async_impl")]
pub type UsedExecutor = super::primitive::PrimitiveExecutor;
#[cfg(feature = "stable_async_impl")]
pub type UsedExecutor = super::stable::StableExecutor;

pub static mut Executor: Lazy<RefCell<UsedExecutor>> = Lazy::new(|| RefCell::new(UsedExecutor::new()));
