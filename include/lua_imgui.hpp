#ifndef __LUA_HPP__
#define __LUA_HPP__

#include <iostream>
#include <lua.hpp>
#include <imgui.h>
#include <memory>
#include <ostream>
#include <string>
#include <vector>

class Widgets {
public:
    virtual ~Widgets() {}
    virtual void render() = 0;
};

class DragInt : public Widgets{
public:
    DragInt(const char* label);
    ~DragInt() override {};

    int data = 0;
    std::string label = "label";

    void render() override;
};

class Button: public Widgets {
public:
    Button();
    ~Button(){}

    std::string label = "label";
    void render() override;
};

struct LuaWindow;

struct LuaWindow{
    lua_State *state = nullptr;
    std::string code = "";
    std::vector<std::unique_ptr<Widgets>> widgets = {};

    LuaWindow(std::string code);
    LuaWindow(const LuaWindow& win) = delete;
    LuaWindow(LuaWindow&& win) = delete;

    ~LuaWindow();

    void render();

    DragInt* add_drag_int(const char* label){
        auto drag = new DragInt(label);
        std::cout << "created drag int for this: " << this << std::endl;
        this->widgets.push_back(std::unique_ptr<Widgets>( (Widgets*)drag ));
        std::cout << "size: " << this->widgets.size() << std::endl;

        return drag;
    }
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

    ~LuaHandler(){
        for (auto& window: windows)
            delete window;
    }
};

#endif
