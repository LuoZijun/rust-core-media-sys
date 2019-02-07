use crate::libc::{c_char, c_void, size_t, c_long, int32_t, int64_t, uint32_t, uint64_t};
use crate::core_foundation_sys::base::{OSStatus, Boolean, CFAllocatorRef, CFTypeID, CFTypeRef};
use crate::core_foundation_sys::dictionary::CFDictionaryRef;
use crate::core_foundation_sys::string::CFStringRef;

use crate::sync::CMClockRef;


pub const kCMFormatDescriptionError_InvalidParameter: OSStatus      = -12710;
pub const kCMFormatDescriptionError_AllocationFailed: OSStatus      = -12711;
pub const kCMFormatDescriptionError_ValueNotAvailable: OSStatus     = -12718;

pub type CMFormatDescriptionRef = *mut c_void;
pub type CMAudioFormatDescriptionRef = CMFormatDescriptionRef;
pub type CMVideoFormatDescriptionRef = CMFormatDescriptionRef;


// https://developer.apple.com/documentation/kernel/fourcharcode?language=objc
pub type FourCharCode = uint32_t;

pub type CMMediaType       = FourCharCode;
pub type CMAudioCodecType  = FourCharCode;
pub type CMVideoCodecType  = FourCharCode;
pub type CMPixelFormatType = FourCharCode;


const fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}


pub const kCMMediaType_Video: CMMediaType              = as_u32_be(b"vide");
pub const kCMMediaType_Audio: CMMediaType              = as_u32_be(b"soun");
pub const kCMMediaType_Muxed: CMMediaType              = as_u32_be(b"muxx");
pub const kCMMediaType_Text: CMMediaType               = as_u32_be(b"text");
pub const kCMMediaType_ClosedCaption: CMMediaType      = as_u32_be(b"clcp");
pub const kCMMediaType_Subtitle: CMMediaType           = as_u32_be(b"sbtl");
pub const kCMMediaType_TimeCode: CMMediaType           = as_u32_be(b"tmcd");
pub const kCMMediaType_Metadata: CMMediaType           = as_u32_be(b"meta");

pub const kCMAudioCodecType_AAC_LCProtected: CMAudioCodecType      = as_u32_be(b"paac");
pub const kCMAudioCodecType_AAC_AudibleProtected: CMAudioCodecType = as_u32_be(b"aaac");

pub type CMAudioFormatDescriptionMask = uint32_t;
pub const kCMAudioFormatDescriptionMask_StreamBasicDescription: CMAudioFormatDescriptionMask    = (1<<0);
pub const kCMAudioFormatDescriptionMask_MagicCookie: CMAudioFormatDescriptionMask               = (1<<1);
pub const kCMAudioFormatDescriptionMask_ChannelLayout: CMAudioFormatDescriptionMask             = (1<<2);
pub const kCMAudioFormatDescriptionMask_Extensions: CMAudioFormatDescriptionMask                = (1<<3);
pub const kCMAudioFormatDescriptionMask_All: CMAudioFormatDescriptionMask                       = kCMAudioFormatDescriptionMask_StreamBasicDescription
                                                                                                | kCMAudioFormatDescriptionMask_MagicCookie
                                                                                                | kCMAudioFormatDescriptionMask_ChannelLayout
                                                                                                | kCMAudioFormatDescriptionMask_Extensions;


pub const kCMPixelFormat_32ARGB: CMPixelFormatType                   = 32;
pub const kCMPixelFormat_32BGRA: CMPixelFormatType                   = as_u32_be(b"BGRA");
pub const kCMPixelFormat_24RGB: CMPixelFormatType                    = 24;
pub const kCMPixelFormat_16BE555: CMPixelFormatType                  = 16;
pub const kCMPixelFormat_16BE565: CMPixelFormatType                  = as_u32_be(b"B565");
pub const kCMPixelFormat_16LE555: CMPixelFormatType                  = as_u32_be(b"L555");
pub const kCMPixelFormat_16LE565: CMPixelFormatType                  = as_u32_be(b"L565");
pub const kCMPixelFormat_16LE5551: CMPixelFormatType                 = as_u32_be(b"5551");
pub const kCMPixelFormat_422YpCbCr8: CMPixelFormatType               = as_u32_be(b"2vuy");
pub const kCMPixelFormat_422YpCbCr8_yuvs: CMPixelFormatType          = as_u32_be(b"yuvs");
pub const kCMPixelFormat_444YpCbCr8: CMPixelFormatType               = as_u32_be(b"v308");
pub const kCMPixelFormat_4444YpCbCrA8: CMPixelFormatType             = as_u32_be(b"v408");
pub const kCMPixelFormat_422YpCbCr16: CMPixelFormatType              = as_u32_be(b"v216");
pub const kCMPixelFormat_422YpCbCr10: CMPixelFormatType              = as_u32_be(b"v210");
pub const kCMPixelFormat_444YpCbCr10: CMPixelFormatType              = as_u32_be(b"v410");
pub const kCMPixelFormat_8IndexedGray_WhiteIsZero: CMPixelFormatType = 0x00000028;



pub const kCMVideoCodecType_422YpCbCr8: CMVideoCodecType       = kCMPixelFormat_422YpCbCr8;
pub const kCMVideoCodecType_Animation: CMVideoCodecType        = as_u32_be(b"rle ");
pub const kCMVideoCodecType_Cinepak: CMVideoCodecType          = as_u32_be(b"cvid");
pub const kCMVideoCodecType_JPEG: CMVideoCodecType             = as_u32_be(b"jpeg");
pub const kCMVideoCodecType_JPEG_OpenDML: CMVideoCodecType     = as_u32_be(b"dmb1");
pub const kCMVideoCodecType_SorensonVideo: CMVideoCodecType    = as_u32_be(b"SVQ1");
pub const kCMVideoCodecType_SorensonVideo3: CMVideoCodecType   = as_u32_be(b"SVQ3");
pub const kCMVideoCodecType_H263: CMVideoCodecType             = as_u32_be(b"h263");
pub const kCMVideoCodecType_H264: CMVideoCodecType             = as_u32_be(b"avc1");
pub const kCMVideoCodecType_HEVC: CMVideoCodecType             = as_u32_be(b"hvc1");
pub const kCMVideoCodecType_MPEG4Video: CMVideoCodecType       = as_u32_be(b"mp4v");
pub const kCMVideoCodecType_MPEG2Video: CMVideoCodecType       = as_u32_be(b"mp2v");
pub const kCMVideoCodecType_MPEG1Video: CMVideoCodecType       = as_u32_be(b"mp1v");

pub const kCMVideoCodecType_DVCNTSC: CMVideoCodecType          = as_u32_be(b"dvc ");
pub const kCMVideoCodecType_DVCPAL: CMVideoCodecType           = as_u32_be(b"dvcp");
pub const kCMVideoCodecType_DVCProPAL: CMVideoCodecType        = as_u32_be(b"dvpp");
pub const kCMVideoCodecType_DVCPro50NTSC: CMVideoCodecType     = as_u32_be(b"dv5n");
pub const kCMVideoCodecType_DVCPro50PAL: CMVideoCodecType      = as_u32_be(b"dv5p");
pub const kCMVideoCodecType_DVCPROHD720p60: CMVideoCodecType   = as_u32_be(b"dvhp");
pub const kCMVideoCodecType_DVCPROHD720p50: CMVideoCodecType   = as_u32_be(b"dvhq");
pub const kCMVideoCodecType_DVCPROHD1080i60: CMVideoCodecType  = as_u32_be(b"dvh6");
pub const kCMVideoCodecType_DVCPROHD1080i50: CMVideoCodecType  = as_u32_be(b"dvh5");
pub const kCMVideoCodecType_DVCPROHD1080p30: CMVideoCodecType  = as_u32_be(b"dvh3");
pub const kCMVideoCodecType_DVCPROHD1080p25: CMVideoCodecType  = as_u32_be(b"dvh2");

pub const kCMVideoCodecType_AppleProRes4444: CMVideoCodecType  = as_u32_be(b"ap4h");
pub const kCMVideoCodecType_AppleProRes422HQ: CMVideoCodecType = as_u32_be(b"apch");
pub const kCMVideoCodecType_AppleProRes422: CMVideoCodecType   = as_u32_be(b"apcn");
pub const kCMVideoCodecType_AppleProRes422LT: CMVideoCodecType = as_u32_be(b"apcs");
pub const kCMVideoCodecType_AppleProRes422Proxy: CMVideoCodecType = as_u32_be(b"apco");


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMVideoDimensions {
    pub width: int32_t,
    pub height: int32_t,
}


extern "C" {
    pub static kCMFormatDescriptionExtension_OriginalCompressionSettings: CFStringRef;
    pub static kCMFormatDescriptionExtension_SampleDescriptionExtensionAtoms: CFStringRef;
    pub static kCMFormatDescriptionExtension_VerbatimSampleDescription: CFStringRef;
    pub static kCMFormatDescriptionExtension_VerbatimISOSampleEntry: CFStringRef;

    pub static kCMFormatDescriptionExtension_FormatName: CFStringRef;
    pub static kCMFormatDescriptionExtension_Depth: CFStringRef;

    pub static kCMFormatDescriptionExtension_CleanAperture: CFStringRef;
    pub static kCMFormatDescriptionKey_CleanApertureWidth: CFStringRef;
    pub static kCMFormatDescriptionKey_CleanApertureHeight: CFStringRef;
    pub static kCMFormatDescriptionKey_CleanApertureHorizontalOffset: CFStringRef;
    pub static kCMFormatDescriptionKey_CleanApertureVerticalOffset: CFStringRef;

    pub static kCMFormatDescriptionKey_CleanApertureWidthRational: CFStringRef;
    pub static kCMFormatDescriptionKey_CleanApertureHeightRational: CFStringRef;
    pub static kCMFormatDescriptionKey_CleanApertureHorizontalOffsetRational: CFStringRef;
    pub static kCMFormatDescriptionKey_CleanApertureVerticalOffsetRational: CFStringRef;
    pub static kCMFormatDescriptionExtension_FieldCount: CFStringRef;

    pub static kCMFormatDescriptionExtension_FieldDetail: CFStringRef;
    pub static kCMFormatDescriptionFieldDetail_TemporalTopFirst: CFStringRef;
    pub static kCMFormatDescriptionFieldDetail_TemporalBottomFirst: CFStringRef;
    pub static kCMFormatDescriptionFieldDetail_SpatialFirstLineEarly: CFStringRef;
    pub static kCMFormatDescriptionFieldDetail_SpatialFirstLineLate: CFStringRef;



    pub fn CMFormatDescriptionCreate(allocator: CFAllocatorRef,
                                     mediaType: CMMediaType,
                                     mediaSubtype: FourCharCode,
                                     extensions: CFDictionaryRef,
                                     descOut: *mut CMFormatDescriptionRef) -> OSStatus;

    pub fn CMFormatDescriptionGetTypeID() -> CFTypeID;
    pub fn CMFormatDescriptionEqual(desc1: CMFormatDescriptionRef,
                                    desc2: CMFormatDescriptionRef) -> Boolean;

    pub fn CMFormatDescriptionGetMediaType(desc: CMFormatDescriptionRef) -> CMMediaType;
    pub fn CMFormatDescriptionGetMediaSubType(desc: CMFormatDescriptionRef) -> FourCharCode;


    // /System/Library/Frameworks/CoreAudio.framework/Versions/A/Headers/CoreAudioTypes.h
    // pub fn CMAudioFormatDescriptionCreate(allocator: CFAllocatorRef,
    //                                       asbd: *const AudioStreamBasicDescription,
    //                                       layoutSize: size_t,
    //                                       layout: *const AudioChannelLayout,
    //                                       magicCookieSize: size_t,
    //                                       magicCookie: *const c_void,
    //                                       extensions: CFDictionaryRef,
    //                                       outDesc: *mut CMAudioFormatDescriptionRef) -> OSStatus;

    pub fn CMVideoFormatDescriptionCreate(allocator: CFAllocatorRef,
                                          codecType: CMVideoCodecType,
                                          width: int32_t,
                                          height: int32_t,
                                          extensions: CFDictionaryRef,
                                          outDesc: *mut CMVideoFormatDescriptionRef) -> OSStatus;
}