#pragma once

#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <string>

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
    int launch();

private:
    VideoMode m_VideoMode;
    GLFWwindow* m_Window;
};