#include "graph.h"
#include <iostream>

using namespace std;

Graph::Graph(){};

void Graph::show_list() {
    for (auto i = this->adj_list.cbegin(); i != this->adj_list.cend(); ++i) {
      cout << i->first << "  has siblings :" << endl;
      for (auto value : i->second) {
        cout << value->id << " " << value->name << endl;
      }
    }
  };

void Graph::add_node(Node *node) { this->adj_list[node->id] = node->siblings; }

void Graph::add(Node *node) {
    this->adj_list[node->id] = node->siblings;
    vector<Node *> sibs = node->siblings;

    if (sibs.empty()) {
      return;
    }

    for (auto &s : sibs) {
      adj_list[s->id] = s->siblings;
    }
  }
