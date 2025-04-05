#include <iostream>

#include "GuiApplication.hpp"

GuiApplication::GuiApplication(const VideoMode& mode)
    : m_mode(mode),
      m_window(nullptr)
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

    m_window = glfwCreateWindow(m_mode.width, m_mode.height, m_mode.title.c_str(), 0, 0);
    if (!m_window)
    {
        std::cerr << "Error: Failed to create a window" << std::endl;
        glfwTerminate();
        return 1;
    }

    glfwMakeContextCurrent(m_window);
    if (glewInit() != GLEW_OK)
    {
        std::cerr << "Error: Failed to initialize GLEW" << std::endl;
        glfwDestroyWindow(m_window);
        glfwTerminate();
        return 1;
    }

    glfwSetFramebufferSizeCallback(m_window, HandleResize);
    glViewport(0, 0, m_mode.width, m_mode.height);
    while (!glfwWindowShouldClose(m_window))
    {
        glClearColor(0.175f, 0.25f, 0.3f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);

        // App rendering and logic

        glfwSwapBuffers(m_window);
        glfwPollEvents();
    }

    glfwDestroyWindow(m_window);
    glfwTerminate();
    return 0;
}