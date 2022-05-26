#include <DeckLinkAPI.h>

struct Buffer;

extern "C" {

HRESULT decklink_get_e_fail();

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
HRESULT decklink_input_enable_audio_input(IDeckLinkInput* input, BMDAudioSampleRate sampleRate, BMDAudioSampleType sampleType, uint32_t channelCount);
HRESULT decklink_input_enable_video_input(IDeckLinkInput* input, BMDDisplayMode displayMode, BMDPixelFormat pixelFormat, BMDVideoInputFlags flags);
HRESULT decklink_input_disable_video_input(IDeckLinkInput* input);
HRESULT decklink_input_disable_audio_input(IDeckLinkInput* input);
HRESULT decklink_input_set_callback(IDeckLinkInput* input, IDeckLinkInputCallback* callback);
HRESULT decklink_input_get_hardware_reference_clock(IDeckLinkInput* input, BMDTimeScale timeScale, BMDTimeValue* hardwareTime, BMDTimeValue* timeInFrame, BMDTimeValue* ticksPerFrame);

HRESULT decklink_output_get_display_mode_iterator(IDeckLinkOutput* output, IDeckLinkDisplayModeIterator** iterator);
HRESULT decklink_output_create_video_frame(IDeckLinkOutput* output, int32_t width, int32_t height, int32_t rowBytes, BMDPixelFormat pixelFormat, BMDFrameFlags flags, IDeckLinkMutableVideoFrame **outFrame);
HRESULT decklink_output_disable_video_output(IDeckLinkOutput* output);
HRESULT decklink_output_enable_video_output(IDeckLinkOutput* output, BMDDisplayMode displayMode, BMDVideoOutputFlags flags);
HRESULT decklink_output_set_scheduled_frame_completion_callback(IDeckLinkOutput* output, IDeckLinkVideoOutputCallback* callback);
HRESULT decklink_output_start_scheduled_playback(IDeckLinkOutput* output, BMDTimeValue playbackStartTime, BMDTimeScale timeScale, double playbackSpeed);
HRESULT decklink_output_schedule_video_frame(IDeckLinkOutput* output, IDeckLinkVideoFrame* theFrame, BMDTimeValue displayTime, BMDTimeValue displayDuration, BMDTimeScale timeScale);

HRESULT decklink_display_mode_iterator_next(IDeckLinkDisplayModeIterator* iterator, IDeckLinkDisplayMode** deckLinkDisplayMode);

BMDDisplayMode decklink_display_mode_get_display_mode(IDeckLinkDisplayMode* mode);
HRESULT decklink_display_mode_get_name(IDeckLinkDisplayMode* mode, Buffer** value);
HRESULT decklink_display_mode_get_frame_rate(IDeckLinkDisplayMode* mode, BMDTimeValue* frameDuration, BMDTimeScale* timeScale);
long decklink_display_mode_get_width(IDeckLinkDisplayMode* mode);
long decklink_display_mode_get_height(IDeckLinkDisplayMode* mode);
BMDFieldDominance decklink_display_mode_get_field_dominance(IDeckLinkDisplayMode* mode);

IDeckLinkInputCallback* create_decklink_input_callback(void* implementation);
IDeckLinkVideoOutputCallback* create_decklink_video_output_callback(void* implementation);

long decklink_audio_input_packet_get_sample_frame_count(IDeckLinkAudioInputPacket* packet);
HRESULT decklink_audio_input_packet_get_bytes(IDeckLinkAudioInputPacket* packet, void** bytes);
HRESULT decklink_audio_input_packet_get_packet_time(IDeckLinkAudioInputPacket* packet, BMDTimeValue* packetTime, BMDTimeScale timeScale);

long decklink_video_frame_get_width(IDeckLinkVideoFrame* frame);
long decklink_video_frame_get_height(IDeckLinkVideoFrame* frame);
long decklink_video_frame_get_row_bytes(IDeckLinkVideoFrame* frame);
BMDPixelFormat decklink_video_frame_get_pixel_format(IDeckLinkVideoFrame* frame);
BMDFrameFlags decklink_video_frame_get_flags(IDeckLinkVideoFrame* frame);
HRESULT decklink_video_frame_get_bytes(IDeckLinkVideoFrame* frame, void** bytes);
HRESULT decklink_video_frame_get_timecode(IDeckLinkVideoFrame* frame, BMDTimecodeFormat format, IDeckLinkTimecode** timecode);

HRESULT decklink_video_input_frame_get_stream_time(IDeckLinkVideoInputFrame* frame, BMDTimeValue* frameTime, BMDTimeValue* frameDuration, BMDTimeScale timeScale);
HRESULT decklink_video_input_frame_get_hardware_reference_timestamp(IDeckLinkVideoInputFrame* frame, BMDTimeScale timeScale, BMDTimeValue* frameTime, BMDTimeValue* frameDuration);

IDeckLinkVideoConversion* create_decklink_video_conversion_instance();
HRESULT decklink_video_conversion_convert_frame(IDeckLinkVideoConversion* conversion, IDeckLinkVideoFrame* srcFrame, IDeckLinkVideoFrame* dstFrame);

HRESULT decklink_timecode_get_components(IDeckLinkTimecode* timecode, uint8_t* hours, uint8_t* minutes, uint8_t* seconds, uint8_t* frames);
HRESULT decklink_timecode_get_string(IDeckLinkTimecode* timecode, Buffer** value);

IDeckLinkAPIInformation* create_decklink_api_information_instance();
HRESULT decklink_api_information_get_flag(IDeckLinkAPIInformation* apiInfo, BMDDeckLinkAPIInformationID cfgID, bool* value);
HRESULT decklink_api_information_get_int(IDeckLinkAPIInformation* apiInfo, BMDDeckLinkAPIInformationID cfgID, int64_t* value);
HRESULT decklink_api_information_get_float(IDeckLinkAPIInformation* apiInfo, BMDDeckLinkAPIInformationID cfgID, double* value);
HRESULT decklink_api_information_get_string(IDeckLinkAPIInformation* apiInfo, BMDDeckLinkAPIInformationID cfgID, Buffer** value);

const void* buffer_data(Buffer* str);
void buffer_release(Buffer* str);

}
