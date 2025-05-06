#ifndef NODE_H // include guard
#define NODE_H

#include <string>

using namespace std;

// TODO: this should be generic
class Node {
public:
  Node(int id, string name);
  int id;
  string name;
};

#endif
