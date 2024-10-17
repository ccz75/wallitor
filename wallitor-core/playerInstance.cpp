#include "pch.h"
#include "playerInstance.h"

HWND hWorkerw = NULL;

static BOOL CALLBACK EnumWindowsProc(_In_ HWND hwnd, _In_ LPARAM Lparam) {
	HWND hDefView = FindWindowEx(hwnd, 0, L"SHELLDLL_DefView", 0);
	if (hDefView != 0) {
		hWorkerw = FindWindowEx(0, hwnd, L"WorkerW", 0);
		return FALSE;		
	}
	return TRUE;
}

BOOL try_find_worker(HWND hProgman,HWND hPlayer) {
	HWND hWorkerW = FindWindowEx(hProgman, 0, L"WorkerW", 0);
	if (hWorkerW != NULL) {
		SetParent(hPlayer, hWorkerW);
		return TRUE;
	}
	else return FALSE;
}

HWND shell_in_progman(HWND hProgman) {
	HWND hShell = FindWindowEx(hProgman, 0, L"SHELLDLL_DefView", 0);
	return hShell;
}

BOOL set_as_wallpaper(const char* window_title) {
	HWND hProgman = FindWindow(L"Progman", 0);// 找到PI窗口
	SendMessageTimeout(hProgman, 0x052c, 0, 0, SMTO_NORMAL, 0x3e8, 0);// 给它发特殊消息
	std::string tmp = window_title;
	HWND hPlayer = FindWindowW(0, std::wstring(tmp.begin(), tmp.end()).c_str());// 找到视频窗口
	if (hPlayer == NULL) return FALSE;
	HWND hDefView = shell_in_progman(hProgman);
	if (hDefView!=NULL) {
		if (try_find_worker(hProgman, hPlayer)) {
			ShowWindow(hDefView, SW_HIDE);
			Sleep(0);
			ShowWindow(hDefView, SW_SHOWNORMAL);
			return TRUE;
		}
		else return FALSE;
	}
	else {
		SetParent(hPlayer, hProgman);// 将视频窗口设苦为PM的子窗口
		EnumWindows(EnumWindowsProc, 0);// 找到第二个workerw窗口并隐藏它
		if (hWorkerw) {
			ShowWindow(hWorkerw, SW_HIDE);
			return TRUE;
		}
		return FALSE;
	}
}

static BOOL CALLBACK CheckMaximized(_In_ HWND hwnd, _In_ LPARAM Lparam) {
	BOOL isMaximized = IsZoomed(hwnd);
	if (isMaximized) return FALSE;
	else return TRUE;
}

BOOL detectWindowMaximized() {
	return EnumWindows(CheckMaximized, 0);
}