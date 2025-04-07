#include <iostream>
#include <fstream>
#include <vector>

#include <glad/gl.h>

#include "renderer/Shader.hpp"

static bool Compile(unsigned int shader, const char* source)
{
    std::cout << "Compiling shader..." << std::endl;
    glShaderSource(shader, 1, &source, nullptr);
    glCompileShader(shader);

    int result;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &result);
    if (!result)
    {
        int size;
        glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &size);

        std::vector<char> log(size);
        glGetShaderInfoLog(shader, size, nullptr, log.data());

        std::cerr << "Failed to compile shader: \n" << log.data() << std::endl;
        return false;
    }

    return true;
}

static bool Link(unsigned int program, unsigned int vShader, unsigned int fShader)
{
    std::cout << "Linking shaders..." << std::endl;
    glAttachShader(program, vShader);
    glAttachShader(program, fShader);
    glLinkProgram(program);
    
    int linkResult;
    glGetProgramiv(program, GL_LINK_STATUS, &linkResult);
    if (!linkResult)
    {
        int size;
        glGetProgramiv(program, GL_INFO_LOG_LENGTH, &size);
        
        std::vector<char> log(size);
        glGetProgramInfoLog(program, size, nullptr, log.data());

        std::cerr << "Failed to link shader program: \n" << log.data() << std::endl;
        return false;
    }

    glValidateProgram(program);
    glDeleteShader(vShader);
    glDeleteShader(fShader);
    return true;
}

bool Shader::Load(const std::string& path)
{
    std::cout << "Reading shader file: " << path << std::endl;
    std::ifstream shaderFile(path);
    if (!shaderFile)
    {
        std::cerr << "Failed to open a shader: " << path << std::endl;
        return false;
    }

    std::string line;
    std::string vShaderCode;
    std::string fShaderCode;

    bool readingVertex = false;
    bool readingFragment = false;
    while (std::getline(shaderFile, line))
    {
        if (line.find("#shader") != std::string::npos)
        {
            if (line.find("vertex") == std::string::npos && line.find("fragment") == std::string::npos)
            {
                std::cerr << "Unknown shader tag found in: " << path << std::endl;
                std::cerr << "-> " << line << " <-" << std::endl;
                return false;
            }
        }

        if (line.find("#shader vertex") != std::string::npos)
        {
            if (readingFragment)
            {
                std::cout << "Reading vertex shader source..." << std::endl;
                readingFragment = false;
                readingVertex = true;
                continue;
            }

            if (readingVertex)
            {
                std::cerr << "Unexpected shader tag found in: " << path << std::endl;
                std::cerr << "-> " << line << " <- " << std::endl;
                return false;
            }

            std::cout << "Reading vertex shader source..." << std::endl;
            readingVertex = true;
            continue;
        }

        if (line.find("#shader fragment") != std::string::npos)
        {
            if (readingVertex)
            {
                std::cout << "Reading fragment shader source..." << std::endl;
                readingVertex = false;
                readingFragment = true;
                continue;
            }

            if (readingFragment)
            {
                std::cerr << "Unexpected shader tag found in: " << path << std::endl;
                std::cerr << "-> " << line << " <- " << std::endl;
                return false;
            }

            std::cout << "Reading fragment shader source..." << std::endl;
            readingFragment = true;
            continue;
        }

        if (readingVertex)
        {
            if (!line.empty())
                vShaderCode.append(line + "\n");

            continue;
        }

        if (readingFragment)
        {
            if (!line.empty())
                fShaderCode.append(line + "\n");
            
            continue;
        }
    }

    if (vShaderCode.empty() || fShaderCode.empty())
    {
        std::cerr << "Missing shader code in: " << path << std::endl;
        return false;
    }

    std::cout << "Shader source loaded successfully from file: " << path << std::endl;
    shaderFile.close();

    // Pass shaders to OpenGL
    m_program = glCreateProgram();
    unsigned int vShader = glCreateShader(GL_VERTEX_SHADER);
    unsigned int fShader = glCreateShader(GL_FRAGMENT_SHADER);

    if (!Compile(vShader, vShaderCode.c_str()))
        return false;
    if (!Compile(fShader, fShaderCode.c_str()))
        return false;

    if (!Link(m_program, vShader, fShader))
        return false;
    
    std::cout << "Shader constructed successfully from file: " << path << std::endl;
    return true;
}

void Shader::Use()
{
    if (m_program == 0)
    {
        std::cerr << "Error: Attempted to use an invalid shader" << std::endl;
        return;
    }

    // Use shader
    glUseProgram(m_program);
}

Shader::Shader()
{
    m_program = 0;
}

Shader::~Shader()
{
    if (m_program != 0)
        glDeleteProgram(m_program);
}
