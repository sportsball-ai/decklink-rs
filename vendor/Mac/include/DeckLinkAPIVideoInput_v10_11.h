/* -LICENSE-START-
** Copyright (c) 2017 Blackmagic Design
**  
** Permission is hereby granted, free of charge, to any person or organization 
** obtaining a copy of the software and accompanying documentation (the 
** "Software") to use, reproduce, display, distribute, sub-license, execute, 
** and transmit the Software, and to prepare derivative works of the Software, 
** and to permit third-parties to whom the Software is furnished to do so, in 
** accordance with:
** 
** (1) if the Software is obtained from Blackmagic Design, the End User License 
** Agreement for the Software Development Kit ("EULA") available at 
** https://www.blackmagicdesign.com/EULA/DeckLinkSDK; or
** 
** (2) if the Software is obtained from any third party, such licensing terms 
** as notified by that third party,
** 
** and all subject to the following:
** 
** (3) the copyright notices in the Software and this entire statement, 
** including the above license grant, this restriction and the following 
** disclaimer, must be included in all copies of the Software, in whole or in 
** part, and all derivative works of the Software, unless such copies or 
** derivative works are solely in the form of machine-executable object code 
** generated by a source language processor.
** 
** (4) THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS 
** OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, 
** FITNESS FOR A PARTICULAR PURPOSE, TITLE AND NON-INFRINGEMENT. IN NO EVENT 
** SHALL THE COPYRIGHT HOLDERS OR ANYONE DISTRIBUTING THE SOFTWARE BE LIABLE 
** FOR ANY DAMAGES OR OTHER LIABILITY, WHETHER IN CONTRACT, TORT OR OTHERWISE, 
** ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER 
** DEALINGS IN THE SOFTWARE.
** 
** A copy of the Software is available free of charge at 
** https://www.blackmagicdesign.com/desktopvideo_sdk under the EULA.
** 
** -LICENSE-END-
*/

#ifndef BMD_DECKLINKAPIVIDEOINPUT_v10_11_H
#define BMD_DECKLINKAPIVIDEOINPUT_v10_11_H

#include "DeckLinkAPI.h"
#include "DeckLinkAPI_v10_11.h"
#include "DeckLinkAPIVideoInput_v11_5_1.h"

// Type Declarations

BMD_CONST REFIID IID_IDeckLinkInput_v10_11                               = /* AF22762B-DFAC-4846-AA79-FA8883560995 */ {0xAF,0x22,0x76,0x2B,0xDF,0xAC,0x48,0x46,0xAA,0x79,0xFA,0x88,0x83,0x56,0x09,0x95};

/* Interface IDeckLinkInput_v10_11 - DeckLink input interface. */

class IDeckLinkInput_v10_11 : public IUnknown
{
public:
    virtual HRESULT DoesSupportVideoMode (/* in */ BMDDisplayMode displayMode, /* in */ BMDPixelFormat pixelFormat, /* in */ BMDVideoInputFlags flags, /* out */ BMDDisplayModeSupport_v10_11 *result, /* out */ IDeckLinkDisplayMode **resultDisplayMode) = 0;
    virtual HRESULT GetDisplayModeIterator (/* out */ IDeckLinkDisplayModeIterator **iterator) = 0;

    virtual HRESULT SetScreenPreviewCallback (/* in */ IDeckLinkScreenPreviewCallback *previewCallback) = 0;

    /* Video Input */

    virtual HRESULT EnableVideoInput (/* in */ BMDDisplayMode displayMode, /* in */ BMDPixelFormat pixelFormat, /* in */ BMDVideoInputFlags flags) = 0;
    virtual HRESULT DisableVideoInput (void) = 0;
    virtual HRESULT GetAvailableVideoFrameCount (/* out */ uint32_t *availableFrameCount) = 0;
    virtual HRESULT SetVideoInputFrameMemoryAllocator (/* in */ IDeckLinkMemoryAllocator *theAllocator) = 0;

    /* Audio Input */

    virtual HRESULT EnableAudioInput (/* in */ BMDAudioSampleRate sampleRate, /* in */ BMDAudioSampleType sampleType, /* in */ uint32_t channelCount) = 0;
    virtual HRESULT DisableAudioInput (void) = 0;
    virtual HRESULT GetAvailableAudioSampleFrameCount (/* out */ uint32_t *availableSampleFrameCount) = 0;

    /* Input Control */

    virtual HRESULT StartStreams (void) = 0;
    virtual HRESULT StopStreams (void) = 0;
    virtual HRESULT PauseStreams (void) = 0;
    virtual HRESULT FlushStreams (void) = 0;
    virtual HRESULT SetCallback (/* in */ IDeckLinkInputCallback_v11_5_1 *theCallback) = 0;

    /* Hardware Timing */

    virtual HRESULT GetHardwareReferenceClock (/* in */ BMDTimeScale desiredTimeScale, /* out */ BMDTimeValue *hardwareTime, /* out */ BMDTimeValue *timeInFrame, /* out */ BMDTimeValue *ticksPerFrame) = 0;

protected:
    virtual ~IDeckLinkInput_v10_11 () {} // call Release method to drop reference count
};

#endif /* defined(BMD_DECKLINKAPIVIDEOINPUT_v10_11_H) */
