#include <lua_imgui.hpp>
#include <iostream>
#include <lualib.h>

void LuaWindow::create(){
    std::cout << "creating window with code:" << std::endl;
    std::cout << this->code << std::endl;;

    if (luaL_dofile(this->state, "./scripts/example_1_run_script.lua") == LUA_OK) {
        std::cout << "[C] Executed example_1_run_script.lua\n";
    } else {
        std::cout << "[C] Error reading script\n";
        luaL_error(this->state, "Error: %s\n", lua_tostring(this->state, -1));
    }
}

void LuaWindow::render(){
    create();
    std::cout << "new frame" << std::endl;
    ImGui::NewFrame();
    for(auto& w : widgets)
        w->render();
    ImGui::Render();
    widgets.clear();
}

LuaWindow::LuaWindow(std::string code){
    this->code = code;
    this->state = luaL_newstate();
    luaL_openlibs(this->state);
    /*
    std::cout << "drag int function creation" << std::endl;
    static auto add_drag_int = [](lua_State* state){
        std::cout << "creating drag int";
        LuaWindow *self = (LuaWindow*) lua_topointer(state, lua_upvalueindex(1));
        auto label = lua_tostring(state, 0);
        self->add_drag_int(label);
        return 0;
    };

    std::cout << "push this" << std::endl;
    void** data = (void**)lua_newuserdata(state, sizeof(void*));
    *data = (void*)this;

    std::cout << "push closure" << std::endl;
    lua_pushcclosure(this->state, add_drag_int, 1) ;
    lua_setglobal(this->state, "drag_int");
    */
}

LuaWindow::~LuaWindow(){
    lua_close(this->state);
}
