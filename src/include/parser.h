#ifndef PARSER_H
#define PARSER_H

#include <string>
#include <vector>

using namespace std;

class Parser {
public:
  Parser();
  vector<string> split(string &token, const string &delimiter);

private:
  vector<string> upper_sentence(vector<string> &sentence);
  string upper_word(string &word);
};

#endif
