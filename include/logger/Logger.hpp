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
    // Main logging functions
    template <typename... Args>
    static void Log(Level level, std::string_view msg, Args&&... args);

    template <typename... Args>
    static void Force(Level level, std::string_view msg, Args&&... args);
    
    // Level-specific logging functions
    template <typename... Args>
    static void Trace(std::string_view msg, Args&&... args);

    template <typename... Args>
    static void Debug(std::string_view msg, Args&&... args);

    template <typename... Args>
    static void Info(std::string_view msg, Args&&... args);

    template <typename... Args>
    static void Warn(std::string_view msg, Args&&... args);

    template <typename... Args>
    static void Error(std::string_view msg, Args&&... args);

    template <typename... Args>
    static void Fatal(std::string_view msg, Args&&... args);

    static void SetLevel(Level level);
    static Level GetLevel();

private:
    static Logger& GetInstance();

private:
    Level m_level;
};