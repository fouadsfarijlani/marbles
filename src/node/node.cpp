#include "node.h"
#include <iostream>

using namespace ::std;

Node::Node(int id, string namne) {
  this->id = id;
  this->name = name;
};

void Node::add_sibling(Node *sibling) { this->siblings.push_back(sibling); };

void Node::show_siblibngs() {
  if (this->siblings.empty()) {
    cout << "No siblings assigned to this node" << endl;
    return;
  };

  cout << "Node " << this->name << " id: " << this->id
       << " has sibling to: " << endl;
  for (const auto &s : this->siblings) {
    cout << s->name << " " << s->id << endl;
  }
};
