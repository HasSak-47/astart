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
};

int main() {
    Map map = {};
    map.populate_and_link();
    return 0;
}
