#ifndef GRAPH_SERVICE_H
#define GRAPH_SERVICE_H

#include "graph.h"
#include "node.h"
#include <map>
#include <string>
#include <vector>

using namespace std;

class GraphService {
public:
  GraphService();
  int ids = 0;
  map<int, Graph *> graph_cluster;

  vector<string> split(string &data, const string &delimiter);
  string get_action(string token);
  void create(string token);
  void show(string token);

private:
  void create_graph();
};

#endif
