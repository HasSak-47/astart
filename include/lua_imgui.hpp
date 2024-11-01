#ifndef __LUA_HPP__
#define __LUA_HPP__

#include <lua.hpp>
#include <imgui.h>
#include <memory>
#include <string>
#include <vector>

class Widgets {
public:
    virtual ~Widgets(){}
    virtual bool render() = 0;
};

class DragInt : Widgets{
public:
    DragInt(std::string label): label(label){}
    DragInt(const char* label): label(label){}
    int data = 0;
    std::string label = "label";
    bool render() override {
        return ImGui::DragInt(this->label.c_str(), &this->data);
    }
};

struct LuaWindow;

struct LuaWindow{
    lua_State *state;
    std::string code;
    std::vector<std::shared_ptr<Widgets>> widgets = {};

    LuaWindow(std::string code);
    ~LuaWindow();

    void create();
    void render();
};

struct LuaHandler{
    std::vector<LuaWindow> windows = {};

    LuaHandler(std::vector<std::string> scripts){
        for (auto script : scripts) 
            windows.push_back(script);
    }

    void run(){
        for (auto& window: windows)
            window.render();
    }

    ~LuaHandler(){
    }
};

#endif
