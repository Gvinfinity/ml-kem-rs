def bitrev(i):
    # print(i)
    o = 0
    o |= (i & 1) << 6
    o |= ((i >> 1) & 1) << 5
    o |= ((i >> 2) & 1) << 4
    o |= ((i >> 3) & 1) << 3
    o |= ((i >> 4) & 1) << 2
    o |= ((i >> 5) & 1 ) << 1
    o |= i >> 6

    return o

for i in range(128):
    print(bin(bitrev(i)))