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
    float w, h;
    float u, v;
    float radius;
    uint32_t color;
};

class RoundedQuadRenderer : public Renderer<RoundedQuad>
{
public:
    RoundedQuadRenderer();
    ~RoundedQuadRenderer();

public:
    void Submit(const RoundedQuad& quad) override;

protected:
    void Configure() override;
    bool CanRender() override;
    void Reset() override;

private:
    std::vector<RoundedQuad> m_quads;
    std::vector<RoundedQuadVertex> m_vertices;
    std::vector<uint32_t> m_indices;
};
