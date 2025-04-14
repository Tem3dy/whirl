#pragma once

#include <cstdint>

#include "DrawMode.hpp"

class IndexBuffer
{
public:
    IndexBuffer();
    ~IndexBuffer();

    // Move
    IndexBuffer(IndexBuffer&& other) noexcept;
    IndexBuffer& operator=(IndexBuffer&& other) noexcept;

    // Copy
    IndexBuffer(const IndexBuffer&) = delete;
    IndexBuffer& operator=(const IndexBuffer&) = delete;

public:
    void Data(const void* data, uint32_t size, DrawMode mode);
    void Bind() const;
    void Unbind() const;

    uint32_t GetBuffer() const;

private:
    uint32_t m_buffer = 0;
};