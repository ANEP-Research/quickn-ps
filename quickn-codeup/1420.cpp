#include <bits/stdc++.h>

using namespace std;

typedef int i32;

i32 main(void) {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    i32 n = 0;
    cin >> n;
    vector< pair<i32, string> > arr;
    for (i32 i = 0; i < n; i++) {
        string s;
        i32 cost = 0;
        cin >> s >> cost;
        arr.push_back(make_pair(cost, s));
    }
    auto cmp = [](pair<i32, string> s1, pair<i32, string> s2) { return s1.first > s2.first; };
    sort(arr.begin(), arr.end(), cmp);
    cout << arr[2].second;
    return 0;
}