#include "graph.h"
#include "graph_service.h"
#include "node.h"
#include <iostream>

using namespace std;

int main(int argc, char *argv[]) {
  // GraphService *graph_service = new GraphService();
  // for (string line; cout << "Marbles> " && getline(std::cin, line);) {
  //
  //   if (line == "exit") {
  //     cout << "Goodbye.\n";
  //     return 0;
  //   };
  // };
  //

  Graph graph = Graph();

  graph.add_one(1, "super man");
  auto key_exist = graph.key_exist(1);
  cout << key_exist << endl;

  graph.show();

  graph.remove_one(1);
  bool key_exist_2 = graph.key_exist(1);

  cout << key_exist_2 << endl;

  return 0;
}
