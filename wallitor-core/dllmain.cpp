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

playerInstance* player_instance = NULL;

 extern "C" __declspec(dllexport) void init(const char* ffpath,const char* videoPath, BOOL mute) {
     player_instance = new playerInstance(conFig(ffpath, videoPath, mute));
     player_instance->generate();
}

 extern "C" __declspec(dllexport)  void destroy() {
     player_instance->exit();
     delete player_instance;
 }

 extern "C" __declspec(dllexport) BOOL set_wallpaper(const char* window_title) {
     return set_as_wallpaper(window_title);
 }

 extern "C" __declspec(dllexport) BOOL any_maximized() {
     return !detectWindowMaximized();
 }