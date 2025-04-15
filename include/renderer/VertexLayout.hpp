#pragma once

#include <cstdint>
#include <vector>
#include <initializer_list>

#include <glad/gl.h>

#include "Logger.hpp"

enum class VertexFormat
{
    // Always 4 bytes
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
    inline std::vector<VertexAttribute> New(const std::initializer_list<VertexAttribute>& attributes)
    {
        return {attributes};
    }

    inline uint32_t GetStride(const std::vector<VertexAttribute>& attributes)
    {
        int32_t stride = 0;
        for (const auto& attribute : attributes)
            stride += 4 * attribute.size;

        return stride;
    }

    inline uint32_t GetOffset(const std::vector<VertexAttribute>& attributes, int index)
    {
        if (index >= attributes.size())
        {
            // Might wanna throw here
            WHIRL_WARN("Tried to get offset of vertex attribute out of bounds: {}", index);
            return 0;
        }

        uint32_t offset = 0;
        for (int i = 0; i < index; i++)
            offset += 4 * attributes[i].size;

        return offset;
    }
};