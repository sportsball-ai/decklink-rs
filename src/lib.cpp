#include "lib.hpp"

#include <atomic>
#include <cstdlib>
#include <cstring>

HRESULT decklink_get_e_fail() {
    return E_FAIL;
}

struct Buffer {
    explicit Buffer(const char* data) : _data(data) {}
    ~Buffer() {
        free((void*)_data);
    }
    const char* _data;
};

struct StringArg {
    explicit StringArg(Buffer** dest) : _temp(nullptr), _dest(dest) {}

#ifdef __APPLE__
    ~StringArg() {
        if (_temp == nullptr) {
            *_dest = nullptr;
        } else {
            CFIndex length = CFStringGetLength(_temp);
            CFIndex maxSize = CFStringGetMaximumSizeForEncoding(length, kCFStringEncodingUTF8) + 1;
            char* data = (char*)malloc(maxSize);
            CFStringGetCString(_temp, data, maxSize, kCFStringEncodingUTF8);
            *_dest = new Buffer(data);
            CFRelease(_temp);
        }
    }

    operator CFStringRef*() {
        return &_temp;
    }

    CFStringRef _temp;
#else
    ~StringArg() {
        *_dest = new Buffer(_temp);
    }

    operator const char**() {
        return &_temp;
    }

    const char* _temp;
#endif

    Buffer** _dest;
};

extern "C" {

ULONG unknown_add_ref(IUnknown* obj) {
    return obj->AddRef();
}

ULONG unknown_release(IUnknown* obj) {
    return obj->Release();
}

IDeckLinkIterator* create_decklink_iterator_instance() {
    return CreateDeckLinkIteratorInstance();
}

HRESULT decklink_iterator_next(IDeckLinkIterator* iterator, IDeckLink** deckLinkInstance) {
    return iterator->Next(deckLinkInstance);
}

HRESULT decklink_get_model_name(IDeckLink* decklink, Buffer** str) {
    return decklink->GetModelName(StringArg(str));
}

HRESULT decklink_query_interface(IDeckLink* decklink, REFIID iid, LPVOID* iface) {
    return decklink->QueryInterface(iid, iface);
}

HRESULT decklink_attributes_get_flag(IDeckLinkAttributes* attr, BMDDeckLinkAttributeID cfgID, bool* value) {
    return attr->GetFlag(cfgID, value);
}

HRESULT decklink_attributes_get_int(IDeckLinkAttributes* attr, BMDDeckLinkAttributeID cfgID, int64_t* value) {
    return attr->GetInt(cfgID, value);
}

HRESULT decklink_attributes_get_float(IDeckLinkAttributes* attr, BMDDeckLinkAttributeID cfgID, double* value) {
    return attr->GetFloat(cfgID, value);
}

HRESULT decklink_attributes_get_string(IDeckLinkAttributes* attr, BMDDeckLinkAttributeID cfgID, Buffer** value) {
    return attr->GetString(cfgID, StringArg(value));
}

HRESULT decklink_status_get_flag(IDeckLinkStatus* status, BMDDeckLinkStatusID statusID, bool* value) {
    return status->GetFlag(statusID, value);
}

HRESULT decklink_status_get_int(IDeckLinkStatus* status, BMDDeckLinkStatusID statusID, int64_t* value) {
    return status->GetInt(statusID, value);
}

HRESULT decklink_input_get_display_mode_iterator(IDeckLinkInput* input, IDeckLinkDisplayModeIterator** iterator) {
	return input->GetDisplayModeIterator(iterator);
}

HRESULT decklink_input_start_streams(IDeckLinkInput* input) {
	return input->StartStreams();
}

HRESULT decklink_input_stop_streams(IDeckLinkInput* input) {
	return input->StopStreams();
}

HRESULT decklink_input_pause_streams(IDeckLinkInput* input) {
	return input->PauseStreams();
}

HRESULT decklink_input_flush_streams(IDeckLinkInput* input) {
	return input->FlushStreams();
}

HRESULT decklink_input_enable_audio_input(IDeckLinkInput* input, BMDAudioSampleRate sampleRate, BMDAudioSampleType sampleType, uint32_t channelCount) {
    return input->EnableAudioInput(sampleRate, sampleType, channelCount);
}

HRESULT decklink_input_enable_video_input(IDeckLinkInput* input, BMDDisplayMode displayMode, BMDPixelFormat pixelFormat, BMDVideoInputFlags flags) {
    return input->EnableVideoInput(displayMode, pixelFormat, flags);
}

HRESULT decklink_input_disable_video_input(IDeckLinkInput* input) {
	return input->DisableVideoInput();
}

HRESULT decklink_input_disable_audio_input(IDeckLinkInput* input) {
	return input->DisableAudioInput();
}

HRESULT decklink_input_set_callback(IDeckLinkInput* input, IDeckLinkInputCallback* callback) {
    return input->SetCallback(callback);
}

HRESULT decklink_output_get_display_mode_iterator(IDeckLinkOutput* output, IDeckLinkDisplayModeIterator** iterator) {
	return output->GetDisplayModeIterator(iterator);
}

HRESULT decklink_output_create_video_frame(IDeckLinkOutput* output, int32_t width, int32_t height, int32_t rowBytes, BMDPixelFormat pixelFormat, BMDFrameFlags flags, IDeckLinkMutableVideoFrame **outFrame) {
	return output->CreateVideoFrame(width, height, rowBytes, pixelFormat, flags, outFrame);
}

HRESULT decklink_output_disable_video_output(IDeckLinkOutput* output) {
    return output->DisableVideoOutput();
}

HRESULT decklink_output_enable_video_output(IDeckLinkOutput* output, BMDDisplayMode displayMode, BMDVideoOutputFlags flags) {
    return output->EnableVideoOutput(displayMode, flags);
}

HRESULT decklink_output_set_scheduled_frame_completion_callback(IDeckLinkOutput* output, IDeckLinkVideoOutputCallback* callback) {
    return output->SetScheduledFrameCompletionCallback(callback);
}

HRESULT decklink_output_start_scheduled_playback(IDeckLinkOutput* output, BMDTimeValue playbackStartTime, BMDTimeScale timeScale, double playbackSpeed) {
    return output->StartScheduledPlayback(playbackStartTime, timeScale, playbackSpeed);
}

HRESULT decklink_output_schedule_video_frame(IDeckLinkOutput* output, IDeckLinkVideoFrame* theFrame, BMDTimeValue displayTime, BMDTimeValue displayDuration, BMDTimeScale timeScale) {
    return output->ScheduleVideoFrame(theFrame, displayTime, displayDuration, timeScale);
}

HRESULT decklink_display_mode_iterator_next(IDeckLinkDisplayModeIterator* iterator, IDeckLinkDisplayMode** deckLinkDisplayMode) {
	return iterator->Next(deckLinkDisplayMode);
}

BMDDisplayMode decklink_display_mode_get_display_mode(IDeckLinkDisplayMode* mode) {
	return mode->GetDisplayMode();
}

HRESULT decklink_display_mode_get_name(IDeckLinkDisplayMode* mode, Buffer** value) {
	return mode->GetName(StringArg(value));
}

HRESULT decklink_display_mode_get_frame_rate(IDeckLinkDisplayMode* mode, BMDTimeValue* frameDuration, BMDTimeScale* timeScale) {
    return mode->GetFrameRate(frameDuration, timeScale);
}

long decklink_display_mode_get_width(IDeckLinkDisplayMode* mode) {
    return mode->GetWidth();
}

long decklink_display_mode_get_height(IDeckLinkDisplayMode* mode) {
    return mode->GetHeight();
}

BMDFieldDominance decklink_display_mode_get_field_dominance(IDeckLinkDisplayMode* mode) {
    return mode->GetFieldDominance();
}

extern HRESULT input_callback_video_input_format_changed(void*, BMDVideoInputFormatChangedEvents, IDeckLinkDisplayMode*, BMDDetectedVideoInputFormatFlags);
extern HRESULT input_callback_video_input_frame_arrived(void*, IDeckLinkVideoInputFrame*, IDeckLinkAudioInputPacket*);

struct InputCallback: IDeckLinkInputCallback {
    explicit InputCallback(void* implementation) : _ref_count(1), _implementation(implementation) {}
    virtual ~InputCallback() {}

    virtual HRESULT VideoInputFormatChanged(BMDVideoInputFormatChangedEvents notificationEvents, IDeckLinkDisplayMode *newDisplayMode, BMDDetectedVideoInputFormatFlags detectedSignalFlags) {
        return input_callback_video_input_format_changed(_implementation, notificationEvents, newDisplayMode, detectedSignalFlags);
    }

    virtual HRESULT VideoInputFrameArrived(IDeckLinkVideoInputFrame* videoFrame, IDeckLinkAudioInputPacket* audioPacket) {
        return input_callback_video_input_frame_arrived(_implementation, videoFrame, audioPacket);
    }

    virtual HRESULT QueryInterface(REFIID iid, LPVOID *ppv) {
        if (ppv == NULL) {
            return E_INVALIDARG;
        }

        *ppv = NULL;

        CFUUIDBytes iunknown = CFUUIDGetUUIDBytes(IUnknownUUID);
        HRESULT result = E_NOINTERFACE;
        if (memcmp(&iid, &iunknown, sizeof(REFIID)) == 0) {
            *ppv = this;
            AddRef();
            result = S_OK;
        } else if (memcmp(&iid, &IID_IDeckLinkInputCallback, sizeof(REFIID)) == 0) {
            *ppv = (IDeckLinkInputCallback*)this;
            AddRef();
            result = S_OK;
        }

        return result;
    }

    virtual ULONG AddRef() {
        return _ref_count.fetch_add(1);
    }

    virtual ULONG Release() {
        int refs = _ref_count.fetch_sub(1);
        if (refs == 0) {
            delete this;
        }
        return refs;
    }

    std::atomic<uint32_t> _ref_count;
    void* _implementation;
};

IDeckLinkInputCallback* create_decklink_input_callback(void* implementation) {
    return new InputCallback(implementation);
}

extern HRESULT video_output_callback_scheduled_frame_completed(void*, IDeckLinkVideoFrame*, BMDOutputFrameCompletionResult);
extern HRESULT video_output_callback_scheduled_playback_has_stopped(void*);

struct VideoOutputCallback: IDeckLinkVideoOutputCallback {
    explicit VideoOutputCallback(void* implementation) : _ref_count(1), _implementation(implementation) {}
    virtual ~VideoOutputCallback() {}


    virtual HRESULT ScheduledFrameCompleted(IDeckLinkVideoFrame* completedFrame, BMDOutputFrameCompletionResult result) {
        return video_output_callback_scheduled_frame_completed(_implementation, completedFrame, result);
    }

    virtual HRESULT ScheduledPlaybackHasStopped(void) {
        return video_output_callback_scheduled_playback_has_stopped(_implementation);
    }

    virtual HRESULT QueryInterface(REFIID iid, LPVOID *ppv) {
        if (ppv == NULL) {
            return E_INVALIDARG;
        }

        *ppv = NULL;

        CFUUIDBytes iunknown = CFUUIDGetUUIDBytes(IUnknownUUID);
        HRESULT result = E_NOINTERFACE;
        if (memcmp(&iid, &iunknown, sizeof(REFIID)) == 0) {
            *ppv = this;
            AddRef();
            result = S_OK;
        } else if (memcmp(&iid, &IID_IDeckLinkVideoOutputCallback, sizeof(REFIID)) == 0) {
            *ppv = (IDeckLinkVideoOutputCallback*)this;
            AddRef();
            result = S_OK;
        }

        return result;
    }

    virtual ULONG AddRef() {
        return _ref_count.fetch_add(1);
    }

    virtual ULONG Release() {
        int refs = _ref_count.fetch_sub(1);
        if (refs == 0) {
            delete this;
        }
        return refs;
    }

    std::atomic<uint32_t> _ref_count;
    void* _implementation;
};

IDeckLinkVideoOutputCallback* create_decklink_video_output_callback(void* implementation) {
    return new VideoOutputCallback(implementation);
}

long decklink_audio_input_packet_get_sample_frame_count(IDeckLinkAudioInputPacket* packet) {
    return packet->GetSampleFrameCount();
}

HRESULT decklink_audio_input_packet_get_bytes(IDeckLinkAudioInputPacket* packet, void** bytes) {
    return packet->GetBytes(bytes);
}

HRESULT decklink_audio_input_packet_get_packet_time(IDeckLinkAudioInputPacket* packet, BMDTimeValue* packetTime, BMDTimeScale timeScale) {
    return packet->GetPacketTime(packetTime, timeScale);
}

long decklink_video_frame_get_width(IDeckLinkVideoFrame* frame) {
    return frame->GetWidth();
}

long decklink_video_frame_get_height(IDeckLinkVideoFrame* frame) {
    return frame->GetHeight();
}

long decklink_video_frame_get_row_bytes(IDeckLinkVideoFrame* frame) {
    return frame->GetRowBytes();
}

BMDPixelFormat decklink_video_frame_get_pixel_format(IDeckLinkVideoFrame* frame) {
    return frame->GetPixelFormat();
}

BMDFrameFlags decklink_video_frame_get_flags(IDeckLinkVideoFrame* frame) {
    return frame->GetFlags();
}

HRESULT decklink_video_frame_get_bytes(IDeckLinkVideoFrame* frame, void** bytes) {
    return frame->GetBytes(bytes);
}

HRESULT decklink_video_frame_get_timecode(IDeckLinkVideoFrame* frame, BMDTimecodeFormat format, IDeckLinkTimecode** timecode) {
    return frame->GetTimecode(format, timecode);
}

HRESULT decklink_video_input_frame_get_stream_time(IDeckLinkVideoInputFrame* frame, BMDTimeValue* frameTime, BMDTimeValue* frameDuration, BMDTimeScale timeScale) {
    return frame->GetStreamTime(frameTime, frameDuration, timeScale);
}

HRESULT decklink_video_input_frame_get_hardware_reference_timestamp(IDeckLinkVideoInputFrame* frame, BMDTimeScale timeScale, BMDTimeValue* frameTime, BMDTimeValue* frameDuration) {
    return frame->GetHardwareReferenceTimestamp(timeScale, frameTime, frameDuration);
}

IDeckLinkVideoConversion* create_decklink_video_conversion_instance() {
	return CreateVideoConversionInstance();
}

HRESULT decklink_video_conversion_convert_frame(IDeckLinkVideoConversion* conversion, IDeckLinkVideoFrame* srcFrame, IDeckLinkVideoFrame* dstFrame) {
    return conversion->ConvertFrame(srcFrame, dstFrame);
}

HRESULT decklink_timecode_get_components(IDeckLinkTimecode* timecode, uint8_t* hours, uint8_t* minutes, uint8_t* seconds, uint8_t* frames) {
    return timecode->GetComponents(hours, minutes, seconds, frames);
}

HRESULT decklink_timecode_get_string(IDeckLinkTimecode* timecode, Buffer** value) {
    return timecode->GetString(StringArg(value));
}

IDeckLinkAPIInformation* create_decklink_api_information_instance() {
	return CreateDeckLinkAPIInformationInstance();
}

HRESULT decklink_api_information_get_flag(IDeckLinkAPIInformation* apiInfo, BMDDeckLinkAPIInformationID cfgID, bool* value) {
    return apiInfo->GetFlag(cfgID, value);
}

HRESULT decklink_api_information_get_int(IDeckLinkAPIInformation* apiInfo, BMDDeckLinkAPIInformationID cfgID, int64_t* value) {
    return apiInfo->GetInt(cfgID, value);
}

HRESULT decklink_api_information_get_float(IDeckLinkAPIInformation* apiInfo, BMDDeckLinkAPIInformationID cfgID, double* value) {
    return apiInfo->GetFloat(cfgID, value);
}

HRESULT decklink_api_information_get_string(IDeckLinkAPIInformation* apiInfo, BMDDeckLinkAPIInformationID cfgID, Buffer** value) {
    return apiInfo->GetString(cfgID, StringArg(value));
}

const void* buffer_data(Buffer* buf) {
    return buf->_data;
}

void buffer_release(Buffer* obj) {
    if (obj != nullptr) {
        delete obj;
    }
}

}
