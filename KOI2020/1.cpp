#include <bits/stdc++.h>

using namespace std;

typedef int i32;

const i32 ALPHABETS = 26;

i32 main(void) {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    string s, t;
    cin >> s >> t;
    vector<vector<i32>> next(t.length()+1, vector<i32>(ALPHABETS, -1));
    vector<i32> cur(ALPHABETS, -1);
    for (i32 i = t.length()-1; i >= 0; i--) {
        i32 a = static_cast<i32>(t[i]-'a');
        cur[a] = i;
    }
    for (i32 i = t.length()-1; i >= 0; i--) {
        i32 a = static_cast<i32>(t[i]-'a');
        for (i32 j = 0; j < ALPHABETS; j++)
            next[i+1][j] = cur[j];
        cur[a] = i;
    }
    for (i32 j = 0; j < ALPHABETS; j++)
        next[0][j] = cur[j];
    i32 pointer = -1;
    i32 res = 1;
    for (auto c: s) {
        i32 prev = pointer;
        i32 a = static_cast<i32>(c - 'a');
        pointer = next[pointer+1][a];
        if (pointer == -1) {
            res = -1;
            break;
        }
        if (pointer >= t.length()) {
            pointer %= t.length();
            res += 1;
        }
        if (pointer <= prev)
            res += 1;
    }
    cout << res;
    return 0;
}