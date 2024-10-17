// dllmain.cpp : 定义 DLL 应用程序的入口点。
#include "pch.h"
#include "playerInstance.h"

BOOL APIENTRY DllMain( HMODULE hModule,
                       DWORD  ul_reason_for_call,
                       LPVOID lpReserved
                     )
{
    switch (ul_reason_for_call)
    {
    case DLL_PROCESS_ATTACH:
    case DLL_THREAD_ATTACH:
    case DLL_THREAD_DETACH:
    case DLL_PROCESS_DETACH:
        break;
    }
    return TRUE;
}

 extern "C" __declspec(dllexport) BOOL set_wallpaper(const char* window_title) {
     return set_as_wallpaper(window_title);
 }

 extern "C" __declspec(dllexport) BOOL any_maximized() {
     return !detectWindowMaximized();
 }