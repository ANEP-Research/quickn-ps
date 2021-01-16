#include <bits/stdc++.h>

using namespace std;

typedef int i32;

i32 main(void) {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    i32 n = 0;
    cin >> n;
    unordered_set<i32> s;
    for (i32 i = 0; i < n; i++) {
        i32 a = 0;
        cin >> a;
        s.insert(a);
    }
    i32 q = 0;
    cin >> q;
    for (i32 _i = 0; _i < q; _i++) {
        i32 a = 0;
        cin >> a;
        if (s.find(a) != s.end()) {
            cout << "1 ";
        } else {
            cout << "0 ";
        }
    }
    return 0;
}