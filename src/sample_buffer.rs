use crate::libc::{c_char, c_void, size_t, c_long, int32_t, int64_t, uint32_t, uint64_t};
use crate::core_foundation_sys::base::{OSStatus, Boolean, CFAllocatorRef, CFTypeID, CFTypeRef};
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::string::CFStringRef;

use crate::base::CMItemIndex;
use crate::sync::CMClockRef;
use crate::time::CMTime;


pub type CMSampleBufferRef = *mut c_void;
pub type CMSampleBufferMakeDataReadyCallback = extern "C" fn(sbuf: CMSampleBufferRef, 
                                                             makeDataReadyRefcon: *mut c_void) -> OSStatus;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMSampleTimingInfo {
    pub duration: CMTime,
    pub presentation_time_stamp: CMTime,
    pub decode_time_stamp: CMTime,
}


extern "C" {
    pub fn CMSampleBufferGetSampleTimingInfo(sbuf: CMSampleBufferRef,
                                             sampleIndex: CMItemIndex,
                                             timingInfoOut: *mut CMSampleTimingInfo) -> OSStatus;
}