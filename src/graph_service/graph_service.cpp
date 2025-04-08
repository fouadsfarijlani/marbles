#include "graph_service.h"

GraphService::GraphService() {};

vector<string> GraphService::split(string &data, const string &delimiter) {
  // TODO: this function is shit change it
  size_t pos = 0;
  vector<string> tokens;
  string token;
  while ((pos = data.find(delimiter)) != string::npos) {
    token = data.substr(0, pos);
    tokens.push_back(token);
    data.erase(0, pos + delimiter.length());
  }
  tokens.push_back(data);

  return tokens;
};

string GraphService::get_action(string token) {
  string delimimter = " ";
  string action = token.substr(0, token.find(delimimter));

  return action;
};

void GraphService::create(string token) { return; }
void GraphService::show(string token) { return; }

void GraphService::create_graph() {
  ids += 1;
  Graph *graph = new Graph();
  graph_cluster[ids] = graph;
};
