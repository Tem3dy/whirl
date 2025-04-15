#pragma once

#include <string>
#include <vector>
#include <cstdint>

#include "Shader.hpp"
#include "VertexArray.hpp"
#include "WhirlError.hpp"

template <typename T>
class Renderer
{
public:
    Renderer(const std::string& shaderPath, const std::vector<VertexAttribute>& layout)
    {
        try
        {
            m_array = std::make_unique<VertexArray>(layout);
            m_shader = std::make_unique<Shader>(shaderPath);
        }
        catch (WhirlError& error)
        {
            error.Context("Big bad");
            throw;
        }
    }

    virtual ~Renderer() = default;

public:
    virtual void Draw(const T& shape) = 0;
    virtual void Submit(const glm::mat4& projection) = 0;

protected:
    virtual void Configure() = 0;

protected:
    std::unique_ptr<Shader> m_shader;
    std::unique_ptr<VertexArray> m_array;
    uint32_t count;
};