#ifndef GRAPH_H
#define GRAPH_H

#include "node.h"
#include <unordered_map>
#include <vector>

// I need the following:
//  {
//    <<id>> : {
//      ptr_to_item
//      array<pointers> siblings
//    }
//  }

struct GraphEntry {
  Node *node;
  vector<Node *> siblings;
};

class Graph {
public:
  Graph();
  std::unordered_map<int, GraphEntry> adj_list;
  void add_one(int key, string name);
  void remove_one(int key);
  bool key_exist(int id); // this needs fixing
  void show();
};
;

#endif
