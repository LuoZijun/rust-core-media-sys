use crate::libc::{c_void, size_t, c_long, int32_t, int64_t, uint32_t, uint64_t};
use crate::core_foundation_sys::base::{OSStatus, Boolean, CFAllocatorRef, CFTypeID, CFTypeRef};
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::string::CFStringRef;

use crate::sync::CMClockRef;


cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        pub type CMItemCount = int64_t;
        pub type CMItemIndex = int64_t;
    } else {
        pub type CMItemCount = c_long;
        pub type CMItemIndex = c_long;
    }
}

cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        pub type CMBaseClassVersion = uint32_t;
        pub type CMStructVersion = uint32_t;
    } else if #[cfg(target_pointer_width = "64")] {
        pub type CMBaseClassVersion = uint64_t;
        pub type CMStructVersion = uint64_t;
    } else {
        pub type CMBaseClassVersion = uint32_t;
        pub type CMStructVersion = uint32_t;
    }
}


pub type CMPersistentTrackID = int32_t;
pub const kCMPersistentTrackID_Invalid: CMPersistentTrackID = 0;

