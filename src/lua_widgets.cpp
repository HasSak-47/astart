#include <iostream>
#include <lua.hpp>
#include <lua_widgets.hpp>


DragInt::DragInt(const char* label){
    if(label == nullptr)
        this->label = "NULL";
    else
        this->label = label;
}


void DragInt::render() {
    ImGui::DragInt(this->label.c_str(), &this->data);
}

int DragInt::lua_new(lua_State* state){
    LuaWindow *self = (LuaWindow*) lua_topointer(state, lua_upvalueindex(1));
    auto label = lua_tostring(state, -1);
    auto drag_int = new DragInt(label);
    std::cout << "Button::lua_new: " << drag_int << std::endl;
    self->add_widget(drag_int);
    lua_pushlightuserdata(state, drag_int);

    return 1;
}

int DragInt::lua_get(lua_State* state){
    //std::cout << "DragInt::lua_get" << std::endl;
    DragInt *di= (DragInt*) lua_topointer(state, -1);
    lua_pushnumber(state, di->data);

    return 1;
}

void DragInt::load_lib(LuaWindow* window, lua_State* state){
    lua_createtable(state, 0, 3);
    lua_pushlightuserdata(state, window);
    luaL_setfuncs(state, DragInt::libs, 1);

    lua_setglobal(state, "DragInt");
}

const sluaL_Reg DragInt::libs[] = {
    {"new", DragInt::lua_new},
    {"get", DragInt::lua_get},
    {NULL, NULL},
};

Button::Button(const char* label) : label(label){ }

void Button::render(){
    this->pressed = ImGui::Button(this->label.c_str());
    return;
}

int Button::lua_new(lua_State* state){
    LuaWindow *self = (LuaWindow*) lua_topointer(state, lua_upvalueindex(1));
    auto label = lua_tostring(state, -1);
    auto button = new Button(label);
    std::cout << "Button::lua_new: " << button << std::endl;
    self->add_widget(button);
    lua_pushlightuserdata(state, button);

    return 1;
}

int Button::lua_get(lua_State* state){
    // std::cout << "Button::lua_get" << std::endl;
    Button *di = (Button*) lua_topointer(state, -1);
    lua_pushboolean(state, di->pressed);

    return 1;
}

void Button::load_lib(LuaWindow* window, lua_State* state){
    lua_createtable(state, 0, 3);
    lua_pushlightuserdata(state, window);
    luaL_setfuncs(state, Button::libs, 1);

    lua_setglobal(state, "Button");
}

const sluaL_Reg Button::libs[] = {
    {"new", Button::lua_new},
    {"get", Button::lua_get},
    {NULL, NULL},
};

bool ReloadButton::reload = false;

void ReloadButton::render(){
    ReloadButton::reload = ImGui::Button(this->label.c_str());
}

int ReloadButton::lua_new(lua_State* state){
    LuaWindow *self = (LuaWindow*) lua_topointer(state, lua_upvalueindex(1));
    auto label = lua_tostring(state, -1);
    auto button = new ReloadButton();
    std::cout << "ReloadButton::lua_new: " << button << std::endl;
    self->add_widget(button);
    lua_pushlightuserdata(state, button);

    return 1;
}

void ReloadButton::load_lib(LuaWindow* window, lua_State* state){
    lua_createtable(state, 0, 3);
    lua_pushlightuserdata(state, window);
    luaL_setfuncs(state, ReloadButton::libs, 1);

    lua_setglobal(state, "ReloadButton");
}
const sluaL_Reg ReloadButton::libs[] = {
    {"new", ReloadButton::lua_new},
    {NULL, NULL},
};
