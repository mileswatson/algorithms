def table(s1, s2):
    tbl = [[None for _ in range(len(s2))] for _ in range(len(s1))]
    for k in range(max(len(s1), len(s2))):
        points = []

        # Ensures that (k, k) will be the last point in the array
        if k < len(s1):
            points += [(k, i) for i in range(min(k, len(s2)))]
        if k < len(s2):
            points += [(i, k) for i in range(min(k, len(s1)))]
        if k < len(s1) and k < len(s2):
            points += [(k, k)]

        for (i, j) in points:
            if s1[i] == s2[j]:
                leftAbove = tbl[i-1][j-1] if i > 0 and j > 0 else 0
                tbl[i][j] = 1 + leftAbove
            else:
                left = tbl[i][j-1] if j > 0 else 0
                above = tbl[i-1][j] if i > 0 else 0
                tbl[i][j] = max(left, above)
    return tbl


def match_length(tbl):
    if len(tbl) > 0 and len(tbl[-1]) > 0:
        return tbl[-1][-1]
    else:
        return 0


def get_or_zero(tbl, i, j):
    if i < 0 or j < 0:
        return 0
    elif i >= len(tbl) or j >= len(tbl[i]):
        print(i, j)
        raise Exception()
    else:
        return tbl[i][j]


def match_string(s1, s2, tbl):
    acc = ""
    i, j = len(s1)-1, len(s2)-1
    while get_or_zero(tbl, i, j) > 0:
        if s1[i] == s2[j]:
            acc = s1[i] + acc
            i, j = i-1, j-1
        elif get_or_zero(tbl, i, j-1) > get_or_zero(tbl, i-1, j):
            i, j = i, j-1
        else:
            i, j = i-1, j
    return acc
