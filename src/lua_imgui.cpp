#include <lua_imgui.hpp>
#include <iostream>
#include <ostream>


DragInt::DragInt(const char* label){
    std::cout << "Label: " << label << std::endl;
    if(label == nullptr){
        this->label = "NULL";
    }
    else{
        this->label = label;
    }
}


bool DragInt::render() {
    return ImGui::DragInt(this->label.c_str(), &this->data);
}

void LuaWindow::render(){
    lua_getglobal(this->state, "Loop");
    lua_pcall(this->state, 0, 0, 0);

    ImGui::NewFrame();
    // std::cout << "total widgets: " << widgets.size() << std::endl;
    for(auto& w : widgets){
        // std::cout << "widget: " << w.get() << std::endl;
        w->render();
    }
    ImGui::Render();
}

int lua_new_drag_int(lua_State* state){
    LuaWindow *self = (LuaWindow*) lua_topointer(state, lua_upvalueindex(1));
    auto label = lua_tostring(state, -1);
    auto drag = self->add_drag_int(label);

    lua_pushlightuserdata(state, drag);
    return 1;
};

int lua_get_drag_int(lua_State* state){
    DragInt *di= (DragInt*) lua_topointer(state, -1);
    lua_pushnumber(state, di->data);
    return 1;
};

LuaWindow::LuaWindow(std::string code):
    code(code)
{
    std::cout << "LuaWindow created: " << this << std::endl;
    this->state = luaL_newstate();
    luaL_openlibs(this->state);

    const struct luaL_Reg drag_int_lib[] = {
      {"new", lua_new_drag_int},
      {"get", lua_get_drag_int},
      {NULL, NULL}
    };

    //luaL_newlib(this->state, drag_int_lib);

    lua_createtable(this->state, 0, 3);
    lua_pushlightuserdata(state, this);
    luaL_setfuncs(this->state, drag_int_lib, 1);

    lua_setglobal(this->state, "DragInt");

    luaL_dostring(this->state, this->code.c_str());
    lua_getglobal(this->state, "Setup");
    lua_pcall(this->state, 0, 0, 0);

    std::cout << "end of constructor: " << this << std::endl;
}

LuaWindow::~LuaWindow(){
    if(this->state != nullptr){
        lua_close(this->state);
    }
}
