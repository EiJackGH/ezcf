#include <string>
#include <vector>

struct Card {
    std::string title;
    std::string content;
    std::string timestamp;
};

class CardFile {
public:
    void addCard(std::string t, std::string c);
    void saveToFile(std::string filename);
    void searchCards(std::string query);
private:
    std::vector<Card> index;
};
