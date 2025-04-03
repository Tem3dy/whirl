#include <iostream>

#include "GuiApplication.hpp"
#include "renderer/Shader.hpp"

GuiApplication::GuiApplication(const VideoMode& mode)
    : m_VideoMode(mode),
      m_Window(nullptr)
{
}

static void HandleResize(GLFWwindow* window, int width, int height)
{
    glViewport(0, 0, width, height);
}

int GuiApplication::Launch()
{
    if (!glfwInit())
    {
        std::cerr << "Error: Failed to initialize GLFW" << std::endl;
        return 1;
    }

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 5);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    m_Window = glfwCreateWindow(m_VideoMode.Width, m_VideoMode.Height, m_VideoMode.Title.c_str(), 0, 0);
    if (!m_Window)
    {
        std::cerr << "Error: Failed to create a window" << std::endl;
        glfwTerminate();
        return 1;
    }

    glfwMakeContextCurrent(m_Window);
    if (glewInit() != GLEW_OK)
    {
        std::cerr << "Error: Failed to initialize GLEW" << std::endl;
        glfwDestroyWindow(m_Window);
        glfwTerminate();
        return 1;
    }

    glfwSetFramebufferSizeCallback(m_Window, HandleResize);
    glViewport(0, 0, m_VideoMode.Width, m_VideoMode.Height);
    while (!glfwWindowShouldClose(m_Window))
    {
        glClearColor(0.175f, 0.25f, 0.3f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);

        // App rendering and logic

        glfwSwapBuffers(m_Window);
        glfwPollEvents();
    }

    glfwDestroyWindow(m_Window);
    glfwTerminate();
    return 0;
}