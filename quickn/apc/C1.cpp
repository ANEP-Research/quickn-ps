#include <bits/stdc++.h>

using namespace std;

const int FREEZE = 180;

int main(void) {
    //ios_base::sync_with_stdio(false);
    //cin.tie(nullptr);
    int n = 0, m = 0;
    cin >> n >> m;
    vector< tuple<int, int, int, int> > list(n, make_tuple(0, 0, 0, 0)); // (id, cnt, pan, last)
    for (int i = 0; i < n; i++)
        get<0>(list[i]) = i;
    vector< vector< tuple<int, int, int> > > sub(n);
    for (int i = 0; i < m; i++) {
        int h = 0, m = 0, id = 0, p = 0, s = 0;
        scanf("%d:%d %d %d %d", &h, &m, &id, &p, &s);
        id--;
        int t = h*60 + m;
        if (t > FREEZE) {
            sub[id].push_back(make_tuple(t, p, s));
        } else {
            get<2>(list[id]) += t + (s-1)*20;
            get<1>(list[id]) += 1;
            get<3>(list[id]) = t;
        }
    }
    sort(list.begin(), list.end(), [](tuple<int, int, int, int> a, tuple<int, int, int, int> b) { 
        if (get<1>(a) == get<1>(b)) {
            if (get<2>(a) == get<2>(b)) {
                get<3>(a) < get<3>(b)
            } else {
                get<2>(a) < get<2>(b)
            }
        } else {
            get<1>(a) > get<1>(b)
        }
    });
    return 0;
}