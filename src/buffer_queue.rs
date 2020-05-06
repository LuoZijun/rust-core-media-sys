use crate::libc::{c_void, size_t};
use crate::core_foundation_sys::base::{OSStatus, Boolean, CFAllocatorRef, CFTypeID, CFTypeRef, CFComparisonResult};
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::string::CFStringRef;

use crate::base::CMItemCount;
use crate::sync::CMClockRef;
use crate::time::CMTime;


pub const kCMBufferQueueError_AllocationFailed: OSStatus                        = -12760;
pub const kCMBufferQueueError_RequiredParameterMissing: OSStatus                = -12761;
pub const kCMBufferQueueError_InvalidCMBufferCallbacksStruct: OSStatus          = -12762;
pub const kCMBufferQueueError_EnqueueAfterEndOfData: OSStatus                   = -12763;
pub const kCMBufferQueueError_QueueIsFull: OSStatus                             = -12764;
pub const kCMBufferQueueError_BadTriggerDuration: OSStatus                      = -12765;
pub const kCMBufferQueueError_CannotModifyQueueFromTriggerCallback: OSStatus    = -12766;
pub const kCMBufferQueueError_InvalidTriggerCondition: OSStatus                 = -12767;
pub const kCMBufferQueueError_InvalidTriggerToken: OSStatus                     = -12768;
pub const kCMBufferQueueError_InvalidBuffer: OSStatus                           = -12769;


pub type CMBufferQueueRef = *mut c_void;
pub type CMBufferRef = CFTypeRef;

pub type CMBufferGetTimeCallback = extern "C" fn (buf: CMBufferRef, refcon: *mut c_void) -> CMTime;
pub type CMBufferGetBooleanCallback = extern "C" fn (buf: CMBufferRef, refcon: *mut c_void) -> Boolean;
pub type CMBufferCompareCallback = extern "C" fn (buf1: CMBufferRef,
                                                  buf2: CMBufferRef,
                                                  refcon: *mut c_void) -> CFComparisonResult;
pub type CMBufferGetSizeCallback = extern "C" fn (buf: CMBufferRef, refcon: *mut c_void) -> size_t;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMBufferCallbacks {
    pub version: u32,
    pub refcon: *mut c_void,
    pub getDecodeTimeStamp: CMBufferGetTimeCallback,
    pub getPresentationTimeStamp: CMBufferGetTimeCallback,
    pub getDuration: CMBufferGetTimeCallback,
    pub isDataReady: CMBufferGetBooleanCallback,
    pub compare: CMBufferCompareCallback,
    pub dataBecameReadyNotification: CFStringRef,
    pub getSize: CMBufferGetSizeCallback
}


pub type CMBufferQueueTriggerToken = *mut c_void;
pub type CMBufferQueueTriggerCallback = extern "C" fn (triggerRefcon: *mut c_void,
                                                       triggerToken: CMBufferQueueTriggerToken);
pub type CMBufferQueueTriggerCondition = i32;
pub const kCMBufferQueueTrigger_WhenDurationBecomesLessThan: CMBufferQueueTriggerCondition = 1;
pub const kCMBufferQueueTrigger_WhenDurationBecomesLessThanOrEqualTo: CMBufferQueueTriggerCondition = 2;
pub const kCMBufferQueueTrigger_WhenDurationBecomesGreaterThan: CMBufferQueueTriggerCondition = 3;
pub const kCMBufferQueueTrigger_WhenDurationBecomesGreaterThanOrEqualTo: CMBufferQueueTriggerCondition = 4;
pub const kCMBufferQueueTrigger_WhenMinPresentationTimeStampChanges: CMBufferQueueTriggerCondition = 5;
pub const kCMBufferQueueTrigger_WhenMaxPresentationTimeStampChanges: CMBufferQueueTriggerCondition = 6;
pub const kCMBufferQueueTrigger_WhenDataBecomesReady: CMBufferQueueTriggerCondition = 7;
pub const kCMBufferQueueTrigger_WhenEndOfDataReached: CMBufferQueueTriggerCondition = 8;
pub const kCMBufferQueueTrigger_WhenReset: CMBufferQueueTriggerCondition = 9;
pub const kCMBufferQueueTrigger_WhenBufferCountBecomesLessThan: CMBufferQueueTriggerCondition = 10;
pub const kCMBufferQueueTrigger_WhenBufferCountBecomesGreaterThan: CMBufferQueueTriggerCondition = 11;
pub const kCMBufferQueueTrigger_WhenDurationBecomesGreaterThanOrEqualToAndBufferCountBecomesGreaterThan: CMBufferQueueTriggerCondition = 12;

pub type CMBufferValidationCallback = extern "C" fn (queue: CMBufferQueueRef,
                                                     buf: CMBufferRef,
                                                     validationRefCon: *mut c_void) -> OSStatus;


extern "C" {
    pub fn CMBufferQueueGetCallbacksForUnsortedSampleBuffers() -> *const CMBufferCallbacks;
    pub fn CMBufferQueueGetCallbacksForSampleBuffersSortedByOutputPTS() -> *const CMBufferCallbacks;
    pub fn CMBufferQueueCreate(allocator: CFAllocatorRef,
                               capacity: CMItemCount,
                               callbacks: *const CMBufferCallbacks,
                               queueOut: *mut CMBufferQueueRef) -> OSStatus;
    pub fn CMBufferQueueGetTypeID() -> CFTypeID;
    pub fn CMBufferQueueEnqueue(queue: CMBufferQueueRef,
                                buf: CMBufferRef) -> OSStatus;
    pub fn CMBufferQueueDequeueAndRetain(queue: CMBufferQueueRef) -> CMBufferRef;
    pub fn CMBufferQueueDequeueIfDataReadyAndRetain(queue: CMBufferQueueRef) -> CMBufferRef;
    pub fn CMBufferQueueGetHead(queue: CMBufferQueueRef) -> CMBufferRef;
    pub fn CMBufferQueueIsEmpty(queue: CMBufferQueueRef) -> Boolean;
    pub fn CMBufferQueueMarkEndOfData(queue: CMBufferQueueRef) -> OSStatus;
    pub fn CMBufferQueueContainsEndOfData(queue: CMBufferQueueRef) -> Boolean;
    pub fn CMBufferQueueIsAtEndOfData(queue: CMBufferQueueRef) -> Boolean;
    pub fn CMBufferQueueReset(queue: CMBufferQueueRef) -> OSStatus;
    pub fn CMBufferQueueResetWithCallback(queue: CMBufferQueueRef,
                                          callback: extern "C" fn (buffer: CMBufferRef, refcon: *mut c_void),
                                          refcon: *mut c_void) -> OSStatus;
    pub fn CMBufferQueueGetBufferCount(queue: CMBufferQueueRef) -> CMItemCount;
    pub fn CMBufferQueueGetDuration(queue: CMBufferQueueRef) -> CMTime;
    pub fn CMBufferQueueGetMinDecodeTimeStamp(queue: CMBufferQueueRef) -> CMTime;
    pub fn CMBufferQueueGetFirstDecodeTimeStamp(queue: CMBufferQueueRef) -> CMTime;
    pub fn CMBufferQueueGetMinPresentationTimeStamp(queue: CMBufferQueueRef) -> CMTime;
    pub fn CMBufferQueueGetFirstPresentationTimeStamp(queue: CMBufferQueueRef) -> CMTime;
    pub fn CMBufferQueueGetMaxPresentationTimeStamp(queue: CMBufferQueueRef) -> CMTime;
    pub fn CMBufferQueueGetEndPresentationTimeStamp(queue: CMBufferQueueRef) -> CMTime;
    pub fn CMBufferQueueGetTotalSize(queue: CMBufferQueueRef) -> size_t;


    pub fn CMBufferQueueInstallTrigger(queue: CMBufferQueueRef,
                                       triggerCallback: CMBufferQueueTriggerCallback,
                                       triggerRefcon: *mut c_void,
                                       triggerCondition: CMBufferQueueTriggerCondition,
                                       triggerTime: CMTime,
                                       triggerTokenOut: *mut CMBufferQueueTriggerToken) -> OSStatus;
    pub fn CMBufferQueueInstallTriggerWithIntegerThreshold(queue: CMBufferQueueRef,
                                                           triggerCallback: CMBufferQueueTriggerCallback,
                                                           triggerRefcon: *mut c_void,
                                                           triggerCondition: CMBufferQueueTriggerCondition,
                                                           triggerThreshold: CMItemCount,
                                                           triggerTokenOut: *mut CMBufferQueueTriggerToken) -> OSStatus;
    pub fn CMBufferQueueRemoveTrigger(queue: CMBufferQueueRef,
                                      triggerToken: CMBufferQueueTriggerToken) -> OSStatus;
    pub fn CMBufferQueueTestTrigger(queue: CMBufferQueueRef,
                                    triggerToken: CMBufferQueueTriggerToken) -> Boolean;
    pub fn CMBufferQueueCallForEachBuffer(queue: CMBufferQueueRef,
                                          callback: extern "C" fn (buffer: CMBufferRef,
                                                                   refcon: *mut c_void) -> OSStatus,
                                          refcon: *mut c_void) -> OSStatus;


    pub fn CMBufferQueueSetValidationCallback(queue: CMBufferQueueRef,
                                              validationCallback: CMBufferValidationCallback,
                                              validationRefCon: *mut c_void) -> OSStatus;
}
