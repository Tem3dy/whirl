#include <iostream>

#include "GuiApplication.hpp"

GuiApplication::GuiApplication(const VideoMode& mode)
    : m_mode(mode),
      m_window(nullptr)
{
}

int GuiApplication::Launch()
{
    // clang-format off
    glfwSetErrorCallback([](int error, const char* description) {
        std::cerr << "GLFW Error: " << error << ": " << description << std::endl;
    });
    // clang-format on

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

    // Load GLAD
    if (!gladLoadGL(glfwGetProcAddress))
    {
        std::cerr << "ERROR: Failed to initialize GLAD" << std::endl;
        glfwDestroyWindow(m_window);
        glfwTerminate();
        return 1;
    }

    std::cout << "INFO: OpenGL Version: " << glGetString(GL_VERSION) << std::endl;
    std::cout << "INFO: GLFW Version: " << glfwGetVersionString() << std::endl;

    // Set up window user pointer
    glfwSetWindowUserPointer(m_window, this);

    // Set up viewport
    glViewport(0, 0, m_mode.width, m_mode.height);
    // clang-format off
    glfwSetFramebufferSizeCallback(m_window, [](GLFWwindow* window, int width, int height) {
        glViewport(0, 0, width, height);

        // TODO: Figure this out better at some point
        auto app = static_cast<GuiApplication*>(glfwGetWindowUserPointer(window));
        if (app) 
        {
            app->m_mode.width = width;
            app->m_mode.height = height;
            app->m_renderer.Adjust(width, height);   
        }
    });
    // clang-format on

    m_renderer.Open();
    m_renderer.Adjust(m_mode.width, m_mode.height);
    while (!glfwWindowShouldClose(m_window))
    {
        glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);

        // App rendering and logic

        glfwSwapBuffers(m_window);
        glfwPollEvents();
    }

    m_renderer.Close();
    glfwDestroyWindow(m_window);
    glfwTerminate();
    return 0;
}