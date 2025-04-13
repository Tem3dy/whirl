#pragma once

#include <glad/gl.h>
#include <GLFW/glfw3.h>

#include "VideoMode.hpp"
#include "Renderer.hpp"

class GuiApplication
{
public:
    GuiApplication(const VideoMode& mode);

public:
    int Launch();

private:
    VideoMode m_mode;
    Renderer m_renderer;
    GLFWwindow* m_window;
};