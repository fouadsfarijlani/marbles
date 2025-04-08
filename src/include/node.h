#ifndef NODE_H // include guard
#define NODE_H

#include <string>
#include <vector>

using namespace std;

class Node {
public:
  Node(int id, string name);
  int id;
  string name;
  vector<Node *> siblings;

  void add_sibling(Node *sibling);
  void show_siblibngs();
};

#endif
