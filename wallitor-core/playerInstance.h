#pragma once
#include <string>
#include "conFig.h"
class playerInstance
{
private:
	HWND hFfplay;
	conFig config;
public:
	playerInstance(const conFig& config);
	BOOL showWindow(LPCWSTR lpParameter);
	void generate();
	void exit();
};

BOOL set_as_wallpaper(const char* window_title);