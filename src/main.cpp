#include "graph_service.h"
#include <iostream>

using namespace std;

int main(int argc, char *argv[]) {
  GraphService *graph_service = new GraphService();
  for (string line; cout << "Nodzy> " && getline(std::cin, line);) {
    vector<string> tokens = graph_service->split(line, " ");

    if (line == "exit") {
      cout << "Goodbye.\n";
      return 0;
    };
  };
  return 0;
}
