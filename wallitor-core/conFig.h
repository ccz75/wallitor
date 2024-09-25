#pragma once
#include <string>
class conFig
{
public:
	std::wstring ffpath;
	std::wstring videoPath;
	BOOL mute;
	conFig(const char* ffpath, const char* videoPath,BOOL mute);
	conFig(const conFig& config);
	conFig();
};

