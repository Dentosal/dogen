function = lambda x: (1-x**2)**0.5

# comment

print "123"
print function
print 1+v(x)*y**4
print("123")
print(function)
print(1+v(x)*y**4)

for i in range(1, 11):
    print i


def add_middle_points(L):
    ret = [L[0]]
    for l in L[1:]:
        ret+=[(ret[-1]+l)/2.0, l]
    return ret


iteration = 0
points = [0, 1]
while True:
    iteration += 1
    points = add_middle_points(points)
    values = map(function,points)
    slice_width = (1.0 / (len(values)-1))

    print sum(map(lambda q: q*slice_width, values))*4
