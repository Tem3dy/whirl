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
            error.Context("Failed to load renderer resources");
            throw;
        }
    }

    virtual ~Renderer() = default;

public:
    virtual void Submit(const T& shape) = 0;
    virtual void Draw(const glm::mat4& projection)
    {
        if (CanRender())
        {
            m_array->Bind();
            Configure();
            m_shader->Use();
            m_shader->SetMat4("u_projection", projection);
            glDrawElements(GL_TRIANGLES, m_count, GL_UNSIGNED_INT, nullptr);
            m_array->Unbind();
            m_array->GetVertexBuffer().Unbind();
            m_array->GetIndexBuffer().Unbind();
            Reset();
    
            // Make this more robust
            auto error = 0;
            while ((error = glGetError()) != GL_NO_ERROR)
            {
                WHIRL_ERROR("GL ERROR: {}", error);
            }
        }
    }

protected:
    virtual void Configure() = 0;
    virtual bool CanRender() = 0;
    virtual void Reset() = 0;

protected:
    std::unique_ptr<Shader> m_shader;
    std::unique_ptr<VertexArray> m_array;
    uint32_t m_count = 0;
};