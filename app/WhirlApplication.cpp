#include "Whirl.hpp"
#include <iostream>

int main()
{
    auto app = Whirl::CreateApplication(800, 600, "Whirl Application");
    if (!app)
    {
        std::cerr << "Failed to create a Whirl Application, exiting" << std::endl;
        return 1;
    }

    return app->Launch();
}