use crate::libc::{c_char, c_void, size_t, c_long};
use crate::core_foundation_sys::base::{OSStatus, Boolean, CFAllocatorRef, CFTypeID, CFTypeRef};
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::string::{CFStringRef, CFStringEncoding};

use crate::sync::CMClockRef;
use crate::block_buffer::CMBlockBufferRef;
use crate::format_description::CMVideoFormatDescriptionRef;



pub const kCMFormatDescriptionBridgeError_InvalidParameter: OSStatus                    = -12712;
pub const kCMFormatDescriptionBridgeError_AllocationFailed: OSStatus                    = -12713;
pub const kCMFormatDescriptionBridgeError_InvalidSerializedSampleDescription: OSStatus  = -12714;
pub const kCMFormatDescriptionBridgeError_InvalidFormatDescription: OSStatus            = -12715;
pub const kCMFormatDescriptionBridgeError_IncompatibleFormatDescription: OSStatus       = -12716;
pub const kCMFormatDescriptionBridgeError_UnsupportedSampleDescriptionFlavor: OSStatus  = -12717;
pub const kCMFormatDescriptionBridgeError_InvalidSlice: OSStatus                        = -12719;


extern "C" {
    pub static kCMImageDescriptionFlavor_QuickTimeMovie: CFStringRef;
    pub static kCMImageDescriptionFlavor_ISOFamily: CFStringRef;
    pub static kCMImageDescriptionFlavor_3GPFamily: CFStringRef;

    
    pub fn CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionData(
        allocator: CFAllocatorRef,
        imageDescriptionData: *const u8,
        imageDescriptionSize: size_t,
        imageDescriptionStringEncoding: CFStringEncoding,
        imageDescriptionFlavor: CFStringRef,
        videoFormatDescriptionOut: *mut CMVideoFormatDescriptionRef,
    ) -> OSStatus;
    pub fn CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionBlockBuffer(
        allocator: CFAllocatorRef,
        imageDescriptionBlockBuffer: CMBlockBufferRef,
        imageDescriptionStringEncoding: CFStringEncoding,
        imageDescriptionFlavor: CFStringRef,
        videoFormatDescriptionOut: *mut CMVideoFormatDescriptionRef
    ) -> OSStatus;

}