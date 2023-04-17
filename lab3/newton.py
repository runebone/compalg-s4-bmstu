import numpy as np

def interpolate_newton(x, n, x_list, y_list):
    x_list, y_list = sort_first_array_apply_permutation_to_second(x_list, y_list)

    x_nearest_index = get_x_nearest_index(x, x_list)
    start, end = get_start_end_bounds(x_nearest_index, n + 1)

    x_list = x_list[start:end]
    y_list = y_list[start:end]

    dd_list = calculate_divided_differnces(x_list, y_list)

    return get_newton_polynome_func(x_list, dd_list)

def get_newton_polynome_func(x_list, dd_list):
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

def get_start_end_bounds(index, n_points):
    start = index - n_points // 2
    end = index + n_points // 2

    # End is exclusive
    if n_points % 2 == 1:
        end += 1

    if start < 0:
        end -= start
        start -= start

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
