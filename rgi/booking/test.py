test = [None] * 10**6

for var in test:
    test.remove(var)
    if len(test) % 1000 == 0:
        print(len(test))

