#pragma once

#include <string>

#include <GL/glew.h>
#include <GLFW/glfw3.h>

struct VideoMode
{
    int Width;
    int Height;
    std::string Title;
};

class GuiApplication
{
public:
    GuiApplication(const VideoMode& mode);

public:
    int Launch();

private:
    VideoMode m_VideoMode;
    GLFWwindow* m_Window;
};