#pragma once

#include <string>

class Shader
{
public:
    Shader();
    ~Shader();

public:
    bool Load(const std::string& path);
    void Use();

    bool SetBool(const std::string& name, bool value) const;
    bool SetInt(const std::string& name, int value) const;
    bool SetUInt(const std::string& name, unsigned int value) const;
    bool SetFloat(const std::string& name, float value) const;

private:
    unsigned int m_program;
};