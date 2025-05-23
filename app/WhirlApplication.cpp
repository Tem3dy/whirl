#include <iostream>

#include "Whirl.hpp"

int main()
{
    auto app = Whirl::CreateApplication(1280, 720, "Whirl Application");
    if (!app)
    {
        std::cerr << "Failed to create a Whirl Application, exiting" << std::endl;
        return 1;
    }

    return app->Launch();
}