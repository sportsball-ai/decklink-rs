#include <DeckLinkAPI.h>

struct Buffer;

extern "C" {

ULONG unknown_add_ref(IUnknown* obj);
ULONG unknown_release(IUnknown* obj);

IDeckLinkIterator* create_decklink_iterator_instance();
HRESULT decklink_iterator_next(IDeckLinkIterator* iterator, IDeckLink** deckLinkInstance);

HRESULT decklink_get_model_name(IDeckLink* decklink, Buffer** str);
HRESULT decklink_query_interface(IDeckLink* decklink, REFIID iid, LPVOID* iface);

HRESULT decklink_attributes_get_flag(IDeckLinkAttributes* attr, BMDDeckLinkAttributeID cfgID, bool* value);
HRESULT decklink_attributes_get_int(IDeckLinkAttributes* attr, BMDDeckLinkAttributeID cfgID, int64_t* value);
HRESULT decklink_attributes_get_float(IDeckLinkAttributes* attr, BMDDeckLinkAttributeID cfgID, double* value);
HRESULT decklink_attributes_get_string(IDeckLinkAttributes* attr, BMDDeckLinkAttributeID cfgID, Buffer** value);

HRESULT decklink_status_get_flag(IDeckLinkStatus* status, BMDDeckLinkStatusID statusID, bool* value);
HRESULT decklink_status_get_int(IDeckLinkStatus* status, BMDDeckLinkStatusID statusID, int64_t* value);

HRESULT decklink_input_get_display_mode_iterator(IDeckLinkInput* input, IDeckLinkDisplayModeIterator** iterator);
HRESULT decklink_input_start_streams(IDeckLinkInput* input);
HRESULT decklink_input_stop_streams(IDeckLinkInput* input);
HRESULT decklink_input_pause_streams(IDeckLinkInput* input);
HRESULT decklink_input_flush_streams(IDeckLinkInput* input);
HRESULT decklink_input_enable_video_input(IDeckLinkInput* input, BMDDisplayMode displayMode, BMDPixelFormat pixelFormat, BMDVideoInputFlags flags);
HRESULT decklink_input_disable_video_input(IDeckLinkInput* input);
HRESULT decklink_input_disable_audio_input(IDeckLinkInput* input);
HRESULT decklink_input_set_callback(IDeckLinkInput* input, IDeckLinkInputCallback* callback);

HRESULT decklink_output_get_display_mode_iterator(IDeckLinkOutput* output, IDeckLinkDisplayModeIterator** iterator);
HRESULT decklink_output_create_video_frame(IDeckLinkOutput* output, int32_t width, int32_t height, int32_t rowBytes, BMDPixelFormat pixelFormat, BMDFrameFlags flags, IDeckLinkMutableVideoFrame **outFrame);

HRESULT decklink_display_mode_iterator_next(IDeckLinkDisplayModeIterator* iterator, IDeckLinkDisplayMode** deckLinkDisplayMode);

BMDDisplayMode decklink_display_mode_get_display_mode(IDeckLinkDisplayMode* mode);
HRESULT decklink_display_mode_get_name(IDeckLinkDisplayMode* mode, Buffer** value);

IDeckLinkInputCallback* create_decklink_input_callback(void* implementation);

long decklink_video_frame_get_width(IDeckLinkVideoFrame* frame);
long decklink_video_frame_get_height(IDeckLinkVideoFrame* frame);
long decklink_video_frame_get_row_bytes(IDeckLinkVideoFrame* frame);
BMDPixelFormat decklink_video_frame_get_pixel_format(IDeckLinkVideoFrame* frame);
BMDFrameFlags decklink_video_frame_get_flags(IDeckLinkVideoFrame* frame);
HRESULT decklink_video_frame_get_bytes(IDeckLinkVideoFrame* frame, void** bytes);

IDeckLinkVideoConversion* create_decklink_video_conversion_instance();
HRESULT decklink_video_conversion_convert_frame(IDeckLinkVideoConversion* conversion, IDeckLinkVideoFrame* srcFrame, IDeckLinkVideoFrame* dstFrame);

const void* buffer_data(Buffer* str);
void buffer_release(Buffer* str);

}
