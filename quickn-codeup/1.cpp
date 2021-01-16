#include <bits/stdc++.h>

using namespace std;

typedef int i32;

i32 main(void) {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    i32 n = 0;
    cin >> n;
    i32 time = 0;
    i32 sum = 0;
    for (i32 i = 1; i <= n; i++) {
        sum += i;
        time++;
    }
    while (n --> 0) {
        cout << n << "\n";
        sum -= n;
        time++;
        //cout << sum << "\n";
    }
    cout << "Time: " << time;
    return 0;
}