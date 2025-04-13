#pragma once

#include <iostream>
#include <string>

enum class Level
{
    // clang-format off
    OFF     = 0,
    TRACE   = 1,
    DEBUG   = 2,
    INFO    = 3,
    WARN    = 4,
    ERROR   = 5,
    FATAL   = 6,
    // clang-format on
};

class Logger
{
public:
    template <typename... Args>
    static void Log(Level level, const std::string_view msg, Args&&... args);

    template <typename... Args>
    static void Force(Level level, const std::string_view msg, Args&&... args);
    
    template <typename... Args>
    static void Trace(const std::string_view msg, Args&&... args);

    template <typename... Args>
    static void Debug(const std::string_view msg, Args&&... args);

    template <typename... Args>
    static void Info(const std::string_view msg, Args&&... args);

    template <typename... Args>
    static void Warn(const std::string_view msg, Args&&... args);

    template <typename... Args>
    static void Error(const std::string_view msg, Args&&... args);

    template <typename... Args>
    static void Fatal(const std::string_view msg, Args&&... args);

    static void SetLevel(Level level);
    static Level GetLevel();

private:
    static Logger& GetInstance();

private:
    Level m_level;
};