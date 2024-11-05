#ifndef __LUA_WIDGETS_HPP__
#define __LUA_WIDGETS_HPP__

#include "lua_imgui.hpp"
#include <lua.hpp>

typedef struct luaL_Reg sluaL_Reg;

class DragInt : public Widget{
public:
    DragInt(const char* label);
    ~DragInt() override {};

    int data = 0;
    std::string label = "label";

    void render() override;

    static int lua_new(lua_State* state);
    static int lua_get(lua_State* state);

    static void load_lib(LuaWindow* window, lua_State* state);
    static const sluaL_Reg libs[];
};

class Button: public Widget{
public:
    Button(const char* label);
    ~Button() override {};

    bool pressed = false;
    std::string label = "label";

    void render() override;

    static int lua_new(lua_State* state);
    static int lua_get(lua_State* state);
    static void load_lib(LuaWindow* window, lua_State* state);
    static const sluaL_Reg libs[];
};

class Text: public Widget{
public:
    Text(const char* label);
    ~Text() override {};

    std::string text = "label";

    void render() override;

    static int lua_new(lua_State* state);
    static int lua_get(lua_State* state);
    static int lua_set(lua_State* state);
    static void load_lib(LuaWindow* window, lua_State* state);
    static const sluaL_Reg libs[];
};

class ReloadButton: public Button{
public:
    ReloadButton() : Button("reload"){}
    ~ReloadButton() override {};

    void render() override;

    static bool reload;
    static int lua_new(lua_State* state);
    static void load_lib(LuaWindow* window, lua_State* state);
    static const sluaL_Reg libs[];
};

#endif
