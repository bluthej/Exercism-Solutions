def steps(number):
    """
    Calculate number of steps for the Collatz conjecture
    """
    if number < 1:
        raise ValueError('Only positive integers are allowed')
    n = 0
    while number != 1:
        n += 1
        if number % 2 == 0:
            number /= 2
        else:
            number = 3 * number + 1
    return n
