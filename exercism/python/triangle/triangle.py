def equilateral(sides):
    if valid_triangle(sides=sides):
        if sides[0] == sides[1] and sides[1] == sides[2] and sides[2] == sides[0]:
            return True
    return False


def isosceles(sides):
    if valid_triangle(sides=sides):
        if sides[0] == sides[1] or sides[1] == sides[2] or sides[2] == sides[0]:
            return True
    return False


def scalene(sides):
    if valid_triangle(sides=sides):
        if sides[0] != sides[1] and sides[1] != sides[2] and sides[2] != sides[0]:
            return True
    return False


def valid_triangle(sides):
    # [0,0,0]
    if sides[0] == 0 and sides[1] == 0 and sides[2] == 0:
        return False
    # a + b ≥ c
    if sides[0] + sides[1] < sides[2]:
        return False
    # b + c ≥ a
    if sides[1] + sides[2] < sides[0]:
        return False
    # a + c ≥ b
    if sides[0] + sides[2] < sides[1]:
        return False
    return True
