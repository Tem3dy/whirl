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
    QuadRenderer();
    ~QuadRenderer();

public:
    void Submit(const Quad& quad) override;

protected:
    void Configure() override;
    bool CanRender() override;
    void Reset() override;

private:
    std::vector<Quad> m_quads;
    std::vector<QuadVertex> m_vertices;
    std::vector<uint32_t> m_indices;
};