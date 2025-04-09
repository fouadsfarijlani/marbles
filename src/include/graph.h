#ifndef GRAPH_H
#define GRAPH_H

#include "node.h"
#include <map>
#include <vector>

using namespace std;

class Graph {
public:
  Graph();
  map<int, vector<Node *>> adj_list;

  void show_list();
  void add_node(Node *node);
  void add(Node *node);
};
;

#endif
