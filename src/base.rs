use crate::libc::{c_void, size_t, c_long};
use crate::core_foundation_sys::base::{OSStatus, Boolean, CFAllocatorRef, CFTypeID, CFTypeRef};
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::string::CFStringRef;

use crate::sync::CMClockRef;


cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        pub type CMItemCount = i64;
        pub type CMItemIndex = i64;
    } else {
        pub type CMItemCount = c_long;
        pub type CMItemIndex = c_long;
    }
}

cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        pub type CMBaseClassVersion = u32;
        pub type CMStructVersion = u32;
    } else if #[cfg(target_pointer_width = "64")] {
        pub type CMBaseClassVersion = u64;
        pub type CMStructVersion = u64;
    } else {
        pub type CMBaseClassVersion = u32;
        pub type CMStructVersion = u32;
    }
}


pub type CMPersistentTrackID = i32;
pub const kCMPersistentTrackID_Invalid: CMPersistentTrackID = 0;

