#include <iostream>
#include <vector>
#include <string>
#include <sstream>
#include <random>
#include <openssl/sha.h> // für SHA256

struct Node {
    std::string name;
    std::string hash;
    std::vector<Node> children;
};

// Hilfsfunktion: SHA256 Hash aus String
std::string sha256(const std::string &str) {
    unsigned char hash[SHA256_DIGEST_LENGTH];
    SHA256((unsigned char*)str.c_str(), str.size(), hash);
    std::stringstream ss;
    for(int i = 0; i < SHA256_DIGEST_LENGTH; ++i)
        ss << std::hex << (int)hash[i];
    return ss.str();
}

std::string generateName(const std::string &level) {
    return level + "_" + std::to_string(rand() % 1000 + 1);
}

Node generateNode(const std::string &level, int depth, const std::string &parentHash="") {
    Node node;
    node.name = generateName(level);
    int numChildren = (depth > 0) ? (rand() % 3 + 1) : 0;

    for(int i = 0; i < numChildren; ++i) {
        if(depth == 3)
            node.children.push_back(generateNode("Bundesland", depth-1, parentHash));
        else if(depth == 2)
            node.children.push_back(generateNode("Bezirk", depth-1, parentHash));
        else if(depth == 1)
            node.children.push_back(generateNode("Dorf", depth-1, parentHash));
    }

    // Hash berechnen
    std::string data = node.name + parentHash;
    for(const auto &c : node.children)
        data += c.hash;
    node.hash = sha256(data);

    return node;
}

// Hilfsfunktion zum Drucken
void printNode(const Node &node, int indent=0) {
    for(int i=0;i<indent;i++) std::cout << "  ";
    std::cout << node.name << " : " << node.hash << "\n";
    for(const auto &c : node.children)
        printNode(c, indent+1);
}

int main() {
    srand(time(NULL));
    Node tree = generateNode("Land", 3);
    printNode(tree);
    return 0;
}
