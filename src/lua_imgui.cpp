#include "imgui.h"
#include "lua_widgets.hpp"
#include <iostream>
#include <lua_imgui.hpp>

void LuaWindow::render(){
    //std::cout << "loop start" << std::endl;

    this->loop();
    //std::cout << "loop end" << std::endl;
    //std::cout << "widgets: " << widgets.size() << std::endl;
    ImGui::NewFrame();
    // //std::cout << "total widgets: " << widgets.size() << std::endl;
    for(auto& w : widgets){
        //std::cout << "widget: " << w.get() << std::endl;
        w->render();
    }
    ImGui::Render();
}

LuaWindow::LuaWindow(std::string code):
    code(code)
{
    this->state = luaL_newstate();
    luaL_openlibs(this->state);

    //luaL_newlib(this->state, drag_int_lib);

    DragInt::load_lib(this, this->state);
    Button::load_lib(this, this->state);
    ReloadButton::load_lib(this, this->state);
    //std::cout << "loading code:" << std::endl;
    //std::cout << this->code << std::endl;
    luaL_dostring(this->state, this->code.c_str());

    //std::cout << "runnig setup" << std::endl;
    this->setup();
    //std::cout << "stopped setup" << std::endl;
}

void LuaWindow::add_widget(Widget* widget){
    //std::cout  << "added widget: " << (void*)widget << std::endl;
    this->widgets.push_back(std::unique_ptr<Widget>( widget ));
}

void LuaWindow::setup(){
    this->widgets.clear();
    lua_getglobal(this->state, "Setup");
    lua_pcall(this->state, 0, 0, 0);
}

void LuaWindow::loop(){
    lua_getglobal(this->state, "Loop");
    lua_pcall(this->state, 0, 0, 0);
}

LuaWindow::~LuaWindow(){
    if(this->state != nullptr){
        lua_close(this->state);
    }
}
