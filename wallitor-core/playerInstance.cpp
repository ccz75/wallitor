#include "pch.h"
#include "playerInstance.h"

playerInstance::playerInstance(const conFig& config) {
	this->config = config;
	this->hFfplay = NULL;
}

static BOOL CALLBACK EnumWindowsProc(_In_ HWND hwnd, _In_ LPARAM Lparam) {
	HWND hDefView = FindWindowEx(hwnd, 0, L"SHELLDLL_DefView", 0);
	if (hDefView != 0) {
		HWND hWorkerw = FindWindowEx(0, hwnd, L"WorkerW", 0);
		ShowWindow(hWorkerw, SW_HIDE);
		return FALSE;
	}
	return TRUE;
}

BOOL playerInstance::showWindow(LPCWSTR lpParameter) {
	if (this->hFfplay != NULL) {
		DWORD dwPID = 0;
		GetWindowThreadProcessId(hFfplay, &dwPID);
		char strCmd[MAX_PATH] = { 0 };
		sprintf_s(strCmd, "taskkill /pid %d -f", dwPID);
		system(strCmd);
	}
	STARTUPINFO si{ 0 };
	si.dwFlags = STARTF_USESHOWWINDOW;
	si.wShowWindow = SW_HIDE;
	PROCESS_INFORMATION pi{ 0 };
	if (CreateProcess(this->config.ffpath.c_str(), (LPWSTR)lpParameter, 0, 0, 0, CREATE_NO_WINDOW, 0, 0, &si, &pi)) {
		Sleep(600);//等待视频播放器启动完成
		HWND hProgman = FindWindow(L"Progman", 0);// 找到PI窗口
		SendMessageTimeout(hProgman, 0x052c, 0, 0, 0, 100, 0);// 给它发特殊消息
		this->hFfplay = FindWindowW(L"SDL_app", 0);// 找到视频窗口
		SetParent(hFfplay, hProgman);// 将视频窗口设苦为PM的子窗口
		int systemWidth = GetSystemMetrics(0);
		int systemHeight = GetSystemMetrics(1);
		RECT cRct;
		GetWindowRect(hFfplay, &cRct);
		//if (horw) {
			int width = cRct.right - cRct.left;
			int x = (systemWidth - width) / 2;
			MoveWindow(hFfplay, 0, 0, systemWidth, systemHeight, 1);
		/* }
		else {
			int height = cRct.bottom - cRct.top;
			int x = (sheight - height) / 2;
			MoveWindow(hFfplay, 0, x, cRct.right - cRct.left, cRct.bottom - cRct.top, 0);
		}*/
		EnumWindows(EnumWindowsProc, 0);// 找到第二个workerw窗口并隐藏它
		return TRUE;
	}
	CloseHandle(pi.hProcess);
	CloseHandle(pi.hThread);
	return FALSE;
}

void playerInstance::generate() {
	std::wstring fcmd = L" \"";
	fcmd += this->config.videoPath + L"\"";
	fcmd += L" -noborder -loop 0";
	if (this->config.mute) {
		fcmd += L" -an";
	}
	fcmd += L" -fs";
	this->showWindow(fcmd.c_str());
}

void playerInstance::exit() {
	if (this->hFfplay != NULL) {
		DWORD dwPID = 0;
		GetWindowThreadProcessId(this->hFfplay, &dwPID);
		char strCmd[MAX_PATH] = { 0 };
		sprintf_s(strCmd, "taskkill /pid %d -f", dwPID);
		system(strCmd);
	}
}

BOOL set_as_wallpaper(const char* window_title) {
	HWND hProgman = FindWindow(L"Progman", 0);// 找到PI窗口
	SendMessageTimeout(hProgman, 0x052c, 0, 0, 0, 100, 0);// 给它发特殊消息
	std::string tmp = window_title;
	HWND player = FindWindowW(0, std::wstring(tmp.begin(), tmp.end()).c_str());// 找到视频窗口
	if (player == NULL) return FALSE;
	SetParent(player, hProgman);// 将视频窗口设苦为PM的子窗口
	EnumWindows(EnumWindowsProc, 0);// 找到第二个workerw窗口并隐藏它
	return TRUE;
}

static BOOL CALLBACK CheckMaximized(_In_ HWND hwnd, _In_ LPARAM Lparam) {
	BOOL isMaximized = IsZoomed(hwnd);
	if (isMaximized) return FALSE;
	else return TRUE;
}

BOOL detectWindowMaximized() {
	return EnumWindows(CheckMaximized, 0);
}