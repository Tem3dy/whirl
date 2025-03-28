#include "Whirl.hpp"
#include <iostream>

std::optional<GuiApplication> Whirl::CreateApplication(int width, int height, const std::string& title)
{
    if (title.empty())
    {
        std::cerr << "Error: Empty title" << std::endl;
        return std::nullopt;
    }

    if (width <= 0)
    {
        std::cerr << "Error: Width can't be <= 0" << std::endl;
        return std::nullopt;
    }

    if (height <= 0)
    {
        std::cerr << "Error: Height can't be <= 0" << std::endl;
        return std::nullopt;
    }

    return GuiApplication({width, height, title});
}