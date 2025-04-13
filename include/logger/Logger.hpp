#pragma once

#include <iostream>
#include <string>
#include <array>
#include <chrono>

#include <fmt/core.h>
#include <fmt/chrono.h>

// clang-format off
#define WHIRL_TRACE(...) Logger::Trace(__VA_ARGS__)
#define WHIRL_DEBUG(...) Logger::Debug(__VA_ARGS__)
#define WHIRL_INFO(...)  Logger::Info(__VA_ARGS__)
#define WHIRL_WARN(...)  Logger::Warn(__VA_ARGS__)
#define WHIRL_ERROR(...) Logger::Error(__VA_ARGS__)
#define WHIRL_FATAL(...) Logger::Fatal(__VA_ARGS__)
#define WHIRL_FORCE(...) Logger::Force(__VA_ARGS__)
#define WHIRL_LOG(...)   Logger::Log(__VA_ARGS__)
// clang-format on

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
private:
    Logger(Level level)
        : m_level(level)
    {
    }

public:
    // Main logging functions
    template <typename... Args>
    static void Log(Level level, std::string_view msg, Args&&... args)
    {
        if (level == Level::OFF || GetLevel() > level)
        {
            // Ignore for now
            return;
        }

        const std::string head = fmt::format("[{}, {}]: ", GetHeadTime(), GetHeadLevel(level));
        const std::string body = fmt::format(fmt::runtime(msg), std::forward<Args>(args)...);
        if (level >= Level::ERROR)
        {
            std::cerr << head << body << std::endl;
            // Maybe flush and terminate if level = FATAL
        }
        else
        {
            std::cout << head << body << std::endl;
        }
    }

    template <typename... Args>
    static void Force(Level level, std::string_view msg, Args&&... args)
    {
        const auto temp = GetLevel();
        SetLevel(level);
        Log(level, msg, args...);
        SetLevel(temp);
    }

    // Level-specific logging functions
    template <typename... Args>
    static void Trace(std::string_view msg, Args&&... args)
    {
        Logger::Log(Level::TRACE, msg, std::forward<Args>(args)...);
    }

    template <typename... Args>
    static void Debug(std::string_view msg, Args&&... args)
    {
        Logger::Log(Level::DEBUG, msg, std::forward<Args>(args)...);
    }

    template <typename... Args>
    static void Info(std::string_view msg, Args&&... args)
    {
        Logger::Log(Level::INFO, msg, std::forward<Args>(args)...);
    }

    template <typename... Args>
    static void Warn(std::string_view msg, Args&&... args)
    {
        Logger::Log(Level::WARN, msg, std::forward<Args>(args)...);
    }

    template <typename... Args>
    static void Error(std::string_view msg, Args&&... args)
    {
        Logger::Log(Level::ERROR, msg, std::forward<Args>(args)...);
    }

    template <typename... Args>
    static void Fatal(std::string_view msg, Args&&... args)
    {
        Logger::Log(Level::FATAL, msg, std::forward<Args>(args)...);
    }

    static void SetLevel(Level level)
    {
        if (level < Level::OFF || level > Level::FATAL)
        {
            WHIRL_FORCE(Level::ERROR, "Invalid log level: {}", static_cast<int>(level));
            return;
        }

        GetInstance().m_level = level;
    }

    static Level GetLevel()
    {
        return GetInstance().m_level;
    }

private:
    static Logger& GetInstance()
    {
        static Logger instance(Level::TRACE);
        return instance;
    }

    static std::string_view GetHeadLevel(Level level)
    {
        static constexpr std::array<std::string_view, 7> levels = {
            "OFF",
            "TRACE",
            "DEBUG",
            "INFO",
            "WARN",
            "ERROR",
            "FATAL",
        };

        return levels[static_cast<int>(level)];
    }

    static std::string GetHeadTime()
    {
        using namespace std::chrono;
        const auto now = system_clock::now();

        // clang-format off
        return fmt::format("{:%H:%M:%S}.{:03d}",
            fmt::localtime(system_clock::to_time_t(now)),  // Thread-safe time conversion
            duration_cast<milliseconds>(now.time_since_epoch()).count() % 1000
        );
        // clang-format on
    }

private:
    Level m_level;
};