#pragma once

#include <cstdint>
#include <cstddef>
#include <vector>
#include <initializer_list>

#include <glad/gl.h>

#include "Logger.hpp"

// Always 4 bytes, make this more flexible later on
constexpr size_t FORMAT_SIZE = 4;

enum class VertexFormat
{
    INT,
    UINT,
    FLOAT,
};

struct VertexAttribute
{
    const uint32_t size;
    const VertexFormat format;
};

namespace VertexLayout
{
    // Factory method for semantic meaning
    inline std::vector<VertexAttribute> New(const std::initializer_list<VertexAttribute>& attributes)
    {
        return {attributes};
    }

    inline uint32_t GetStride(const std::vector<VertexAttribute>& attributes) noexcept
    {
        int32_t stride = 0;
        for (const auto& attribute : attributes)
            stride += FORMAT_SIZE * attribute.size;

        return stride;
    }

    inline uint32_t GetOffset(const std::vector<VertexAttribute>& attributes, int index) noexcept
    {
        if (index >= attributes.size())
        {
            // Might wanna throw here
            WHIRL_WARN("Tried to get offset of vertex attribute out of bounds: {}", index);
            return 0;
        }

        uint32_t offset = 0;
        for (int i = 0; i < index; i++)
            offset += FORMAT_SIZE * attributes[i].size;

        return offset;
    }

    inline uint32_t GetType(VertexFormat format) noexcept
    {
        switch (format)
        {
            case VertexFormat::FLOAT:
                return GL_FLOAT;
            case VertexFormat::INT:
                return GL_INT;
            case VertexFormat::UINT:
                return GL_UNSIGNED_INT;

            default:
                return GL_FLOAT;
        }
    }
};