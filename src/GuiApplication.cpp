#include "GuiApplication.hpp"
#include <iostream>

GuiApplication::GuiApplication(const VideoMode& mode)
    : m_VideoMode(mode),
      m_Window(nullptr)
{
}

int GuiApplication::launch()
{
    if (!glfwInit())
    {
        std::cerr << "Error: Failed to initialize GLFW" << std::endl;
        return 1;
    }

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    m_Window = glfwCreateWindow(m_VideoMode.width, m_VideoMode.height, m_VideoMode.title.c_str(), 0, 0);
    if (!m_Window)
    {
        std::cerr << "Error: Failed to create a window" << std::endl;
        glfwTerminate();
        return 1;
    }

    glfwMakeContextCurrent(m_Window);
    glfwSwapInterval(1);
    if (glewInit() != GLEW_OK)
    {
        std::cerr << "Error: Failed to initialize GLEW" << std::endl;
        glfwDestroyWindow(m_Window);
        glfwTerminate();
        return 1;
    }

    while (!glfwWindowShouldClose(m_Window))
    {
        glClear(GL_COLOR_BUFFER_BIT);
        
        // App rendering and logic

        glfwSwapBuffers(m_Window);
        glfwPollEvents();
    }

    glfwDestroyWindow(m_Window);
    glfwTerminate();
    return 0;
}