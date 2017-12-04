
with open('input') as f:
    i = f.read().strip()


def yield_sequential_equal_elements(seq):
    seq2 = iter(seq)
    first = next(seq2)
    seq2 = list(seq2) + [first]
    for a, b in zip(seq, seq2):
        if a == b:
            yield int(a)


assert sum(yield_sequential_equal_elements('1122')) == 3
assert sum(yield_sequential_equal_elements('1111')) == 4
assert sum(yield_sequential_equal_elements('1234')) == 0
assert sum(yield_sequential_equal_elements('9121212129')) == 9
print(sum(yield_sequential_equal_elements(i)))


def yield_halfway_equal_elements(seq):
    seq2 = iter(seq)
    n_to_shift = int(len(seq) / 2)
    first_n = [next(seq2) for _ in range(n_to_shift)]
    seq2 = list(seq2) + first_n
    for a, b in zip(seq, seq2):
        if a == b:
            yield int(a)


assert sum(yield_halfway_equal_elements('1212')) == 6
assert sum(yield_halfway_equal_elements('1221')) == 0
assert sum(yield_halfway_equal_elements('123425')) == 4
assert sum(yield_halfway_equal_elements('123123')) == 12
assert sum(yield_halfway_equal_elements('12131415')) == 4
print(sum(yield_halfway_equal_elements(i)))
