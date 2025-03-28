#pragma once

#include "GuiApplication.hpp"
#include <optional>

namespace Whirl
{
    std::optional<GuiApplication> CreateApplication(int width, int height, const std::string& title);
};