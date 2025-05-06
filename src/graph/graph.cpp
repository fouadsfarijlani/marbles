#include "graph.h"
#include <iostream>
#include <string>
#include <utility>

using std::string, std::cout, std::endl, std::make_pair;

Graph::Graph() {};

void Graph::add_one(int id, std::string name) {
  bool key_exist = this->adj_list.contains(id);
  if (key_exist) {
    std::cout << "Violating unique constrains" << std::endl;
    return;
  }

  Node *node = new Node(id = id, name = name);
  cout << "adding one: " << node->id << " " << node->name << endl;
  cout << node << endl;
  GraphEntry entry;
  entry.node = node;
  this->adj_list.insert(make_pair(id, entry));
};

void Graph::remove_one(int id) {
  if (!this->key_exist(id)) {
    return;
  }
  GraphEntry target = this->adj_list[id];
  Node *target_node = target.node;
  if (target_node == NULL) {
    cout << "Node cannot be null";
    return;
  }

  cout << "removing " << target_node << endl;
  delete target_node;

  cout << "removing id " << id << endl;
  this->adj_list.erase(id);
}

bool Graph::key_exist(int id) { return this->adj_list.contains(id); }

void Graph::show() {
  if (this->adj_list.empty()) {
    cout << "No data exists";
  }

  cout << "    id    " << "|" << "    data    " << endl;
  for (auto entry : this->adj_list) {
    cout << "  " << (entry.first) << "  " << "|" << "  " << entry.second.node
         << endl;
  }
}
