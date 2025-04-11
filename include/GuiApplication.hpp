#pragma once

#include <string>

#include <glad/gl.h>
#include <GLFW/glfw3.h>

#include "VideoMode.hpp"
#include "renderer/Renderer.hpp"

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