#include "parser.h"
#include <cctype>

int UPPER_CASE_MIN = 60;
int UPPER_CASE_MAX = 90;

Parser::Parser() {};

vector<string> Parser::split(string &data, const string &delimiter) {

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
}

vector<string> Parser::upper_sentence(vector<string> &sentence) {
  vector<string> new_sentence;
  for (auto &word : sentence) {
    new_sentence.push_back(this->upper_word(word));
  }

  return new_sentence;
};

string Parser::upper_word(string &word) {
  string new_word;
  for (auto &letter : word) {
    char new_letter = isupper(letter) ? letter : tolower(letter);
    new_word.push_back(new_letter);
  }

  return new_word;
};
