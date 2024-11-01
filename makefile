SRC_DIR := src
OBJ_DIR := .cache
OUT := astart

SRCS := $(shell find src -name \*.cpp)
OBJS := $(patsubst $(SRC_DIR)/%.cpp,$(OBJ_DIR)/%.o,$(SRCS))

IMGUI_SRC := $(wildcard imgui/*.cpp)
IMGUI_OBJS := $(patsubst imgui/%.cpp,$(OBJ_DIR)/%.o,$(IMGUI_SRC))

CXX := g++
CXXFLAGS := -g -O0 -Iinclude -Iimgui -Wall -Wextra -std=c++17

build: imgui $(OBJS) 
	cargo build --release
	cp target/release/liba_start.so $(OBJ_DIR)/
	@echo building final executable
	$(CXX) $(CXXFLAGS) $(OBJS) $(IMGUI_OBJS) \
		$(OBJ_DIR)/imgui_impl_opengl3.o $(OBJ_DIR)/imgui_impl_glfw.o $(OBJ_DIR)/liba_start.so \
		-lGL -llua -lglfw -o $(OUT)

run : build
	@./$(OUT)

test : build
	./$(OUT) test

imgui: $(IMGUI_OBJS)
	@echo building backends
	$(CXX) -c imgui/backends/imgui_impl_opengl3.cpp $(CXXFLAGS) -o $(OBJ_DIR)/imgui_impl_opengl3.o
	$(CXX) -c imgui/backends/imgui_impl_glfw.cpp $(CXXFLAGS) -o $(OBJ_DIR)/imgui_impl_glfw.o

$(OBJ_DIR)/%.o: imgui/%.cpp | $(OBJ_DIR)
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) -c $< -o $@

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.cpp | $(OBJ_DIR)
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) -c $< -o $@

$(OBJ_DIR):
	@mkdir -p $(OBJ_DIR)

clean:
	rm $(OBJS)

.PHONY: build run test clean imgui
