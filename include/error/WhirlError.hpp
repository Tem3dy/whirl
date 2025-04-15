#pragma once

#include <exception>
#include <string>
#include <vector>

#include "fmt/format.h"

class WhirlError : public std::exception
{
public:
    explicit WhirlError(const std::string& message)
    {
        m_context.reserve(4);
        m_context.push_back(message);
    }

    void Context(const std::string& error)
    {
        m_context.push_back(error);
    }

    template <typename... Args>
    void Context(const std::string& error, Args&&... args)
    {
        m_context.push_back(fmt::format(error, std::forward<Args>(args)...));
    }

    std::vector<std::string> Get() const
    {
        return m_context;
    }

    // Do not use this function
    const char* what() const noexcept override
    {
        if (m_context.size() > 0)
        {
            return m_context[0].c_str();
        }

        return "Whirl Error";
    }

private:
    std::vector<std::string> m_context;
};