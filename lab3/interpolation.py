from newton import interpolate_newton

def interpolate_newton_2d(x, nx, x_list, table_1d):
    result = interpolate_newton(x, nx, x_list, table_1d)(x)

    return result

def interpolate_newton_3d(x, y, nx, ny, x_list, y_list, table_2d):
    """
    table[A][B] = f(x_list[B], y_list[A])
    table[i] = array of function values
    table = array of arrays of function values
    """
    # new_table_1d[i] = f(x, y_list[i])
    new_table_1d = []
    for i in range(len(y_list)):
        interp_result = interpolate_newton_2d(x, nx, x_list, table_2d[i])
        new_table_1d.append(interp_result)

    result = interpolate_newton(y, ny, y_list, new_table_1d)(y)

    return result

def interpolate_newton_4d(x, y, z, nx, ny, nz, x_list, y_list, z_list, table_3d):
    """
    table[A][B][C] = f(x_list[C], y_list[B], z_list[A])
    table[A][B] = array of f(x, y_list[B], z_list[A]) where x runs x_list values
    """
    # new_table_1d[i] = f(x, y, z_list[i])
    new_table_1d = []
    for i in range(len(z_list)):
        interp_result = interpolate_newton_3d(x, y, nx, ny, x_list, y_list, table_3d[i])
        new_table_1d.append(interp_result)

    result = interpolate_newton(z, nz, z_list, new_table_1d)(z)

    return result
