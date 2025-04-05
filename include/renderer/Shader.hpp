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

private:
    unsigned int m_program;
};