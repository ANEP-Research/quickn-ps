#include <bits/stdc++.h>

using namespace std;

typedef long long i64;

const int INF = 1e9;
const int MAX = 5e4;

int main(void) {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    int n = 0;
    cin >> n;
    int s = 0, g = 0, p = 0, d = 0;
    cin >> s >> g >> p >> d;
    string str;
    cin >> str;
    vector< vector< int > > dp(n, vector< int >(MAX+1, 0));
    for (int i = 0; i < n; i++) {
        int m = 0;
        int min_def = 0, max_def = 0;
        if (str[i] == 'B') {
            min_def = 0;
            max_def = s-1;
        } else if (str[i] == 'S') {
            min_def = s;
            max_def = g-1;
        } else if (str[i] == 'G') {
            min_def = g;
            max_def = p-1;
        } else if (str[i] == 'P') {
            min_def = p;
            max_def = d-1;
        } else {
            min_def = d;
            max_def = d;
        }
        for (int cost1 = 0; cost1 <= max_def; cost1++) {
            if (i == 0) {
                if (cost1 >= min_def)
                    dp[i][cost1] = cost1;
            } else {
                dp[i][cost1] = cost1;
                int l = max(0, min_def - cost1);
                int r = max_def - cost1;
                if (min_def == d)
                    r = MAX;
                for (int cost2 = l; cost2 <= r; cost2++) {
                        dp[i][cost1] = max(dp[i-1][cost2] + cost1, dp[i][cost1]);
                }
            }
            m = max(dp[i][cost1], m);
        }
        cout << m << endl;
    }
    int res = 0;
    for (int cost = 0; cost <= MAX; cost++) {
        if (dp[n-1][cost] != INF)
            res = max(res, dp[n-1][cost]);
    }
    cout << res;
    return 0;
}