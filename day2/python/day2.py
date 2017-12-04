FILENAME = '../input'

with open(FILENAME) as f:
    i = [l.strip() for l in f.readlines()]


lines = [[int(x.strip()) for x in l.split('\t')] for l in i]
s = sum([max(x) - min(x) for x in lines])
print(s)

# lines = [
#     [5, 9, 2, 8],
#     [9, 4, 7, 3],
#     [3, 8, 6, 5],
# ]

total = 0
for l in lines:
    s = sorted(l)
    r = reversed(s)
    for high_item in r:
        for low_item in s:
            if high_item % low_item == 0 and high_item != low_item:
                total += high_item / low_item
                break
print(total)
