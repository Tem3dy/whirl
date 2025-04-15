#include "Whirl.hpp"
#include "Logger.hpp"

std::optional<GuiApplication> Whirl::CreateApplication(int width, int height, const std::string& title)
{
    if (title.empty())
    {
        WHIRL_FATAL("Title cannot be empty");
        return std::nullopt;
    }
    
    if (width <= 0)
    {
        WHIRL_FATAL("Width cannot be <= 0");
        return std::nullopt;
    }

    if (height <= 0)
    {
        WHIRL_FATAL("Height cannot be <= 0");
        return std::nullopt;
    }

    return GuiApplication({width, height, title});
}