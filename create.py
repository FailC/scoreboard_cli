

with open('user.txt', 'w') as f:
    for i in range(1_000_000):
        f.write('User ' + str(i))
        f.write("\n")
