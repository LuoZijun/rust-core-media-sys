#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes,
    dead_code, unused_imports, )]

#[macro_use]
extern crate cfg_if;
extern crate libc;
extern crate core_foundation_sys;


use libc::{c_void, size_t, c_long};
use core_foundation_sys::base::{OSStatus, Boolean, CFAllocatorRef, CFTypeID, CFTypeRef};
use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::string::CFStringRef;


pub mod attachment;
pub mod buffer_queue;
pub mod memory_pool;
pub mod sync;
pub mod audio_device_clock;
pub mod format_description;
pub mod metadata;
pub mod time;
pub mod base;
pub mod format_description_bridge;
pub mod sample_buffer;
pub mod time_range;
pub mod block_buffer;
pub mod sample_queue;


pub use self::attachment::*;
pub use self::buffer_queue::*;
pub use self::memory_pool::*;
pub use self::sync::*;
pub use self::audio_device_clock::*;
pub use self::format_description::*;
pub use self::metadata::*;
pub use self::time::*;
pub use self::base::*;
pub use self::format_description_bridge::*;
pub use self::sample_buffer::*;
pub use self::time_range::*;
pub use self::block_buffer::*;
pub use self::sample_queue::*;