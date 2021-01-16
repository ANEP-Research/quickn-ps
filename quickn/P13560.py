from sys import stdin
n = int(input())
scores = list(map(int, input().split()))
scores.sort()
num = [scores[i] for i in range(n)]
res = 1
for i in range(n-1, -1, -1):
    a = scores[i]
    j = 0
    while j < n and a > 0:
        if num[j] < n-1 and j != i:
            num[j] += 1
            a -= 1
        j += 1
    if a > 0:
        res = -1
        break
for i in range(n):
    if num[i] != n-1:
        res = -1
        break
print(res)
