use crate::core_foundation_sys::base::{OSStatus, Boolean, CFAllocatorRef, CFTypeID, CFTypeRef};
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::string::CFStringRef;

use crate::sync::CMClockRef;


pub type CMTimeValue = i64;
pub type CMTimeScale = i32;
pub type CMTimeEpoch = i64;
pub type CMTimeFlags = u32;

pub const kCMTimeFlags_Valid: CMTimeFlags = 1<<0;
pub const kCMTimeFlags_HasBeenRounded: CMTimeFlags = 1<<1;
pub const kCMTimeFlags_PositiveInfinity: CMTimeFlags = 1<<2;
pub const kCMTimeFlags_NegativeInfinity: CMTimeFlags = 1<<3;
pub const kCMTimeFlags_Indefinite: CMTimeFlags = 1<<4;
pub const kCMTimeFlags_ImpliedValueFlagsMask: CMTimeFlags = kCMTimeFlags_PositiveInfinity 
                                                            | kCMTimeFlags_NegativeInfinity
                                                            | kCMTimeFlags_Indefinite;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMTime {
    pub value: CMTimeValue,
    pub timescale: CMTimeScale,
    pub flags: CMTimeFlags,
    pub epoch: CMTimeEpoch,
}


extern "C" {
    pub static kCMTimeInvalid: CMTime;
    pub static kCMTimeIndefinite: CMTime;
    pub static kCMTimePositiveInfinity: CMTime;
    pub static kCMTimeNegativeInfinity: CMTime;
    pub static kCMTimeZero: CMTime;
    

}