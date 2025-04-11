#pragma once

#include <string>
#include "math/Math.hpp"

class Shader
{
public:
    Shader();
    ~Shader();

public:
    bool Load(const std::string& path);
    void Use();

    // Basic types
    bool SetBool(const std::string& name, bool value) const;
    bool SetInt(const std::string& name, int value) const;
    bool SetUInt(const std::string& name, unsigned int value) const;
    bool SetFloat(const std::string& name, float value) const;

    // Mats
    bool SetMat4(const std::string& name, const glm::mat4& matrix) const;

private:
    unsigned int m_program;
};