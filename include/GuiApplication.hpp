#pragma once

#include "VideoMode.hpp"
#include "Renderer.hpp"

struct GLFWwindow;

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