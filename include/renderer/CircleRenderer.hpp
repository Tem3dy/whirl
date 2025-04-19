#pragma once

#include <cstdint>

#include "Renderer.hpp"

struct Circle
{
    float x, y;
    float radius;
    uint32_t color;
};

struct CircleVertex
{
    float x, y;
    float w, h;
    float u, v;
    float radius;
    uint32_t color;
};

class CircleRenderer : public Renderer<Circle>
{
public:
    CircleRenderer();
    ~CircleRenderer();

public:
    void Submit(const Circle& circle) override;

protected:
    void Configure() override;
    bool CanRender() override;
    void Reset() override;

private:
    std::vector<Circle> m_circles;
    std::vector<CircleVertex> m_vertices;
    std::vector<uint32_t> m_indices;
};