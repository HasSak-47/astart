SRC_DIR := src
OBJ_DIR := .cache
OUT := astart
SRCS := $(shell find src -name \*.cpp)
OBJS := $(patsubst $(SRC_DIR)/%.cpp,$(OBJ_DIR)/%.o,$(SRCS))

CXX := g++
CXXFLAGS := -g -Iinclude -export-dynamic -Wall -Wextra -std=c++17

build: $(OBJS)
	g++ $(CXXFLAGS) -o $(OUT) $(OBJS) $(LDFLAGS)

run : build
	@./$(OUT)

test : build
	./$(OUT) test

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.cpp | $(OBJ_DIR)
	@mkdir -p $(dir $@)
	$(CXX) $(CXXFLAGS) -c $< -o $@

$(OBJ_DIR):
	@mkdir -p $(OBJ_DIR)

clean:
	rm $(OBJS)

.PHONY: build run test clean
