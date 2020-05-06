use crate::libc::{c_char, c_void, size_t, c_long};
use crate::core_foundation_sys::base::{OSStatus, Boolean, CFAllocatorRef, CFTypeID, CFTypeRef};
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::string::CFStringRef;

use crate::sync::CMClockRef;

pub const kCMBlockBufferNoErr: OSStatus                         = 0;
pub const kCMBlockBufferStructureAllocationFailedErr: OSStatus  = -12700;
pub const kCMBlockBufferBlockAllocationFailedErr: OSStatus      = -12701;
pub const kCMBlockBufferBadCustomBlockSourceErr: OSStatus       = -12702;
pub const kCMBlockBufferBadOffsetParameterErr: OSStatus         = -12703;
pub const kCMBlockBufferBadLengthParameterErr: OSStatus         = -12704;
pub const kCMBlockBufferBadPointerParameterErr: OSStatus        = -12705;
pub const kCMBlockBufferEmptyBBufErr: OSStatus                  = -12706;
pub const kCMBlockBufferUnallocatedBlockErr: OSStatus           = -12707;
pub const kCMBlockBufferInsufficientSpaceErr: OSStatus          = -12708;


pub type CMBlockBufferFlags = u32;

pub const kCMBlockBufferAssureMemoryNowFlag: CMBlockBufferFlags       = 1<<0;
pub const kCMBlockBufferAlwaysCopyDataFlag: CMBlockBufferFlags        = 1<<1;
pub const kCMBlockBufferDontOptimizeDepthFlag: CMBlockBufferFlags     = 1<<2;
pub const kCMBlockBufferPermitEmptyReferenceFlag: CMBlockBufferFlags  = 1<<3;

pub type CMBlockBufferRef = CFTypeRef;

#[repr(C)]
#[allow(missing_copy_implementations)]
#[derive(Debug, Copy, Clone)]
pub struct CMBlockBufferCustomBlockSource {
    pub version: u32,
    pub AllocateBlock: extern "C" fn(refCon: *mut c_void, sizeInBytes: size_t) -> *mut c_void,
    pub FreeBlock: extern "C" fn(refCon: *mut c_void, doomedMemoryBlock: *mut c_void, sizeInBytes: size_t),
    pub refCon: *mut c_void,
}

pub const kCMBlockBufferCustomBlockSourceVersion: u32 = 0;


extern "C" {
    pub fn CMBlockBufferCreateEmpty(structureAllocator: CFAllocatorRef,
                                    subBlockCapacity: u32,
                                    flags: CMBlockBufferFlags,
                                    newBBufOut: *mut CMBlockBufferRef) -> OSStatus;
    pub fn CMBlockBufferCreateWithMemoryBlock(structureAllocator: CFAllocatorRef,
                                              memoryBlock: *mut c_void,
                                              blockLength: size_t,
                                              blockAllocator: CFAllocatorRef,
                                              customBlockSource: *const CMBlockBufferCustomBlockSource,
                                              offsetToData: size_t,
                                              dataLength: size_t,
                                              flags: CMBlockBufferFlags,
                                              newBBufOut: *mut CMBlockBufferRef)
                                              -> OSStatus;
    pub fn CMBlockBufferCreateWithBufferReference(structureAllocator: CFAllocatorRef,
                                                  targetBuffer: CMBlockBufferRef,
                                                  offsetToData: size_t,
                                                  dataLength: size_t,
                                                  flags: CMBlockBufferFlags,
                                                  newBBufOut: *mut CMBlockBufferRef) -> OSStatus;
    pub fn CMBlockBufferCreateContiguous(structureAllocator: CFAllocatorRef,
                                         sourceBuffer: CMBlockBufferRef,
                                         blockAllocator: CFAllocatorRef,
                                         customBlockSource: *const CMBlockBufferCustomBlockSource,
                                         offsetToData: size_t,
                                         dataLength: size_t,
                                         flags: CMBlockBufferFlags,
                                         newBBufOut: *mut CMBlockBufferRef) -> OSStatus;
    pub fn CMBlockBufferGetTypeID() -> CFTypeID;
    pub fn CMBlockBufferAppendMemoryBlock(theBuffer: CMBlockBufferRef,
                                          memoryBlock: *mut c_void,
                                          blockLength: size_t,
                                          blockAllocator: CFAllocatorRef,
                                          customBlockSource: *const CMBlockBufferCustomBlockSource,
                                          offsetToData: size_t,
                                          dataLength: size_t,
                                          flags: CMBlockBufferFlags) -> OSStatus;
    pub fn CMBlockBufferAppendBufferReference(theBuffer: CMBlockBufferRef,
                                              targetBBuf: CMBlockBufferRef,
                                              offsetToData: size_t,
                                              dataLength: size_t,
                                              flags: CMBlockBufferFlags) -> OSStatus;
    pub fn CMBlockBufferAssureBlockMemory(theBuffer: CMBlockBufferRef) -> OSStatus;
    pub fn CMBlockBufferAccessDataBytes(theBuffer: CMBlockBufferRef,
                                        offset: size_t,
                                        length: size_t,
                                        temporaryBlock: *mut c_void,
                                        returnedPointer: *mut c_char) -> OSStatus;
    pub fn CMBlockBufferCopyDataBytes(theSourceBuffer: CMBlockBufferRef,
                                      offsetToData: size_t,
                                      dataLength: size_t,
                                      destination: *mut c_void) -> OSStatus;
    pub fn CMBlockBufferReplaceDataBytes(sourceBytes: *const c_void,
                                         destinationBuffer: CMBlockBufferRef,
                                         offsetIntoDestination: size_t,
                                         dataLength: size_t) -> OSStatus;
    pub fn CMBlockBufferFillDataBytes(fillByte: c_char,
                                      destinationBuffer: CMBlockBufferRef,
                                      offsetIntoDestination: size_t,
                                      dataLength: size_t) -> OSStatus;
    pub fn CMBlockBufferGetDataPointer(theBuffer: CMBlockBufferRef,
                                       offset: size_t,
                                       lengthAtOffset: *mut size_t,
                                       totalLength: *mut size_t,
                                       dataPointer: *mut c_char) -> OSStatus;
    pub fn CMBlockBufferGetDataLength(theBuffer: CMBlockBufferRef) -> size_t;
    pub fn CMBlockBufferIsRangeContiguous(theBuffer: CMBlockBufferRef,
                                          offset: size_t,
                                          length: size_t) -> Boolean;
    pub fn CMBlockBufferIsEmpty(theBuffer: CMBlockBufferRef) -> Boolean;
}