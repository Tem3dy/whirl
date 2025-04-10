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

    void SetBool(const std::string& name, bool value) const;
    void SetInt(const std::string& name, int value) const;
    void SetUInt(const std::string& name, unsigned int value) const;
    void SetFloat(const std::string& name, float value) const;

private:
    unsigned int m_program;
};