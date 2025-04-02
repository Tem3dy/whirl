#include "renderer/Shader.hpp"
#include <iostream>
#include <fstream>

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
                std::cout << "Read fragment shader source, reading vertex shader source..." << std::endl;
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
                std::cout << "Read vertex shader source, reading fragment shader source..." << std::endl;
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
        std::cerr << "Missing vertex or fragment shader code in: " << path << std::endl;
        return false;
    }

    std::cout << "Vertex and fragment shaders loaded successfully from: " << path << std::endl;
    std::cout << "Vertex shader (first 100 chars): \n" << vShaderCode.substr(0, 100) << std::endl;
    std::cout << "Fragment shader (first 100 chars): \n" << fShaderCode.substr(0, 100) << std::endl;
    shaderFile.close();

    // Pass shaders to OpenGL
    return true;
}

void Shader::Use()
{
    // Use shader
}
