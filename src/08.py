import sys
from itertools import permutations


def get_inputs():
    res = []
    for line in sys.stdin:
        sig_pats, out_vals = line.split(" | ")
        res.append((sig_pats, out_vals))
    return res


strokes = [
    {1, 2, 3, 4, 5, 6},
    {3, 4},
    {0, 2, 3, 5, 6},
    {0, 2, 3, 4, 5},
    {0, 1, 3, 4},
    {0, 1, 2, 4, 5},
    {0, 1, 2, 4, 5, 6},
    {2, 3, 4},
    {0, 1, 2, 3, 4, 5, 6},
    {0, 1, 2, 3, 4, 5}
]


def check(words, target):
    result = {}
    p = None
    for p in permutations('abcdefg', len('fedabcg')):
        w_idx = None
        for w_idx, word in enumerate(words):
            idcs = set(p.index(c) for c in word)
            try:
                pos = strokes.index(idcs)
                result[word] = pos
            except ValueError:
                break
        if w_idx == len(words) - 1:
            break

    # all fit.
    # result_ = {}
    for t in target:
        idcs = set(p.index(c) for c in t)
        try:
            pos = strokes.index(idcs)
            result[t] = pos
        except ValueError:
            if len(t) == 3:
                result[t] = 7
            if len(t) == 2:
                result[t] = 1
            if len(t) == 7:
                result[t] = 8
            if len(t) == 4:
                result[t] = 4
            else:
                temp_ = set(t)
                for _k in result:
                    if temp_ == set(_k):
                        result[t] = result[_k]
                        break
                assert t in result
    return result


def main():
    inputs = get_inputs()
    acc = 0
    for sig_pat, out_val in inputs:
        mappings = check(sig_pat.strip().split(" "), out_val.strip().split(" "))
        temp_ = ''
        for word in out_val.strip().split(" "):
            temp_ += str(mappings[word])
        acc += int(temp_)

    print(acc)


if __name__ == '__main__':
    main()
