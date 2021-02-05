#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_use]
extern crate bitflags;
extern crate simple_error;

use std::{
    ffi::c_void,
    fmt,
    ops::{Deref, DerefMut},
};

use simple_error::SimpleError;

#[derive(Debug)]
pub struct Error {
    pub result: HRESULT,
}

impl Error {
    pub fn new() -> Self {
        Error {
            result: unsafe { decklink_get_e_fail() },
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "decklink error: {}", self.result)
    }
}

impl std::error::Error for Error {}

fn void_result(result: HRESULT) -> Result<(), Error> {
    match result {
        0 => Ok(()),
        result => Err(Error { result: result }),
    }
}

fn void_option_result(result: HRESULT) -> Result<Option<()>, Error> {
    match result {
        0 => Ok(Some(())),
        1 => Ok(None),
        result => Err(Error { result: result }),
    }
}

pub struct Device {
    implementation: *mut IDeckLink,
}

unsafe impl Send for Device {}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            unknown_release(self.implementation as *mut IUnknown);
        }
    }
}

impl REFIID {
    fn new(b: [u8; 16]) -> REFIID {
        REFIID {
            byte0: b[0],
            byte1: b[1],
            byte2: b[2],
            byte3: b[3],
            byte4: b[4],
            byte5: b[5],
            byte6: b[6],
            byte7: b[7],
            byte8: b[8],
            byte9: b[9],
            byte10: b[10],
            byte11: b[11],
            byte12: b[12],
            byte13: b[13],
            byte14: b[14],
            byte15: b[15],
        }
    }
}

impl Device {
    pub fn get_model_name(&self) -> Result<String, Error> {
        unsafe {
            let mut buf: *mut Buffer = std::ptr::null_mut();
            match decklink_get_model_name(self.implementation, &mut buf) {
                0 => {
                    let ret = std::ffi::CStr::from_ptr(buffer_data(buf) as *const i8)
                        .to_str()
                        .unwrap_or("")
                        .to_string();
                    buffer_release(buf);
                    Ok(ret)
                }
                result => Err(Error { result: result }),
            }
        }
    }

    fn query_interface<T>(&self, iid: REFIID) -> Result<*mut T, Error> {
        unsafe {
            let mut iface: *mut T = std::ptr::null_mut();
            match decklink_query_interface(
                self.implementation,
                iid,
                std::mem::transmute::<&mut *mut T, &mut *mut c_void>(&mut iface),
            ) {
                0 => Ok(iface),
                result => Err(Error { result: result }),
            }
        }
    }

    pub fn query_attributes(&self) -> Result<Attributes, Error> {
        match self.query_interface(REFIID::new([
            0xAB, 0xC1, 0x18, 0x43, 0xD9, 0x66, 0x44, 0xCB, 0x96, 0xE2, 0xA1, 0xCB, 0x5D, 0x31,
            0x35, 0xC4,
        ])) {
            Ok(iface) => Ok(Attributes {
                implementation: iface,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn query_status(&self) -> Result<Status, Error> {
        match self.query_interface(REFIID::new([
            0x5F, 0x55, 0x82, 0x00, 0x40, 0x28, 0x49, 0xBC, 0xBE, 0xAC, 0xDB, 0x3F, 0xA4, 0xA9,
            0x6E, 0x46,
        ])) {
            Ok(iface) => Ok(Status {
                implementation: iface,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn query_input(&self) -> Result<Input, Error> {
        match self.query_interface(REFIID::new([
            0xAF, 0x22, 0x76, 0x2B, 0xDF, 0xAC, 0x48, 0x46, 0xAA, 0x79, 0xFA, 0x88, 0x83, 0x56,
            0x09, 0x95,
        ])) {
            Ok(iface) => Ok(Input {
                implementation: iface,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn query_output(&self) -> Result<Output, Error> {
        match self.query_interface(REFIID::new([
            0xCC, 0x5C, 0x8A, 0x6E, 0x3F, 0x2F, 0x4B, 0x3A, 0x87, 0xEA, 0xFD, 0x78, 0xAF, 0x30,
            0x05, 0x64,
        ])) {
            Ok(iface) => Ok(Output {
                implementation: iface,
            }),
            Err(e) => Err(e),
        }
    }
}

pub struct Attributes {
    implementation: *mut IDeckLinkAttributes,
}

unsafe impl Send for Attributes {}

bitflags! {
    pub struct VideoIOSupport: u32 {
        const CAPTURE = _BMDVideoIOSupport_bmdDeviceSupportsCapture;
        const PLAYBACK = _BMDVideoIOSupport_bmdDeviceSupportsPlayback;
    }
}

impl Attributes {
    fn get_flag(&self, id: BMDDeckLinkAttributeID) -> Result<bool, Error> {
        unsafe {
            let mut v = false;
            match decklink_attributes_get_flag(self.implementation, id, &mut v) {
                0 => Ok(v),
                result => Err(Error { result: result }),
            }
        }
    }

    pub fn get_supports_internal_keying(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsInternalKeying)
    }
    pub fn get_supports_external_keying(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsExternalKeying)
    }
    pub fn get_supports_hd_keying(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsHDKeying)
    }
    pub fn get_supports_input_format_detection(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsInputFormatDetection)
    }
    pub fn get_has_reference_input(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkHasReferenceInput)
    }
    pub fn get_has_serial_port(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkHasSerialPort)
    }
    pub fn get_has_analog_video_output_gain(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkHasAnalogVideoOutputGain)
    }
    pub fn get_can_only_adjust_overall_video_output_gain(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkCanOnlyAdjustOverallVideoOutputGain)
    }
    pub fn get_has_video_input_antialiasing_filter(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkHasVideoInputAntiAliasingFilter)
    }
    pub fn get_has_bypass(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkHasBypass)
    }
    pub fn get_supports_clock_timing_adjustment(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsClockTimingAdjustment)
    }
    pub fn get_supports_full_duplex(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsFullDuplex)
    }
    pub fn get_supports_full_frame_reference_input_timing_offset(&self) -> Result<bool, Error> {
        self.get_flag(
            _BMDDeckLinkAttributeID_BMDDeckLinkSupportsFullFrameReferenceInputTimingOffset,
        )
    }
    pub fn get_supports_smpte_level_a_output(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsSMPTELevelAOutput)
    }
    pub fn get_supports_dual_link_sdi(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsDualLinkSDI)
    }
    pub fn get_supports_quad_link_sdi(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsQuadLinkSDI)
    }
    pub fn get_supports_idle_output(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsIdleOutput)
    }
    pub fn get_has_ltc_timecode_input(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkHasLTCTimecodeInput)
    }
    pub fn get_supports_duplex_mode_configuration(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsDuplexModeConfiguration)
    }
    pub fn get_supports_hdr_metadata(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsHDRMetadata)
    }
    pub fn get_supports_colorspace_metadata(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsColorspaceMetadata)
    }
    pub fn get_supports_hdmi_timecode(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsHDMITimecode)
    }
    pub fn get_supports_high_frame_rate_timecode(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsHighFrameRateTimecode)
    }
    pub fn get_supports_synchronize_to_capture_group(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsSynchronizeToCaptureGroup)
    }
    pub fn get_supports_synchronize_to_playback_group(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkAttributeID_BMDDeckLinkSupportsSynchronizeToPlaybackGroup)
    }

    fn get_int(&self, id: BMDDeckLinkAttributeID) -> Result<i64, Error> {
        unsafe {
            let mut v = 0i64;
            match decklink_attributes_get_int(self.implementation, id, &mut v) {
                0 => Ok(v),
                result => Err(Error { result: result }),
            }
        }
    }

    pub fn get_maximum_audio_channels(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkMaximumAudioChannels)
    }
    pub fn get_maximum_analog_audio_input_channels(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkMaximumAnalogAudioInputChannels)
    }
    pub fn get_maximum_analog_audio_output_channels(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkMaximumAnalogAudioOutputChannels)
    }
    pub fn get_number_of_subdevices(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkNumberOfSubDevices)
    }
    pub fn get_subdevice_index(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkSubDeviceIndex)
    }
    pub fn get_persistent_id(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkPersistentID)
    }
    pub fn get_device_group_id(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkDeviceGroupID)
    }
    pub fn get_topological_id(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkTopologicalID)
    }
    pub fn get_video_output_connections(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkVideoOutputConnections)
    }
    pub fn get_video_input_connections(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkVideoInputConnections)
    }
    pub fn get_audio_output_connections(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkAudioOutputConnections)
    }
    pub fn get_audio_input_connections(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkAudioInputConnections)
    }
    pub fn get_video_io_support(&self) -> Result<VideoIOSupport, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkVideoIOSupport)
            .map(|v| VideoIOSupport::from_bits_truncate(v as u32))
    }
    pub fn get_deck_control_connections(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkDeckControlConnections)
    }
    pub fn get_device_interface(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkDeviceInterface)
    }
    pub fn get_audio_input_rca_channel_count(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkAudioInputRCAChannelCount)
    }
    pub fn get_audio_input_xlr_channel_count(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkAudioInputXLRChannelCount)
    }
    pub fn get_audio_output_rca_channel_count(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkAudioOutputRCAChannelCount)
    }
    pub fn get_audio_output_xlr_channel_count(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkAudioOutputXLRChannelCount)
    }
    pub fn get_paired_device_persistent_id(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkAttributeID_BMDDeckLinkPairedDevicePersistentID)
    }

    fn get_float(&self, id: BMDDeckLinkAttributeID) -> Result<f64, Error> {
        unsafe {
            let mut v = 0f64;
            match decklink_attributes_get_float(self.implementation, id, &mut v) {
                0 => Ok(v),
                result => Err(Error { result: result }),
            }
        }
    }

    pub fn get_video_input_gain_minimum(&self) -> Result<f64, Error> {
        self.get_float(_BMDDeckLinkAttributeID_BMDDeckLinkVideoInputGainMinimum)
    }
    pub fn get_video_input_gain_maximum(&self) -> Result<f64, Error> {
        self.get_float(_BMDDeckLinkAttributeID_BMDDeckLinkVideoInputGainMaximum)
    }
    pub fn get_video_output_gain_minimum(&self) -> Result<f64, Error> {
        self.get_float(_BMDDeckLinkAttributeID_BMDDeckLinkVideoOutputGainMinimum)
    }
    pub fn get_video_output_gain_maximum(&self) -> Result<f64, Error> {
        self.get_float(_BMDDeckLinkAttributeID_BMDDeckLinkVideoOutputGainMaximum)
    }
    pub fn get_microphone_input_gain_minimum(&self) -> Result<f64, Error> {
        self.get_float(_BMDDeckLinkAttributeID_BMDDeckLinkMicrophoneInputGainMinimum)
    }
    pub fn get_microphone_input_gain_maximum(&self) -> Result<f64, Error> {
        self.get_float(_BMDDeckLinkAttributeID_BMDDeckLinkMicrophoneInputGainMaximum)
    }

    fn get_string(&self, id: BMDDeckLinkAttributeID) -> Result<String, Error> {
        unsafe {
            let mut v: *mut Buffer = std::ptr::null_mut();
            match decklink_attributes_get_string(self.implementation, id, &mut v) {
                0 => {
                    let ret = Ok(std::ffi::CStr::from_ptr(buffer_data(v) as *const i8)
                        .to_str()
                        .unwrap_or("")
                        .to_string());
                    buffer_release(v);
                    ret
                }
                result => Err(Error { result: result }),
            }
        }
    }

    pub fn get_serial_port_device_name(&self) -> Result<String, Error> {
        self.get_string(_BMDDeckLinkAttributeID_BMDDeckLinkSerialPortDeviceName)
    }
    pub fn get_vendor_name(&self) -> Result<String, Error> {
        self.get_string(_BMDDeckLinkAttributeID_BMDDeckLinkVendorName)
    }
    pub fn get_display_name(&self) -> Result<String, Error> {
        self.get_string(_BMDDeckLinkAttributeID_BMDDeckLinkDisplayName)
    }
    pub fn get_model_name(&self) -> Result<String, Error> {
        self.get_string(_BMDDeckLinkAttributeID_BMDDeckLinkModelName)
    }
    pub fn get_device_handle(&self) -> Result<String, Error> {
        self.get_string(_BMDDeckLinkAttributeID_BMDDeckLinkDeviceHandle)
    }
}

impl Drop for Attributes {
    fn drop(&mut self) {
        unsafe {
            unknown_release(self.implementation as *mut IUnknown);
        }
    }
}

bitflags! {
    pub struct DeviceBusyState: u32 {
        const CAPTURE_BUSY = _BMDDeviceBusyState_bmdDeviceCaptureBusy;
        const PLAYBACK_BUSY = _BMDDeviceBusyState_bmdDevicePlaybackBusy;
        const SERIAL_PORT_BUSY = _BMDDeviceBusyState_bmdDeviceSerialPortBusy;
    }
}

bitflags! {
    pub struct DisplayModeFlags: u32 {
        const SUPPORTS_3D = _BMDDisplayModeFlags_bmdDisplayModeSupports3D;
        const COLORSPACE_REC601 = _BMDDisplayModeFlags_bmdDisplayModeColorspaceRec601;
        const COLORSPACE_REC709 = _BMDDisplayModeFlags_bmdDisplayModeColorspaceRec709;
        const COLORSPACE_REC2020 = _BMDDisplayModeFlags_bmdDisplayModeColorspaceRec2020;
    }
}

bitflags! {
    pub struct VideoInputFlags: u32 {
        const DEFAULT = _BMDVideoInputFlags_bmdVideoInputFlagDefault;
        const ENABLE_FORMAT_DETECTION = _BMDVideoInputFlags_bmdVideoInputEnableFormatDetection;
        const DUAL_STREAM_3D = _BMDVideoInputFlags_bmdVideoInputDualStream3D;
    }
}

bitflags! {
    pub struct VideoOutputFlags: u32 {
        const DEFAULT = _BMDVideoOutputFlags_bmdVideoOutputFlagDefault;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelFormat(pub u32);

impl PixelFormat {
    pub const FORMAT_8BIT_YUV: PixelFormat = PixelFormat(_BMDPixelFormat_bmdFormat8BitYUV);
    pub const FORMAT_10BIT_YUV: PixelFormat = PixelFormat(_BMDPixelFormat_bmdFormat10BitYUV);
    pub const FORMAT_8BIT_ARGB: PixelFormat = PixelFormat(_BMDPixelFormat_bmdFormat8BitARGB);
    pub const FORMAT_8BIT_BGRA: PixelFormat = PixelFormat(_BMDPixelFormat_bmdFormat8BitBGRA);
    pub const FORMAT_10BIT_RGB: PixelFormat = PixelFormat(_BMDPixelFormat_bmdFormat10BitRGB);
    pub const FORMAT_12BIT_RGB: PixelFormat = PixelFormat(_BMDPixelFormat_bmdFormat12BitRGB);
    pub const FORMAT_12BIT_RGBLE: PixelFormat = PixelFormat(_BMDPixelFormat_bmdFormat12BitRGBLE);
    pub const FORMAT_10BIT_RGBXLE: PixelFormat = PixelFormat(_BMDPixelFormat_bmdFormat10BitRGBXLE);
    pub const FORMAT_10BIT_RGBX: PixelFormat = PixelFormat(_BMDPixelFormat_bmdFormat10BitRGBX);
    pub const FORMAT_H265: PixelFormat = PixelFormat(_BMDPixelFormat_bmdFormatH265);
    pub const FORMAT_DNXHR: PixelFormat = PixelFormat(_BMDPixelFormat_bmdFormatDNxHR);
    pub const FORMAT_12BIT_RAW_GRBG: PixelFormat =
        PixelFormat(_BMDPixelFormat_bmdFormat12BitRAWGRBG);
    pub const FORMAT_12BIT_RAW_JPEG: PixelFormat =
        PixelFormat(_BMDPixelFormat_bmdFormat12BitRAWJPEG);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutputFrameCompletionResult(pub u32);

impl OutputFrameCompletionResult {
    pub const COMPLETED: OutputFrameCompletionResult =
        OutputFrameCompletionResult(_BMDOutputFrameCompletionResult_bmdOutputFrameCompleted);
    pub const DISPLAYED_LATE: OutputFrameCompletionResult =
        OutputFrameCompletionResult(_BMDOutputFrameCompletionResult_bmdOutputFrameDisplayedLate);
    pub const DROPPED: OutputFrameCompletionResult =
        OutputFrameCompletionResult(_BMDOutputFrameCompletionResult_bmdOutputFrameDropped);
    pub const FLUSHED: OutputFrameCompletionResult =
        OutputFrameCompletionResult(_BMDOutputFrameCompletionResult_bmdOutputFrameFlushed);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisplayMode(pub u32);

impl DisplayMode {
    pub const MODE_NTSC: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeNTSC);
    pub const MODE_NTSC2398: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeNTSC2398);
    pub const MODE_PAL: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModePAL);
    pub const MODE_NTSCP: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeNTSCp);
    pub const MODE_PALP: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModePALp);
    pub const MODE_HD1080P2398: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeHD1080p2398);
    pub const MODE_HD1080P24: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeHD1080p24);
    pub const MODE_HD1080P25: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeHD1080p25);
    pub const MODE_HD1080P2997: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeHD1080p2997);
    pub const MODE_HD1080P30: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeHD1080p30);
    pub const MODE_HD1080P50: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeHD1080p50);
    pub const MODE_HD1080P5994: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeHD1080p5994);
    pub const MODE_HD1080P6000: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeHD1080p6000);
    pub const MODE_HD1080I50: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeHD1080i50);
    pub const MODE_HD1080I5994: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeHD1080i5994);
    pub const MODE_HD1080I6000: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeHD1080i6000);
    pub const MODE_HD720P50: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeHD720p50);
    pub const MODE_HD720P5994: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeHD720p5994);
    pub const MODE_HD720P60: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeHD720p60);
    pub const MODE_2K2398: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode2k2398);
    pub const MODE_2K24: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode2k24);
    pub const MODE_2K25: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode2k25);
    pub const MODE_2KDCI2398: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode2kDCI2398);
    pub const MODE_2KDCI24: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode2kDCI24);
    pub const MODE_2KDCI25: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode2kDCI25);
    pub const MODE_2KDCI2997: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode2kDCI2997);
    pub const MODE_2KDCI30: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode2kDCI30);
    pub const MODE_2KDCI50: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode2kDCI50);
    pub const MODE_2KDCI5994: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode2kDCI5994);
    pub const MODE_2KDCI60: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode2kDCI60);
    pub const MODE_4K2160P2398: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode4K2160p2398);
    pub const MODE_4K2160P24: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode4K2160p24);
    pub const MODE_4K2160P25: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode4K2160p25);
    pub const MODE_4K2160P2997: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode4K2160p2997);
    pub const MODE_4K2160P30: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode4K2160p30);
    pub const MODE_4K2160P50: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode4K2160p50);
    pub const MODE_4K2160P5994: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode4K2160p5994);
    pub const MODE_4K2160P60: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode4K2160p60);
    pub const MODE_4KDCI2398: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode4kDCI2398);
    pub const MODE_4KDCI24: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode4kDCI24);
    pub const MODE_4KDCI25: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode4kDCI25);
    pub const MODE_4KDCI2997: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode4kDCI2997);
    pub const MODE_4KDCI30: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode4kDCI30);
    pub const MODE_4KDCI50: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode4kDCI50);
    pub const MODE_4KDCI5994: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode4kDCI5994);
    pub const MODE_4KDCI60: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode4kDCI60);
    pub const MODE_8K4320P2398: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode8K4320p2398);
    pub const MODE_8K4320P24: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode8K4320p24);
    pub const MODE_8K4320P25: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode8K4320p25);
    pub const MODE_8K4320P2997: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode8K4320p2997);
    pub const MODE_8K4320P30: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode8K4320p30);
    pub const MODE_8K4320P50: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode8K4320p50);
    pub const MODE_8K4320P5994: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode8K4320p5994);
    pub const MODE_8K4320P60: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode8K4320p60);
    pub const MODE_8KDCI2398: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode8kDCI2398);
    pub const MODE_8KDCI24: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode8kDCI24);
    pub const MODE_8KDCI25: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode8kDCI25);
    pub const MODE_8KDCI2997: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode8kDCI2997);
    pub const MODE_8KDCI30: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode8kDCI30);
    pub const MODE_8KDCI50: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode8kDCI50);
    pub const MODE_8KDCI5994: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode8kDCI5994);
    pub const MODE_8KDCI60: DisplayMode = DisplayMode(_BMDDisplayMode_bmdMode8kDCI60);
    pub const MODE_CINTEL_RAW: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeCintelRAW);
    pub const MODE_CINTEL_COMPRESSED_RAW: DisplayMode =
        DisplayMode(_BMDDisplayMode_bmdModeCintelCompressedRAW);
    pub const MODE_UNKNOWN: DisplayMode = DisplayMode(_BMDDisplayMode_bmdModeUnknown);
}

pub struct DisplayModeInfo {
    implementation: *mut IDeckLinkDisplayMode,
}

unsafe impl Send for DisplayModeInfo {}

impl Drop for DisplayModeInfo {
    fn drop(&mut self) {
        unsafe {
            unknown_release(self.implementation as *mut IUnknown);
        }
    }
}

impl DisplayModeInfo {
    pub fn get_display_mode(&self) -> DisplayMode {
        unsafe { DisplayMode(decklink_display_mode_get_display_mode(self.implementation) as u32) }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let mut buf: *mut Buffer = std::ptr::null_mut();
            match decklink_display_mode_get_name(self.implementation, &mut buf) {
                0 => {
                    let ret = std::ffi::CStr::from_ptr(buffer_data(buf) as *const i8)
                        .to_str()
                        .unwrap_or("")
                        .to_string();
                    buffer_release(buf);
                    Ok(ret)
                }
                result => Err(Error { result: result }),
            }
        }
    }

    pub fn get_width(&mut self) -> i32 {
        unsafe { decklink_display_mode_get_width(self.implementation) as _ }
    }

    pub fn get_height(&mut self) -> i32 {
        unsafe { decklink_display_mode_get_height(self.implementation) as _ }
    }

    pub fn get_frame_rate(&mut self) -> Result<(i64, i64), Error> {
        unsafe {
            let mut frame_duration = 0;
            let mut time_scale = 0;
            void_result(decklink_display_mode_get_frame_rate(
                self.implementation,
                &mut frame_duration,
                &mut time_scale,
            ))?;
            Ok((frame_duration as _, time_scale as _))
        }
    }
}

pub struct DisplayModeIterator {
    implementation: *mut IDeckLinkDisplayModeIterator,
}

unsafe impl Send for DisplayModeIterator {}

impl Drop for DisplayModeIterator {
    fn drop(&mut self) {
        unsafe {
            unknown_release(self.implementation as *mut IUnknown);
        }
    }
}

impl std::iter::Iterator for DisplayModeIterator {
    type Item = DisplayModeInfo;

    fn next(&mut self) -> Option<DisplayModeInfo> {
        unsafe {
            let mut mode: *mut IDeckLinkDisplayMode = std::ptr::null_mut();
            if decklink_display_mode_iterator_next(self.implementation, &mut mode) != 0
                || mode.is_null()
            {
                return None;
            }
            return Some(DisplayModeInfo {
                implementation: mode,
            });
        }
    }
}

pub struct Status {
    implementation: *mut IDeckLinkStatus,
}

unsafe impl Send for Status {}

impl Status {
    fn get_flag(&self, id: BMDDeckLinkStatusID) -> Result<bool, Error> {
        unsafe {
            let mut v = false;
            match decklink_status_get_flag(self.implementation, id, &mut v) {
                0 => Ok(v),
                result => Err(Error { result: result }),
            }
        }
    }

    pub fn get_video_input_signal_locked(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkStatusID_bmdDeckLinkStatusVideoInputSignalLocked)
    }
    pub fn get_reference_signal_locked(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkStatusID_bmdDeckLinkStatusReferenceSignalLocked)
    }
    pub fn get_received_edid(&self) -> Result<bool, Error> {
        self.get_flag(_BMDDeckLinkStatusID_bmdDeckLinkStatusReceivedEDID)
    }

    fn get_int(&self, id: BMDDeckLinkStatusID) -> Result<i64, Error> {
        unsafe {
            let mut v = 0i64;
            match decklink_status_get_int(self.implementation, id, &mut v) {
                0 => Ok(v),
                result => Err(Error { result: result }),
            }
        }
    }

    pub fn get_detected_video_input_mode(&self) -> Result<DisplayMode, Error> {
        self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusDetectedVideoInputMode)
            .map(|v| DisplayMode(v as u32))
    }
    pub fn get_detected_video_input_flags(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusDetectedVideoInputFlags)
    }
    pub fn get_current_video_input_mode(&self) -> Result<DisplayMode, Error> {
        self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusCurrentVideoInputMode)
            .map(|v| DisplayMode(v as u32))
    }
    pub fn get_current_video_input_pixel_format(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusCurrentVideoInputPixelFormat)
    }
    pub fn get_current_video_input_flags(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusCurrentVideoInputFlags)
    }
    pub fn get_current_video_output_mode(&self) -> Result<DisplayMode, Error> {
        self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusCurrentVideoOutputMode)
            .map(|v| DisplayMode(v as u32))
    }
    pub fn get_current_video_output_flags(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusCurrentVideoOutputFlags)
    }
    pub fn get_pci_express_link_width(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusPCIExpressLinkWidth)
    }
    pub fn get_pci_express_link_speed(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusPCIExpressLinkSpeed)
    }
    pub fn get_last_video_output_pixel_format(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusLastVideoOutputPixelFormat)
    }
    pub fn get_reference_signal_mode(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusReferenceSignalMode)
    }
    pub fn get_reference_signal_flags(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusReferenceSignalFlags)
    }
    pub fn get_duplex_mode(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusDuplexMode)
    }
    pub fn get_busy(&self) -> Result<DeviceBusyState, Error> {
        self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusBusy)
            .map(|v| DeviceBusyState::from_bits_truncate(v as u32))
    }
    pub fn get_interchangeable_panel_type(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusInterchangeablePanelType)
    }
    pub fn get_device_temperature(&self) -> Result<i64, Error> {
        self.get_int(_BMDDeckLinkStatusID_bmdDeckLinkStatusDeviceTemperature)
    }
}

impl Drop for Status {
    fn drop(&mut self) {
        unsafe {
            unknown_release(self.implementation as *mut IUnknown);
        }
    }
}

pub struct Iterator {
    implementation: *mut IDeckLinkIterator,
}

unsafe impl Send for Iterator {}

impl Drop for Iterator {
    fn drop(&mut self) {
        unsafe {
            unknown_release(self.implementation as *mut IUnknown);
        }
    }
}

impl std::iter::Iterator for Iterator {
    type Item = Device;

    fn next(&mut self) -> Option<Device> {
        unsafe {
            let mut device: *mut IDeckLink = std::ptr::null_mut();
            if decklink_iterator_next(self.implementation, &mut device) != 0 || device.is_null() {
                return None;
            }
            return Some(Device {
                implementation: device,
            });
        }
    }
}

impl Iterator {
    pub fn new() -> Result<Iterator, SimpleError> {
        unsafe {
            let iterator = create_decklink_iterator_instance();
            if iterator.is_null() {
                return Err(SimpleError::new("unable to create decklink iterator. the latest decklink drivers may need to be installed"));
            }
            return Ok(Iterator {
                implementation: iterator,
            });
        }
    }
}

bitflags! {
    pub struct VideoInputFormatChangedEvents: u32 {
        const DISPLAY_MODE_CHANGED = _BMDVideoInputFormatChangedEvents_bmdVideoInputDisplayModeChanged;
        const FIELD_DOMINANCE_CHANGED = _BMDVideoInputFormatChangedEvents_bmdVideoInputFieldDominanceChanged;
        const COLORSPACE_CHANGED = _BMDVideoInputFormatChangedEvents_bmdVideoInputColorspaceChanged;
    }
}

bitflags! {
    pub struct DetectedVideoInputFormatFlags: u32 {
        const YCBCR422 = _BMDDetectedVideoInputFormatFlags_bmdDetectedVideoInputYCbCr422;
        const RGB444 = _BMDDetectedVideoInputFormatFlags_bmdDetectedVideoInputRGB444;
        const DUAL_STREAM_3D = _BMDDetectedVideoInputFormatFlags_bmdDetectedVideoInputDualStream3D;
    }
}

pub trait InputCallback {
    fn video_input_format_changed(
        &mut self,
        _notification_events: VideoInputFormatChangedEvents,
        _new_display_mode: DisplayModeInfo,
        _detected_signal_flags: DetectedVideoInputFormatFlags,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn video_input_frame_arrived(
        &mut self,
        _video_frame: Option<VideoInputFrame>,
        _audio_packet: Option<AudioInputPacket>,
    ) -> Result<(), Error> {
        Ok(())
    }
}

#[no_mangle]
unsafe extern "C" fn input_callback_video_input_format_changed(
    implementation: *mut Box<dyn InputCallback>,
    notification_events: u32,
    new_display_mode: *mut IDeckLinkDisplayMode,
    detected_signal_flags: u32,
) -> HRESULT {
    let implementation = &mut *implementation;
    match implementation.video_input_format_changed(
        VideoInputFormatChangedEvents::from_bits_truncate(notification_events),
        {
            unknown_add_ref(new_display_mode as *mut IUnknown);
            DisplayModeInfo {
                implementation: new_display_mode,
            }
        },
        DetectedVideoInputFormatFlags::from_bits_truncate(detected_signal_flags),
    ) {
        Ok(_) => 0,
        Err(e) => e.result,
    }
}

#[no_mangle]
unsafe extern "C" fn input_callback_video_input_frame_arrived(
    implementation: *mut Box<dyn InputCallback>,
    video_frame: *mut IDeckLinkVideoInputFrame,
    audio_packet: *mut IDeckLinkAudioInputPacket,
) -> HRESULT {
    let implementation = &mut *implementation;
    match implementation.video_input_frame_arrived(
        match video_frame.is_null() {
            true => None,
            false => {
                unknown_add_ref(video_frame as *mut IUnknown);
                Some(VideoInputFrame {
                    implementation: video_frame,
                })
            }
        },
        match audio_packet.is_null() {
            true => None,
            false => {
                unknown_add_ref(audio_packet as *mut IUnknown);
                Some(AudioInputPacket {
                    implementation: audio_packet,
                })
            }
        },
    ) {
        Ok(_) => 0,
        Err(e) => e.result,
    }
}

pub struct Input {
    implementation: *mut IDeckLinkInput,
}

unsafe impl Send for Input {}

impl Drop for Input {
    fn drop(&mut self) {
        unsafe {
            unknown_release(self.implementation as *mut IUnknown);
        }
    }
}

pub struct InputWithCallback<'a> {
    inner: Option<Input>,
    _callback: Box<Box<dyn InputCallback + Send + 'a>>,
}

impl<'a> InputWithCallback<'a> {
    pub fn into_inner(mut self) -> Input {
        unsafe {
            self.inner
                .as_mut()
                .unwrap()
                .set_callback(None)
                .expect("set_callback should always succeed");
        }
        self.inner.take().unwrap()
    }
}

impl<'a> Deref for InputWithCallback<'a> {
    type Target = Input;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<'a> DerefMut for InputWithCallback<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}

impl<'a> Drop for InputWithCallback<'a> {
    fn drop(&mut self) {
        unsafe {
            if let Some(inner) = &mut self.inner {
                inner
                    .set_callback(None)
                    .expect("set_callback should always succeed");
            }
        }
    }
}

impl Input {
    pub fn get_display_mode_iterator(&mut self) -> Result<DisplayModeIterator, Error> {
        unsafe {
            let mut iterator: *mut IDeckLinkDisplayModeIterator = std::ptr::null_mut();
            match decklink_input_get_display_mode_iterator(self.implementation, &mut iterator) {
                0 => Ok(DisplayModeIterator {
                    implementation: iterator,
                }),
                result => Err(Error { result: result }),
            }
        }
    }

    pub fn with_callback<'a, T>(mut self, callback: T) -> InputWithCallback<'a>
    where
        T: InputCallback + Send + 'a,
    {
        let mut callback: Box<Box<dyn InputCallback + Send + 'a>> = Box::new(Box::new(callback));
        unsafe {
            self.set_callback(Some(&mut *callback))
                .expect("set_callback should always succeed");
        }
        InputWithCallback {
            inner: Some(self),
            _callback: callback,
        }
    }

    /// The caller must ensure that the given callback lives until the callback is unset. Use with_callback for a safer alternative.
    pub unsafe fn set_callback<'a>(
        &mut self,
        callback: Option<&mut Box<dyn InputCallback + Send + 'a>>,
    ) -> Result<(), Error> {
        match callback {
            Some(callback) => {
                let callback = create_decklink_input_callback(
                    callback as *mut Box<dyn InputCallback + Send + 'a> as *mut c_void,
                );
                let result =
                    void_result(decklink_input_set_callback(self.implementation, callback));
                unknown_release(callback as *mut IUnknown);
                result
            }
            None => void_result(decklink_input_set_callback(
                self.implementation,
                std::ptr::null_mut(),
            )),
        }
    }

    pub fn start_streams(&mut self) -> Result<(), Error> {
        unsafe { void_result(decklink_input_start_streams(self.implementation)) }
    }

    pub fn stop_streams(&mut self) -> Result<(), Error> {
        unsafe { void_result(decklink_input_stop_streams(self.implementation)) }
    }

    pub fn pause_streams(&mut self) -> Result<(), Error> {
        unsafe { void_result(decklink_input_pause_streams(self.implementation)) }
    }

    pub fn flush_streams(&mut self) -> Result<(), Error> {
        unsafe { void_result(decklink_input_flush_streams(self.implementation)) }
    }

    pub fn enable_video_input(
        &mut self,
        display_mode: DisplayMode,
        pixel_format: PixelFormat,
        flags: VideoInputFlags,
    ) -> Result<(), Error> {
        unsafe {
            void_result(decklink_input_enable_video_input(
                self.implementation,
                display_mode.0,
                pixel_format.0,
                flags.bits(),
            ))
        }
    }

    pub fn disable_video_input(&mut self) -> Result<(), Error> {
        unsafe { void_result(decklink_input_disable_video_input(self.implementation)) }
    }

    pub fn disable_audio_input(&mut self) -> Result<(), Error> {
        unsafe { void_result(decklink_input_disable_audio_input(self.implementation)) }
    }
}

pub struct Output {
    implementation: *mut IDeckLinkOutput,
}

unsafe impl Send for Output {}

impl Drop for Output {
    fn drop(&mut self) {
        unsafe {
            unknown_release(self.implementation as *mut IUnknown);
        }
    }
}

pub trait VideoOutputCallback {
    // TODO: add the result argument
    fn scheduled_frame_completed(
        &mut self,
        _completed_frame: MutableVideoFrame,
        _result: OutputFrameCompletionResult,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn scheduled_playback_has_stopped(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

pub struct OutputWithCallback<'a> {
    inner: Option<Output>,
    _callback: Box<Box<dyn VideoOutputCallback + Send + 'a>>,
}

impl<'a> OutputWithCallback<'a> {
    pub fn into_inner(mut self) -> Output {
        unsafe {
            self.inner
                .as_mut()
                .unwrap()
                .set_scheduled_frame_completion_callback(None)
                .expect("set_callback should always succeed");
        }
        self.inner.take().unwrap()
    }
}

impl<'a> Deref for OutputWithCallback<'a> {
    type Target = Output;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<'a> DerefMut for OutputWithCallback<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}

impl<'a> Drop for OutputWithCallback<'a> {
    fn drop(&mut self) {
        unsafe {
            if let Some(inner) = &mut self.inner {
                inner
                    .set_scheduled_frame_completion_callback(None)
                    .expect("set_callback should always succeed");
            }
        }
    }
}

#[no_mangle]
unsafe extern "C" fn video_output_callback_scheduled_frame_completed(
    implementation: *mut Box<dyn VideoOutputCallback>,
    completed_frame: *mut IDeckLinkVideoFrame,
    result: BMDOutputFrameCompletionResult,
) -> HRESULT {
    let implementation = &mut *implementation;
    let completed_frame = {
        unknown_add_ref(completed_frame as *mut IUnknown);
        MutableVideoFrame {
            implementation: completed_frame as _,
        }
    };
    match implementation
        .scheduled_frame_completed(completed_frame, OutputFrameCompletionResult(result))
    {
        Ok(_) => 0,
        Err(e) => e.result,
    }
}

#[no_mangle]
unsafe extern "C" fn video_output_callback_scheduled_playback_has_stopped(
    implementation: *mut Box<dyn VideoOutputCallback>,
) -> HRESULT {
    let implementation = &mut *implementation;
    match implementation.scheduled_playback_has_stopped() {
        Ok(_) => 0,
        Err(e) => e.result,
    }
}

impl Output {
    pub fn get_display_mode_iterator(&mut self) -> Result<DisplayModeIterator, Error> {
        unsafe {
            let mut iterator: *mut IDeckLinkDisplayModeIterator = std::ptr::null_mut();
            match decklink_output_get_display_mode_iterator(self.implementation, &mut iterator) {
                0 => Ok(DisplayModeIterator {
                    implementation: iterator,
                }),
                result => Err(Error { result: result }),
            }
        }
    }

    pub fn create_video_frame(
        &mut self,
        width: i32,
        height: i32,
        row_bytes: i32,
        pixel_format: PixelFormat,
        flags: FrameFlags,
    ) -> Result<MutableVideoFrame, Error> {
        unsafe {
            let mut frame: *mut IDeckLinkMutableVideoFrame = std::ptr::null_mut();
            match decklink_output_create_video_frame(
                self.implementation,
                width,
                height,
                row_bytes,
                pixel_format.0,
                flags.bits(),
                &mut frame,
            ) {
                0 => Ok(MutableVideoFrame {
                    implementation: frame,
                }),
                result => Err(Error { result: result }),
            }
        }
    }

    pub fn enable_video_output(
        &mut self,
        display_mode: DisplayMode,
        flags: VideoOutputFlags,
    ) -> Result<(), Error> {
        unsafe {
            void_result(decklink_output_enable_video_output(
                self.implementation,
                display_mode.0,
                flags.bits(),
            ))
        }
    }

    pub fn disable_video_output(&mut self) -> Result<(), Error> {
        unsafe { void_result(decklink_output_disable_video_output(self.implementation)) }
    }

    /// The caller must ensure that the given callback lives until the callback is unset. Use with_callback for a safer alternative.
    pub unsafe fn set_scheduled_frame_completion_callback<'a>(
        &mut self,
        callback: Option<&mut Box<dyn VideoOutputCallback + Send + 'a>>,
    ) -> Result<(), Error> {
        match callback {
            Some(callback) => {
                let callback = create_decklink_video_output_callback(
                    callback as *mut Box<dyn VideoOutputCallback + Send + 'a> as *mut c_void,
                );
                let result = void_result(decklink_output_set_scheduled_frame_completion_callback(
                    self.implementation,
                    callback,
                ));
                unknown_release(callback as *mut IUnknown);
                result
            }
            None => void_result(decklink_output_set_scheduled_frame_completion_callback(
                self.implementation,
                std::ptr::null_mut(),
            )),
        }
    }

    pub fn with_callback<'a, T>(mut self, callback: T) -> OutputWithCallback<'a>
    where
        T: VideoOutputCallback + Send + 'a,
    {
        let mut callback: Box<Box<dyn VideoOutputCallback + Send + 'a>> =
            Box::new(Box::new(callback));
        unsafe {
            self.set_scheduled_frame_completion_callback(Some(&mut *callback))
                .expect("set_scheduled_frame_completion_callback should always succeed");
        }
        OutputWithCallback {
            inner: Some(self),
            _callback: callback,
        }
    }

    pub fn start_scheduled_playback(
        &mut self,
        playback_start_time: i64,
        time_scale: i64,
        playback_speed: f64,
    ) -> Result<(), Error> {
        unsafe {
            void_result(decklink_output_start_scheduled_playback(
                self.implementation,
                playback_start_time,
                time_scale,
                playback_speed,
            ))
        }
    }

    // TODO: support other types of frames?
    pub fn schedule_video_frame(
        &mut self,
        frame: MutableVideoFrame,
        display_time: i64,
        display_duration: i64,
        time_scale: i64,
    ) -> Result<(), Error> {
        unsafe {
            void_result(decklink_output_schedule_video_frame(
                self.implementation,
                frame.implementation as _,
                display_time as _,
                display_duration as _,
                time_scale as _,
            ))
        }
    }
}

bitflags! {
    pub struct FrameFlags: u32 {
        const DEFAULT = _BMDFrameFlags_bmdFrameFlagDefault as u32;
        const FLIP_VERTICAL = _BMDFrameFlags_bmdFrameFlagFlipVertical as u32;
        const CONTAINS_HDR_METADATA = _BMDFrameFlags_bmdFrameContainsHDRMetadata as u32;
        const CONTAINS_CINTEL_METADATA = _BMDFrameFlags_bmdFrameContainsCintelMetadata as u32;
        const CAPTURED_AS_PS_F = _BMDFrameFlags_bmdFrameCapturedAsPsF as u32;
        const HAS_NO_INPUT_SOURCE = _BMDFrameFlags_bmdFrameHasNoInputSource as u32;
    }
}

pub struct VideoInputFrame {
    implementation: *mut IDeckLinkVideoInputFrame,
}

unsafe impl Send for VideoInputFrame {}

impl Drop for VideoInputFrame {
    fn drop(&mut self) {
        unsafe {
            unknown_release(self.implementation as *mut IUnknown);
        }
    }
}

impl VideoFrame for VideoInputFrame {
    unsafe fn implementation(&mut self) -> *mut IDeckLinkVideoFrame {
        self.implementation as _
    }
}

impl VideoFrame for &mut VideoInputFrame {
    unsafe fn implementation(&mut self) -> *mut IDeckLinkVideoFrame {
        self.implementation as _
    }
}

pub struct AudioInputPacket {
    implementation: *mut IDeckLinkAudioInputPacket,
}

unsafe impl Send for AudioInputPacket {}

impl Drop for AudioInputPacket {
    fn drop(&mut self) {
        unsafe {
            unknown_release(self.implementation as *mut IUnknown);
        }
    }
}

pub struct VideoConversion {
    implementation: *mut IDeckLinkVideoConversion,
}

unsafe impl Send for VideoConversion {}

impl Drop for VideoConversion {
    fn drop(&mut self) {
        unsafe {
            unknown_release(self.implementation as *mut IUnknown);
        }
    }
}

impl VideoConversion {
    pub fn new() -> Result<VideoConversion, SimpleError> {
        unsafe {
            let conversion = create_decklink_video_conversion_instance();
            if conversion.is_null() {
                return Err(SimpleError::new("unable to create decklink video conversion. the latest decklink drivers may need to be installed"));
            }
            return Ok(VideoConversion {
                implementation: conversion,
            });
        }
    }

    pub fn convert_frame<S: VideoFrame, D: VideoFrame>(
        &mut self,
        mut src_frame: S,
        dst_frame: &mut D,
    ) -> Result<(), Error> {
        unsafe {
            void_result(decklink_video_conversion_convert_frame(
                self.implementation,
                src_frame.implementation(),
                dst_frame.implementation(),
            ))
        }
    }
}

pub trait VideoFrame {
    unsafe fn implementation(&mut self) -> *mut IDeckLinkVideoFrame;

    fn get_width(&mut self) -> i32 {
        unsafe { decklink_video_frame_get_width(self.implementation()) as _ }
    }

    fn get_height(&mut self) -> i32 {
        unsafe { decklink_video_frame_get_height(self.implementation()) as _ }
    }

    fn get_row_bytes(&mut self) -> i32 {
        unsafe { decklink_video_frame_get_row_bytes(self.implementation()) as _ }
    }

    fn get_pixel_format(&mut self) -> PixelFormat {
        unsafe { PixelFormat(decklink_video_frame_get_pixel_format(self.implementation())) }
    }

    fn get_flags(&mut self) -> FrameFlags {
        unsafe {
            FrameFlags::from_bits_truncate(decklink_video_frame_get_flags(self.implementation()))
        }
    }

    fn get_bytes(&mut self) -> Result<&[u8], Error> {
        unsafe {
            let mut buf: *mut c_void = std::ptr::null_mut();
            void_result(decklink_video_frame_get_bytes(
                self.implementation(),
                &mut buf,
            ))?;
            Ok(std::slice::from_raw_parts(
                buf as *mut u8,
                (self.get_row_bytes() * self.get_height()) as usize,
            ))
        }
    }

    fn get_timecode(&mut self, format: TimecodeFormat) -> Result<Option<Timecode>, Error> {
        unsafe {
            let mut timecode: *mut IDeckLinkTimecode = std::ptr::null_mut();
            Ok(void_option_result(decklink_video_frame_get_timecode(
                self.implementation(),
                format.0,
                &mut timecode,
            ))?
            .map(|_| Timecode {
                implementation: timecode,
            }))
        }
    }
}

pub struct MutableVideoFrame {
    implementation: *mut IDeckLinkMutableVideoFrame,
}

unsafe impl Send for MutableVideoFrame {}

impl Drop for MutableVideoFrame {
    fn drop(&mut self) {
        unsafe {
            unknown_release(self.implementation as *mut IUnknown);
        }
    }
}

impl MutableVideoFrame {
    pub fn get_bytes_mut(&mut self) -> Result<&mut [u8], Error> {
        unsafe {
            let mut buf: *mut c_void = std::ptr::null_mut();
            void_result(decklink_video_frame_get_bytes(
                self.implementation(),
                &mut buf,
            ))?;
            Ok(std::slice::from_raw_parts_mut(
                buf as *mut u8,
                (self.get_row_bytes() * self.get_height()) as usize,
            ))
        }
    }
}

impl VideoFrame for MutableVideoFrame {
    unsafe fn implementation(&mut self) -> *mut IDeckLinkVideoFrame {
        self.implementation as _
    }
}

impl VideoFrame for &mut MutableVideoFrame {
    unsafe fn implementation(&mut self) -> *mut IDeckLinkVideoFrame {
        self.implementation as _
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimecodeFormat(pub u32);

impl TimecodeFormat {
    pub const FORMAT_RP188_VITC1: TimecodeFormat =
        TimecodeFormat(_BMDTimecodeFormat_bmdTimecodeRP188VITC1);
    pub const FORMAT_RP188_VITC2: TimecodeFormat =
        TimecodeFormat(_BMDTimecodeFormat_bmdTimecodeRP188VITC2);
    pub const FORMAT_RP188_LTC: TimecodeFormat =
        TimecodeFormat(_BMDTimecodeFormat_bmdTimecodeRP188LTC);
    pub const FORMAT_RP188_HIGH_FRAME_RATE: TimecodeFormat =
        TimecodeFormat(_BMDTimecodeFormat_bmdTimecodeRP188HighFrameRate);
    pub const FORMAT_RP188_ANY: TimecodeFormat =
        TimecodeFormat(_BMDTimecodeFormat_bmdTimecodeRP188Any);
    pub const FORMAT_VITC: TimecodeFormat = TimecodeFormat(_BMDTimecodeFormat_bmdTimecodeVITC);
    pub const FORMAT_VITC_FIELD2: TimecodeFormat =
        TimecodeFormat(_BMDTimecodeFormat_bmdTimecodeVITCField2);
    pub const FORMAT_SERIAL: TimecodeFormat = TimecodeFormat(_BMDTimecodeFormat_bmdTimecodeSerial);
}

pub struct Timecode {
    implementation: *mut IDeckLinkTimecode,
}

unsafe impl Send for Timecode {}

impl Drop for Timecode {
    fn drop(&mut self) {
        unsafe {
            unknown_release(self.implementation as *mut IUnknown);
        }
    }
}

impl Timecode {
    pub fn get_components(&self) -> Result<(u8, u8, u8, u8), Error> {
        unsafe {
            let mut components = (0, 0, 0, 0);
            void_result(decklink_timecode_get_components(
                self.implementation,
                &mut components.0,
                &mut components.1,
                &mut components.2,
                &mut components.3,
            ))?;
            Ok(components)
        }
    }

    pub fn get_string(&self) -> Result<String, Error> {
        unsafe {
            let mut v: *mut Buffer = std::ptr::null_mut();
            void_result(decklink_timecode_get_string(self.implementation, &mut v))?;
            let ret = Ok(std::ffi::CStr::from_ptr(buffer_data(v) as *const i8)
                .to_str()
                .unwrap_or("")
                .to_string());
            buffer_release(v);
            ret
        }
    }
}
