#pragma once

#include <cstdint>

#include "Renderer.hpp"

struct RoundedQuad
{
    float x, y;
    float w, h;
    float radius;
    uint32_t color;
};

struct RoundedQuadVertex
{
    float x, y;
    float u, v;
    float radius;
    uint32_t color;
};

class RoundedQuadRenderer : Renderer<RoundedQuad>
{
public:
    RoundedQuadRenderer(const std::string& shaderPath, const std::vector<VertexAttribute>& layout);
    ~RoundedQuadRenderer();

public:
    void Draw(const RoundedQuad& quad) override;
    void Submit(const glm::mat4& projection) override;

protected:
    void Configure() override;
    
private:
    std::vector<RoundedQuad> m_quads;
    std::vector<RoundedQuadVertex> m_vertices;
    std::vector<uint32_t> m_indices;
};
