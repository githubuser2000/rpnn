#include <ncurses.h>
#include <cstdlib>
#include <string>
#include <iostream>
#include <sstream>
#include <memory>

std::string execCommand(const std::string& cmd) {
    std::array<char, 128> buffer;
    std::string result;
    std::shared_ptr<FILE> pipe(popen(cmd.c_str(), "r"), pclose);
    if (!pipe) throw std::runtime_error("popen() failed!");
    while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
        result += buffer.data();
    }
    return result;
}

int main() {
    // Initialisiere ncurses
    initscr();
    cbreak();
    noecho();
    curs_set(0);

    int height, width;
    getmaxyx(stdscr, height, width);

    // Fenster für linken und rechten Bereich
    int mid = width / 2;

    WINDOW* leftWin = newwin(height, mid, 0, 0);
    WINDOW* rightWin = newwin(height, width - mid, 0, mid);

    box(leftWin, 0, 0);
    box(rightWin, 0, 0);

    // Befehle
    std::string cmd1 = "ls -l"; // Beispielbefehl links
    std::string cmd2 = "uptime"; // Beispielbefehl rechts

    // Befehle ausführen
    std::string output1 = execCommand(cmd1);
    std::string output2 = execCommand(cmd2);

    // Ausgaben in die Fenster schreiben
    int y = 1;
    std::istringstream iss1(output1);
    std::string line;
    while (std::getline(iss1, line) && y < height - 1) {
        mvwprintw(leftWin, y++, 1, "%s", line.c_str());
    }

    y = 1;
    std::istringstream iss2(output2);
    while (std::getline(iss2, line) && y < height - 1) {
        mvwprintw(rightWin, y++, 1, "%s", line.c_str());
    }

    wrefresh(leftWin);
    wrefresh(rightWin);

    getch(); // Warten auf Tastendruck

    // Aufräumen
    delwin(leftWin);
    delwin(rightWin);
    endwin();

    return 0;
}
