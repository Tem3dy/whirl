#pragma once

#include <string>

class Shader
{
public:
    bool Load(const std::string& path);
    void Use();
};