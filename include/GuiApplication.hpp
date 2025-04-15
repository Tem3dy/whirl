#pragma once

#include <memory>

#include "VideoMode.hpp"
#include "GuiRenderer.hpp"

// Could forward declare VideoMode and GuiRenderer too (probably not necessary)
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
    std::unique_ptr<GuiRenderer> m_renderer;
};