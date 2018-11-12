#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]
#![cfg(any(target_os = "macos", target_os = "ios"))]

extern crate libc;
extern crate core_foundation_sys;


use libc::{c_void, size_t, c_long};
use core_foundation_sys::base::{OSStatus, Boolean, CFAllocatorRef, CFTypeID, CFTypeRef};
use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::string::CFStringRef;


pub type OSType = u32;

pub type CMItemCount = c_long;
pub type CMItemIndex = c_long;

pub type CMTimeValue = i64;
pub type CMTimeScale = i32;
pub type CMTimeFlags = u32;
pub type CMTimeEpoch = i64;

pub type CMVideoCodecType = u32;








pub type CMBlockBufferFlags = u32;
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct OpaqueCMBlockBuffer;
// pub type CMBlockBufferRef = *mut OpaqueCMBlockBuffer;
pub type CMBlockBufferRef = CFTypeRef;

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct OpaqueCMFormatDescription;
// pub type CMFormatDescriptionRef = *mut OpaqueCMFormatDescription;
pub type CMFormatDescriptionRef = CFTypeRef;
pub type CMVideoFormatDescriptionRef = CMFormatDescriptionRef;

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct OpaqueCMSampleBuffer;
// pub type CMSampleBufferRef = *mut OpaqueCMSampleBuffer;
pub type CMSampleBufferRef = CFTypeRef;

pub type CMSampleBufferMakeDataReadyCallback =
    extern "C" fn(sbuf: CMSampleBufferRef, makeDataReadyRefcon: *mut c_void)
                  -> OSStatus;


pub const kCMTimeFlags_Valid: CMTimeFlags = 1 << 0;
#[allow(non_upper_case_globals)]
pub const kCMVideoCodecType_H264: CMVideoCodecType =
    ((b'a' as u32) << 24) | ((b'v' as u32) << 16) | ((b'c' as u32) << 8) | (b'1' as u32);


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMTime {
    pub value: CMTimeValue,
    pub timescale: CMTimeScale,
    pub flags: CMTimeFlags,
    pub epoch: CMTimeEpoch,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMSampleTimingInfo {
    pub duration: CMTime,
    pub presentation_time_stamp: CMTime,
    pub decode_time_stamp: CMTime,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMTimeRange {
    pub start: CMTime,
    pub duration: CMTime,
}




#[repr(C)]
#[allow(missing_copy_implementations)]
#[derive(Debug, Copy, Clone)]
pub struct CMBlockBufferCustomBlockSource {
    version: u32,
    AllocateBlock: extern "C" fn(refCon: *mut c_void, sizeInBytes: size_t) -> *mut c_void,
    FreeBlock: extern "C" fn(refCon: *mut c_void, doomedMemoryBlock: *mut c_void, sizeInBytes: size_t),
    refCon: *mut c_void,
}


#[link(name="CoreMedia", kind="framework")]
extern {
    pub static kCMFormatDescriptionExtension_SampleDescriptionExtensionAtoms: CFStringRef;

    pub fn CMFormatDescriptionGetTypeID() -> CFTypeID;
    pub fn CMVideoFormatDescriptionCreate(allocator: CFAllocatorRef,
                                          codecType: CMVideoCodecType,
                                          width: i32,
                                          height: i32,
                                          extensions: CFDictionaryRef,
                                          outDesc: *mut CMVideoFormatDescriptionRef)
                                          -> OSStatus;
    pub fn CMBlockBufferGetTypeID() -> CFTypeID;
    pub fn CMBlockBufferCreateWithMemoryBlock(structureAllocator: CFAllocatorRef,
                                              memoryBlock: *mut c_void,
                                              blockLength: size_t,
                                              blockAllocator: CFAllocatorRef,
                                              customBlockSource: *const
                                                CMBlockBufferCustomBlockSource,
                                              offsetToData: size_t,
                                              dataLength: size_t,
                                              flags: CMBlockBufferFlags,
                                              newBBufOut: *mut CMBlockBufferRef)
                                              -> OSStatus;
    pub fn CMBlockBufferReplaceDataBytes(sourceBytes: *const c_void,
                                         destinationBuffer: CMBlockBufferRef,
                                         offsetIntoDestination: size_t,
                                         dataLength: size_t)
                                         -> OSStatus;
    pub fn CMSampleBufferGetTypeID() -> CFTypeID;
    pub fn CMSampleBufferCreate(allocator: CFAllocatorRef,
                                dataBuffer: CMBlockBufferRef,
                                dataReady: Boolean,
                                makeDataReadyCallback: CMSampleBufferMakeDataReadyCallback,
                                makeDataReadyRefcon: *mut c_void,
                                formatDescription: CMFormatDescriptionRef,
                                numSamples: CMItemCount,
                                numSampleTimingEntries: CMItemCount,
                                sampleTimingArray: *const CMSampleTimingInfo,
                                numSampleSizeEntries: CMItemCount,
                                sampleSizeArray: *const size_t,
                                sBufOut: *mut CMSampleBufferRef)
                                -> OSStatus;
    pub fn CMSampleBufferGetSampleTimingInfo(sbuf: CMSampleBufferRef,
                                             sampleIndex: CMItemIndex,
                                             timingInfoOut: *mut CMSampleTimingInfo)
                                             -> OSStatus;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test(){
        unsafe {
            assert_eq!(CMFormatDescriptionGetTypeID(), 269);
        }
    }
}