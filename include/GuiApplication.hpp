#pragma once

#include <memory>

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
    GLFWwindow* m_window;
    std::unique_ptr<Renderer> m_renderer;
};