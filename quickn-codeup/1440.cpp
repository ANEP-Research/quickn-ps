#include <bits/stdc++.h>

using namespace std;

typedef int i32;

i32 main(void) {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    i32 n = 0;
    cin >> n;
    vector<i32> arr(n, 0);
    for (i32 i = 0; i < n; i++)
        cin >> arr[i];
    for (i32 i = 0; i < n; i++) {
        cout << i+1 << ": ";
        for (i32 j = 0; j < n; j++) {
            if (i != j) {
                if (arr[i] > arr[j]) {
                    cout << "> ";
                } else if (arr[i] < arr[j]) {
                    cout << "< ";
                } else {
                    cout << "= ";
                }
            }
        }
        cout << '\n';
    }
    return 0;
}