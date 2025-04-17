#pragma once

#include <string>

#include "Math.hpp"

class Shader
{
public:
    explicit Shader(const std::string& path);
    ~Shader();

public:
    void Use() const;

    // Basic types
    bool SetBool(const std::string& name, bool value);
    bool SetInt(const std::string& name, int value);
    bool SetUInt(const std::string& name, unsigned int value);
    bool SetFloat(const std::string& name, float value);

    // Mats
    bool SetMat4(const std::string& name, const glm::mat4& matrix);

private:
    unsigned int m_program = 0;
};