#include <cstddef>
#include <cstdlib>
#include <iostream>
#include <vector>

struct Node{
    struct Child{
        float distance;
        Node* child;
    };
    std::vector<Child> childs;
};

struct Map{
    std::vector<Node> nodes;

    void populate_and_link() {
        nodes.resize(10, {});
        size_t r = rand() % 10;
        size_t b = rand() % 10;
        while (b == r ){
            b = rand() % 10;
        }

        float d = (rand() % 1024) / 1024.;
        nodes[r].childs.push_back(Node::Child{d, &nodes[b]});
        nodes[b].childs.push_back(Node::Child{d, &nodes[r]});
    }

    bool is_end(size_t id) {
        return id == 10;
    }

    double heuristic(size_t id) {
        return 0.;
    } 

    size_t start(){
        return 0;
    }
};

struct FFI{ char bytes[40]; };
struct FFI_AStart{ char bytes[78]; };


FFI_AStart new_world(FFI world) ;

void start(FFI_AStart* start);
bool next(FFI_AStart * start, size_t* val) ;

FFI new_ffi(
    void* obj,
    void* start,
    void* get_neightbors,
    void* heuristic,
    void* is_end
);



int main() {
    Map map = {};
    map.populate_and_link();

    FFI ffi = new_ffi(
        (void*)&map,
        NULL,
        NULL,
        NULL,
        NULL
    );

    return 0;
}
