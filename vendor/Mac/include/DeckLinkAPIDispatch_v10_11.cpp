/* -LICENSE-START-
** Copyright (c) 2019 Blackmagic Design
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
/* DeckLinkAPIDispatch.cpp */

#include "DeckLinkAPI_v10_11.h"
#include <pthread.h>

#if BLACKMAGIC_DECKLINK_API_MAGIC != 1
	#error The DeckLink API version of DeckLinkAPIDispatch.cpp is not the same version as DeckLinkAPI.h
#endif

#define kDeckLinkAPI_BundlePath "/Library/Frameworks/DeckLinkAPI.framework"


typedef IDeckLinkIterator* (*CreateIteratorFunc)(void);
typedef IDeckLinkAPIInformation* (*CreateAPIInformationFunc)(void);
typedef IDeckLinkGLScreenPreviewHelper* (*CreateOpenGLScreenPreviewHelperFunc)(void);
typedef IDeckLinkCocoaScreenPreviewCallback* (*CreateCocoaScreenPreviewFunc)(void*);
typedef IDeckLinkVideoConversion* (*CreateVideoConversionInstanceFunc)(void);
typedef IDeckLinkDiscovery* (*CreateDeckLinkDiscoveryInstanceFunc)(void);
typedef IDeckLinkVideoFrameAncillaryPackets* (*CreateVideoFrameAncillaryPacketsInstanceFunc)(void);

static pthread_once_t						gDeckLinkOnceControl		= PTHREAD_ONCE_INIT;
static CFBundleRef							gDeckLinkAPIBundleRef		= NULL;
static CreateIteratorFunc					gCreateIteratorFunc			= NULL;
static CreateAPIInformationFunc				gCreateAPIInformationFunc	= NULL;
static CreateOpenGLScreenPreviewHelperFunc	gCreateOpenGLPreviewFunc	= NULL;
static CreateCocoaScreenPreviewFunc			gCreateCocoaPreviewFunc		= NULL;
static CreateVideoConversionInstanceFunc	gCreateVideoConversionFunc	= NULL;
static CreateDeckLinkDiscoveryInstanceFunc  gCreateDeckLinkDiscoveryFunc= NULL;
static CreateVideoFrameAncillaryPacketsInstanceFunc	gCreateVideoFrameAncillaryPacketsFunc = NULL;


static void	InitDeckLinkAPI (void)
{
	CFURLRef		bundleURL;

	bundleURL = CFURLCreateWithFileSystemPath(kCFAllocatorDefault, CFSTR(kDeckLinkAPI_BundlePath), kCFURLPOSIXPathStyle, true);
	if (bundleURL != NULL)
	{
		gDeckLinkAPIBundleRef = CFBundleCreate(kCFAllocatorDefault, bundleURL);
		if (gDeckLinkAPIBundleRef != NULL)
		{
			gCreateIteratorFunc = (CreateIteratorFunc)CFBundleGetFunctionPointerForName(gDeckLinkAPIBundleRef, CFSTR("CreateDeckLinkIteratorInstance_0003"));
			gCreateAPIInformationFunc = (CreateAPIInformationFunc)CFBundleGetFunctionPointerForName(gDeckLinkAPIBundleRef, CFSTR("CreateDeckLinkAPIInformationInstance_0001"));
			gCreateOpenGLPreviewFunc = (CreateOpenGLScreenPreviewHelperFunc)CFBundleGetFunctionPointerForName(gDeckLinkAPIBundleRef, CFSTR("CreateOpenGLScreenPreviewHelper_0001"));
			gCreateCocoaPreviewFunc = (CreateCocoaScreenPreviewFunc)CFBundleGetFunctionPointerForName(gDeckLinkAPIBundleRef, CFSTR("CreateCocoaScreenPreview_0001"));
			gCreateVideoConversionFunc = (CreateVideoConversionInstanceFunc)CFBundleGetFunctionPointerForName(gDeckLinkAPIBundleRef, CFSTR("CreateVideoConversionInstance_0001"));
            gCreateDeckLinkDiscoveryFunc = (CreateDeckLinkDiscoveryInstanceFunc)CFBundleGetFunctionPointerForName(gDeckLinkAPIBundleRef, CFSTR("CreateDeckLinkDiscoveryInstance_0002"));
            gCreateVideoFrameAncillaryPacketsFunc = (CreateVideoFrameAncillaryPacketsInstanceFunc)CFBundleGetFunctionPointerForName(gDeckLinkAPIBundleRef, CFSTR("CreateVideoFrameAncillaryPacketsInstance_0001"));
		}
		CFRelease(bundleURL);
	}
}

bool		IsDeckLinkAPIPresent_v10_11 (void)
{
	// If the DeckLink API bundle was successfully loaded, return this knowledge to the caller
	if (gDeckLinkAPIBundleRef != NULL)
		return true;
	
	return false;
}

IDeckLinkIterator*		CreateDeckLinkIteratorInstance_v10_11 (void)
{
	pthread_once(&gDeckLinkOnceControl, InitDeckLinkAPI);
	
	if (gCreateIteratorFunc == NULL)
		return NULL;
	
	return gCreateIteratorFunc();
}

IDeckLinkAPIInformation*	CreateDeckLinkAPIInformationInstance_v10_11 (void)
{
	pthread_once(&gDeckLinkOnceControl, InitDeckLinkAPI);
	
	if (gCreateAPIInformationFunc == NULL)
		return NULL;
	
	return gCreateAPIInformationFunc();
}

IDeckLinkGLScreenPreviewHelper*		CreateOpenGLScreenPreviewHelper_v10_11 (void)
{
	pthread_once(&gDeckLinkOnceControl, InitDeckLinkAPI);
	
	if (gCreateOpenGLPreviewFunc == NULL)
		return NULL;
	
	return gCreateOpenGLPreviewFunc();
}

IDeckLinkCocoaScreenPreviewCallback*	CreateCocoaScreenPreview_v10_11 (void* parentView)
{
	pthread_once(&gDeckLinkOnceControl, InitDeckLinkAPI);
	
	if (gCreateCocoaPreviewFunc == NULL)
		return NULL;
	
	return gCreateCocoaPreviewFunc(parentView);
}

IDeckLinkVideoConversion* CreateVideoConversionInstance_v10_11 (void)
{
	pthread_once(&gDeckLinkOnceControl, InitDeckLinkAPI);
	
	if (gCreateVideoConversionFunc == NULL)
		return NULL;
	
	return gCreateVideoConversionFunc();
}

IDeckLinkDiscovery* CreateDeckLinkDiscoveryInstance_v10_11 (void)
{
	pthread_once(&gDeckLinkOnceControl, InitDeckLinkAPI);
	
	if (gCreateDeckLinkDiscoveryFunc == NULL)
		return NULL;
	
	return gCreateDeckLinkDiscoveryFunc();
}

IDeckLinkVideoFrameAncillaryPackets* CreateVideoFrameAncillaryPacketsInstance_v10_11 (void)
{
	pthread_once(&gDeckLinkOnceControl, InitDeckLinkAPI);
	
	if (gCreateVideoFrameAncillaryPacketsFunc == NULL)
		return NULL;
	
	return gCreateVideoFrameAncillaryPacketsFunc();
}


#define kBMDStreamingAPI_BundlePath "/Library/Application Support/Blackmagic Design/Streaming/BMDStreamingAPI.bundle"

typedef IBMDStreamingDiscovery* (*CreateDiscoveryFunc)(void);
typedef IBMDStreamingH264NALParser* (*CreateNALParserFunc)(void);

static pthread_once_t      gBMDStreamingOnceControl  = PTHREAD_ONCE_INIT;
static CFBundleRef         gBMDStreamingAPIBundleRef = NULL;
static CreateDiscoveryFunc gCreateDiscoveryFunc      = NULL;
static CreateNALParserFunc gCreateNALParserFunc      = NULL;

static void InitBMDStreamingAPI(void)
{
	CFURLRef bundleURL;

	bundleURL = CFURLCreateWithFileSystemPath(kCFAllocatorDefault, CFSTR(kBMDStreamingAPI_BundlePath), kCFURLPOSIXPathStyle, true);
	if (bundleURL != NULL)
	{
		gBMDStreamingAPIBundleRef = CFBundleCreate(kCFAllocatorDefault, bundleURL);
		if (gBMDStreamingAPIBundleRef != NULL)
		{
			gCreateDiscoveryFunc = (CreateDiscoveryFunc)CFBundleGetFunctionPointerForName(gBMDStreamingAPIBundleRef, CFSTR("CreateBMDStreamingDiscoveryInstance_0002"));
			gCreateNALParserFunc = (CreateNALParserFunc)CFBundleGetFunctionPointerForName(gBMDStreamingAPIBundleRef, CFSTR("CreateBMDStreamingH264NALParser_0001"));
		}

		CFRelease(bundleURL);
	}
}

IBMDStreamingDiscovery* CreateBMDStreamingDiscoveryInstance_v10_11()
{
	pthread_once(&gBMDStreamingOnceControl, InitBMDStreamingAPI);

	if (gCreateDiscoveryFunc == NULL)
		return NULL;

	return gCreateDiscoveryFunc();
}

IBMDStreamingH264NALParser* CreateBMDStreamingH264NALParser_v10_11()
{
	pthread_once(&gBMDStreamingOnceControl, InitBMDStreamingAPI);

	if (gCreateNALParserFunc == NULL)
		return NULL;

	return gCreateNALParserFunc();
}
