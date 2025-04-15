#pragma once

#include <string>

#include "Math.hpp"

class Shader
{
public:
    Shader(const std::string& path);
    ~Shader();

public:
    void Use();

    // Basic types
    bool SetBool(const std::string& name, bool value) const;
    bool SetInt(const std::string& name, int value) const;
    bool SetUInt(const std::string& name, unsigned int value) const;
    bool SetFloat(const std::string& name, float value) const;

    // Mats
    bool SetMat4(const std::string& name, const glm::mat4& matrix) const;

private:
    unsigned int m_program = 0;
};