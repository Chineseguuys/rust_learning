#include <tuple>
#include <iostream>

using namespace std;




int main() {
    tuple<double, int, char> t{3.14, 123, 1};
    auto [x, y, z] = t;
    cout << x << endl;
    return 0;
}