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

static void HandleError(int error, const char* message)
{
    std::cerr << "ERROR: " << message << std::endl;
}

int GuiApplication::Launch()
{
    glfwSetErrorCallback(HandleError);
    if (!glfwInit())
    {
        std::cerr << "ERROR: Failed to initialize GLFW" << std::endl;
        return 1;
    }

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 5);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    m_window = glfwCreateWindow(m_mode.width, m_mode.height, m_mode.title.c_str(), 0, 0);
    if (!m_window)
    {
        std::cerr << "ERROR: Failed to create a window" << std::endl;
        glfwTerminate();
        return 1;
    }

    // TODO: Make this adjustable in the future
    glfwSetWindowSizeLimits(m_window, 200, 100, GLFW_DONT_CARE, GLFW_DONT_CARE);
    glfwMakeContextCurrent(m_window);
    if (!gladLoadGL(glfwGetProcAddress))
    {
        std::cerr << "ERROR: Failed to initialize GLAD" << std::endl;
        glfwDestroyWindow(m_window);
        glfwTerminate();
        return 1;
    }

    std::cout << "INFO: OpenGL Version: " << glGetString(GL_VERSION) << std::endl;
    std::cout << "INFO: GLFW Version: " << glfwGetVersionString() << std::endl;

    glfwSetFramebufferSizeCallback(m_window, HandleResize);
    glViewport(0, 0, m_mode.width, m_mode.height);
    while (!glfwWindowShouldClose(m_window))
    {
        glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);

        // App rendering and logic

        glfwSwapBuffers(m_window);
        glfwPollEvents();
    }

    glfwDestroyWindow(m_window);
    glfwTerminate();
    return 0;
}