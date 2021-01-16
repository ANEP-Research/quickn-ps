#include <bits/stdc++.h>

using namespace std;

typedef long long i64;


i64 pow_mod(i64 a, i64 x, i64 m) {
    i64 a_t = a, x_t = x, r = 1;
    while (x_t != 0) {
        if (x_t % 2 == 1) {
            r *= a_t;
            r %= m;
        }
        a_t *= a_t;
        a_t %= m;
        x_t /= 2;
    }
    return r;
}

int main(void) {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    i64 m = 0, s = 0, x1 = 0, x2 = 0;
    cin >> m >> s >> x1 >> x2;
    i64 a = (((m + x1 - x2)%m)*pow_mod((m + s - x1)%m, m-2, m))%m;
    i64 c = (m + x1 - ((a*s)%m))%m;
    cout << a << " " << c;
    return 0;
}