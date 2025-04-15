#include <glad/gl.h>
#include <GLFW/glfw3.h>

#include "GuiApplication.hpp"
#include "Logger.hpp"

GuiApplication::GuiApplication(const VideoMode& mode)
    : m_mode(mode),
      m_window(nullptr)
{
}

int GuiApplication::Launch()
{
    // clang-format off
    glfwSetErrorCallback([](int error, const char* description) {
        WHIRL_ERROR("GLFW: {} -> {}", error, description);
    });
    // clang-format on

    if (!glfwInit())
    {
        WHIRL_FATAL("Failed to initialize GLFW");
        return 1;
    }

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 5);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    m_window = glfwCreateWindow(m_mode.width, m_mode.height, m_mode.title.c_str(), 0, 0);
    if (!m_window)
    {
        WHIRL_FATAL("Failed to create a window");
        glfwTerminate();
        return 1;
    }

    // TODO: Make this adjustable in the future
    glfwSetWindowSizeLimits(m_window, 200, 100, GLFW_DONT_CARE, GLFW_DONT_CARE);
    glfwMakeContextCurrent(m_window);

    // Load GLAD
    if (!gladLoadGL(glfwGetProcAddress))
    {
        WHIRL_FATAL("Failed to initialize GLAD");
        glfwDestroyWindow(m_window);
        glfwTerminate();
        return 1;
    }

    WHIRL_INFO("OpenGL Version: {}", reinterpret_cast<const char*>(glGetString(GL_VERSION)));
    WHIRL_INFO("GLFW Version: {}", glfwGetVersionString());
    // Set up window user pointer
    glfwSetWindowUserPointer(m_window, this);

    try
    {
        m_renderer = std::make_unique<Renderer>();
    }
    catch (const std::runtime_error& error)
    {
        WHIRL_FATAL("Failed to create a renderer");
        WHIRL_FATAL("{}", error.what());
        return 1;
    }

    // Set up viewport
    glViewport(0, 0, m_mode.width, m_mode.height);
    // clang-format off
    glfwSetFramebufferSizeCallback(m_window, [](GLFWwindow* window, int width, int height) {
        glViewport(0, 0, width, height);

        // TODO: Figure this out better at some point
        auto* app = static_cast<GuiApplication*>(glfwGetWindowUserPointer(window));
        if (app) 
        {
            app->m_mode.width = width;
            app->m_mode.height = height;
            app->m_renderer->Adjust(width, height);   
        }
    });
    // clang-format on

    m_renderer->Adjust(m_mode.width, m_mode.height);
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