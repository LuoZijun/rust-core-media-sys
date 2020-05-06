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