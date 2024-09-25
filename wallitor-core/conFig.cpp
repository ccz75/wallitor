#include "pch.h"
#include "conFig.h"

conFig::conFig(const char* ffpath,const char* videoPath, BOOL mute) {
	std::string tmp =ffpath;
	this->ffpath = std::wstring(tmp.begin(), tmp.end());
	tmp = videoPath;
	this->videoPath = std::wstring(tmp.begin(), tmp.end());
	this->mute = mute;
}

conFig::conFig(const conFig& config) {
	this->ffpath = config.ffpath;
	this->mute = config.mute;
	this->videoPath = config.videoPath;
}

conFig::conFig() {
	this->ffpath = L"";
	this->mute = TRUE;
	this->videoPath = L"";
}
