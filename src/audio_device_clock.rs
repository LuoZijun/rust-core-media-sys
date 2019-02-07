use crate::libc::{c_void, size_t, c_long, uint32_t};
use crate::core_foundation_sys::base::{OSStatus, Boolean, CFAllocatorRef, CFTypeID, CFTypeRef};
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::string::CFStringRef;

use crate::sync::CMClockRef;


// https://developer.apple.com/documentation/coreaudio/audioobjectid?language=objc
pub type AudioObjectID = uint32_t;
// https://developer.apple.com/documentation/coreaudio/audiodeviceid
pub type AudioDeviceID = AudioObjectID;

pub const kAudioObjectUnknown: AudioObjectID = 0;
pub const kAudioStreamUnknown: AudioObjectID = kAudioObjectUnknown;



extern "C" {
    pub fn CMAudioDeviceClockCreate(allocator: CFAllocatorRef,
                                    deviceUID: CFStringRef,
                                    clockOut: *mut CMClockRef) -> OSStatus;
    pub fn CMAudioDeviceClockCreateFromAudioDeviceID(allocator: CFAllocatorRef,
                                                     deviceID: AudioDeviceID,
                                                     clockOut: *mut CMClockRef) -> OSStatus;
    pub fn CMAudioDeviceClockSetAudioDeviceUID(clock: CMClockRef,
                                               deviceUID: CFStringRef) -> OSStatus;
    pub fn CMAudioDeviceClockSetAudioDeviceID(clock: CMClockRef,
                                              deviceID: AudioDeviceID) -> OSStatus;
    pub fn CMAudioDeviceClockGetAudioDevice(clock: CMClockRef,
                                            deviceUIDOut: *mut CFStringRef,
                                            deviceIDOut: *mut AudioDeviceID,
                                            trackingDefaultDeviceOut: *mut Boolean) -> OSStatus;

}