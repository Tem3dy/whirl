#pragma once

#include <cstdint>

#include "Renderer.hpp"

struct Quad
{
    float x, y;
    float w, h;
    uint32_t color;
};

struct QuadVertex
{
    float x, y;
    uint32_t color;
};

class QuadRenderer : public Renderer<Quad>
{
public:
    QuadRenderer(const std::string& shaderPath, const std::vector<VertexAttribute>& layout);
    ~QuadRenderer();

public:
    void Draw(const Quad& quad) override;
    void Submit(const glm::mat4& projection) override;

protected:
    void Configure() override;

private:
    std::vector<Quad> m_quads;
    std::vector<QuadVertex> m_vertices;
    std::vector<uint32_t> m_indices;
};