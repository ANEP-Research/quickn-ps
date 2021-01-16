#include <bits/stdc++.h>

using namespace std;

typedef int i32;

i32 main(void) {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    i32 n = 0, m = 0;
    cin >> n >> m;
    vector<vector<i32>> res(n, vector<i32>(m, 0));
    i32 pointer = n*m;
    i32 x = n-1, y = m-1;
    i32 dir = 0;
    while (pointer > 0) {
        res[x][y] = pointer;
        i32 prev_x = x, prev_y = y;
        if (dir == 0)
            y--;
        else if (dir == 1)
            x--;
        else if (dir == 2)
            y++;
        else
            x++;
        if (x == -1 || y == -1 || x == n || y == m || res[x][y] != 0) {
            x = prev_x;
            y = prev_y;
            dir++;
            dir %= 4;
            if (dir == 0)
            y--;
        else if (dir == 1)
            x--;
        else if (dir == 2)
            y++;
        else
            x++;
        }
        pointer--;
    }
    for (i32 i = 0; i < n; i++) {
        for (i32 j = 0; j < m; j++)
            cout << res[i][j] << " ";
        cout << "\n";
    }
    return 0;
}