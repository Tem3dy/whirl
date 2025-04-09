#pragma once

#include <string>

#include <glad/gl.h>
#include <GLFW/glfw3.h>

#include "VideoMode.hpp"

class GuiApplication
{
public:
    GuiApplication(const VideoMode& mode);

public:
    int Launch();

private:
    VideoMode m_mode;
    GLFWwindow* m_window;
};