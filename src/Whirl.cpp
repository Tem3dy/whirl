#include <iostream>

#include "Whirl.hpp"

std::optional<GuiApplication> Whirl::CreateApplication(int width, int height, const std::string& title)
{
    if (title.empty())
    {
        std::cerr << "ERROR: Empty title" << std::endl;
        return std::nullopt;
    }

    if (width <= 0)
    {
        std::cerr << "ERROR: Width can't be <= 0" << std::endl;
        return std::nullopt;
    }

    if (height <= 0)
    {
        std::cerr << "ERROR: Height can't be <= 0" << std::endl;
        return std::nullopt;
    }

    return GuiApplication({width, height, title});
}