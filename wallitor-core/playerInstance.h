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
	BOOL generate();
	void exit();
};

