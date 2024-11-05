#ifndef __LUA_HPP__
#define __LUA_HPP__

#include <lua.hpp>
#include <imgui.h>
#include <memory>
#include <string>
#include <vector>

class Widget {
public:
    virtual ~Widget() {}
    virtual void render() = 0;
};

struct LuaWindow;

struct LuaWindow{
    lua_State *state = nullptr;
    std::string code = "";
    std::vector<std::unique_ptr<Widget>> widgets = {};

    LuaWindow(std::string code);
    LuaWindow(const LuaWindow& win) = delete;
    LuaWindow(LuaWindow&& win) = delete;

    ~LuaWindow();

    void setup();
    void loop();
    void render();

    void add_widget(Widget* widget);
};

struct LuaHandler{
    std::vector<LuaWindow* > windows = {};

    LuaHandler(std::vector<std::string> scripts){
        for (auto script : scripts) {
            this->windows.push_back(new LuaWindow(script));
        }
    }

    void run(){
        for (auto& window: windows)
            window->render();
    }

    void reload(){
        for (auto& window: windows)
            window->setup();
    }

    ~LuaHandler(){
        for (auto& window: windows)
            delete window;
    }
};

#endif
