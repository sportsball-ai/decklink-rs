#include "lib.hpp"

#include <atomic>
#include <cstdlib>

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

HRESULT decklink_display_mode_iterator_next(IDeckLinkDisplayModeIterator* iterator, IDeckLinkDisplayMode** deckLinkDisplayMode) {
	return iterator->Next(deckLinkDisplayMode);
}

BMDDisplayMode decklink_display_mode_get_display_mode(IDeckLinkDisplayMode* mode) {
	return mode->GetDisplayMode();
}

HRESULT decklink_display_mode_get_name(IDeckLinkDisplayMode* mode, Buffer** value) {
	return mode->GetName(StringArg(value));
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

IDeckLinkVideoConversion* create_decklink_video_conversion_instance() {
	return CreateVideoConversionInstance();
}

HRESULT decklink_video_conversion_convert_frame(IDeckLinkVideoConversion* conversion, IDeckLinkVideoFrame* srcFrame, IDeckLinkVideoFrame* dstFrame) {
    return conversion->ConvertFrame(srcFrame, dstFrame);
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
