#include "Whirl.hpp"
#include "Logger.hpp"

std::optional<GuiApplication> Whirl::CreateApplication(int width, int height, const std::string& title)
{
    if (title.empty())
    {
        WHIRL_ERROR("Title cannot be empty");
        return std::nullopt;
    }
    
    if (width <= 0)
    {
        WHIRL_ERROR("Width cannot be <= 0");
        return std::nullopt;
    }

    if (height <= 0)
    {
        WHIRL_ERROR("Height cannot be <= 0");
        return std::nullopt;
    }

    return GuiApplication({width, height, title});
}