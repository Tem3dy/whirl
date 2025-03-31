#pragma once

#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <string>

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
    int launch();

private:
    VideoMode m_VideoMode;
    GLFWwindow* m_Window;
};