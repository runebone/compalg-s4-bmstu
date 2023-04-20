import numpy as np

def interpolate_newton(x, n, x_list, y_list):
    x_list, y_list = sort_first_array_apply_permutation_to_second(x_list, y_list)

    start, end = get_start_end_bounds(x, x_list, n + 1)

    x_list = x_list[start:end]
    y_list = y_list[start:end]

    dd_list = calculate_divided_differnces(x_list, y_list)

    return get_newton_polynomial_func(x_list, dd_list)

def get_newton_polynomial_func(x_list, dd_list):
    def func(x):
        result = 0

        for i in range(len(x_list)):
            term = 1
            for j in range(i):
                term *= x - x_list[j]
            result += term * dd_list[i]

        return result

    return func

def calculate_divided_differnces(x_list, y_list):
    dd_list = y_list.copy()
    k = 0
    n = len(x_list) - 1

    result_dds = [y_list[0]]

    for _ in range(n):
        for i in range(n - k):
            numerator = dd_list[i] - dd_list[i + 1]
            denominator = x_list[i] - x_list[i + k + 1]

            assert denominator != 0

            dd = numerator / denominator

            if i == 0: result_dds.append(dd)

            dd_list[i] = dd
        k += 1

    return result_dds

def get_start_end_bounds(x, x_list, n_points):
    index = get_x_nearest_index(x, x_list)

    start = index
    end = index + 1 # End is exclusive

    total_points = len(x_list)

    assert n_points < total_points

    i = 0 # Start choosing from the next of current_point_index
    if x < x_list[index]:
        i = 1 # Start choosing from the prev of current_point_index

    for _ in range(1, n_points):
        if i % 2 == 0:
            if end < total_points:
                end += 1
            if start > 0:
                i += 1
        else:
            if start > 0:
                start -= 1
            if end < total_points:
                i += 1

    return start, end

def get_x_nearest_index(x, x_list):
    index = 0
    min_diff = abs(x_list[index] - x)

    for i in range(1, len(x_list)):
        diff = abs(x_list[i] - x)
        if diff < min_diff:
            min_diff = diff
            index = i

    return index

def sort_first_array_apply_permutation_to_second(first, second):
    first = np.array(first)
    second = np.array(second)

    indices = np.argsort(first)

    first = list(first[indices])
    second = list(second[indices])

    return first, second
