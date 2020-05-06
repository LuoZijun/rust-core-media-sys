use crate::core_foundation_sys::base::{OSStatus, Boolean, CFAllocatorRef, CFTypeID, CFTypeRef};
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::string::CFStringRef;


pub type CMAttachmentBearerRef = CFTypeRef;

pub type CMAttachmentMode = u32;
pub const kCMAttachmentMode_ShouldNotPropagate: CMAttachmentMode = 0;
pub const kCMAttachmentMode_ShouldPropagate: CMAttachmentMode    = 1;


extern "C" {
    pub fn CMSetAttachment(target: CMAttachmentBearerRef,
                           key: CFStringRef,
                           value: CFTypeRef,
                           attachmentMode: CMAttachmentMode);
    pub fn CMGetAttachment(target: CMAttachmentBearerRef,
                           key: CFStringRef,
                           attachmentModeOut: *mut CMAttachmentMode) -> CFTypeRef;
    pub fn CMRemoveAttachment(target: CMAttachmentBearerRef,
                              key: CFStringRef);
    pub fn CMRemoveAllAttachments(target: CMAttachmentBearerRef);
    pub fn CMCopyDictionaryOfAttachments(allocator: CFAllocatorRef,
                                         target: CMAttachmentBearerRef,
                                         attachmentMode: CMAttachmentMode) -> CFDictionaryRef;
    pub fn CMSetAttachments(target: CMAttachmentBearerRef,
                            theAttachments: CFDictionaryRef,
                            attachmentMode: CMAttachmentMode);
    pub fn CMPropagateAttachments(source: CMAttachmentBearerRef,
                                  destination: CMAttachmentBearerRef);
}

