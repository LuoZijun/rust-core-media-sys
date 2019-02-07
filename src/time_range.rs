use crate::libc::{c_char, c_void, size_t, c_long, int32_t, int64_t, uint32_t, uint64_t};
use crate::core_foundation_sys::base::{OSStatus, Boolean, CFAllocatorRef, CFTypeID, CFTypeRef};
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::string::CFStringRef;

use crate::sync::CMClockRef;
use crate::time::CMTime;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMTimeRange {
    pub start: CMTime,
    pub duration: CMTime,
}