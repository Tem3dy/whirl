#pragma once

#include <string>

#include <GL/glew.h>
#include <GLFW/glfw3.h>

struct VideoMode
{
    int width;
    int height;
    std::string title;
};

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