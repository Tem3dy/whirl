#pragma once

#include <optional>

#include "GuiApplication.hpp"

namespace Whirl
{
    std::optional<GuiApplication> CreateApplication(int width, int height, const std::string& title);
};