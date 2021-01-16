#include <bits/stdc++.h>

using namespace std;

typedef long long i64;

const int INF = 1e9;
const int MAX = 5e4;

class Segtree {
    private:
        vector<int> data;
    public:
        Segtree() {
            data.resize((MAX+1)*10, 0);
        }

        void _update(int node, int start, int end, int pos, int x) {
            if (start > pos || end < pos)
                return;
            if (start == end) {
                data[node] = x;
            } else {
                int mid = (start+end)/2;
                _update(node*2, start, mid, pos, x);
                _update((node*2)+1, mid+1, end, pos, x);
                data[node] = max(data[node*2], data[(node*2)+1]);
            }
        }

        void update(int pos, int x) {
            _update(1, 0, MAX, pos, x);
        }

        int query(int l, int r) {
            return _query(1, 0, MAX, l, r);
        }

        int _query(int node, int start, int end, int l, int r) {
            if (start > r || end < l)
                return 0;
            if (l <= start && end <= r)
                return data[node];
            else {
                int mid = (start+end)/2;
                return max(_query(node*2, start, mid, l, r), _query((node*2)+1, mid+1, end, l, r));
            }
        }
};

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
    vector< unique_ptr< Segtree > > dp_max;
    for (int i = 0; i < n; i++) {
        dp_max.push_back(make_unique<Segtree>());
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
                dp[i][cost1] = 0;
                int l = max(0, min_def - cost1);
                int r = max_def - cost1;
                if (min_def == d)
                    r = MAX;
                dp[i][cost1] = dp_max[i-1]->query(l, r) + cost1;
            }
            dp_max[i]->update(cost1, dp[i][cost1]);
        }
    }
    cout << dp_max[n-1]->query(0,MAX);
    return 0;
}