import numpy as np
from interpolation import interpolate_newton_3d
from interpolation import interpolate_spline_3d
from interpolation import interpolate_mix_3d

data = np.ndarray(shape=(5,5,5), buffer=np.array([
    0,  1,  4,  9,  16,
    1,  2,  5,  10, 17,
    4,  5,  8,  13, 20,
    9,  10, 13, 18, 25,
    16, 17, 20, 25, 32,

    1,  2,  5,  10, 17,
    2,  3,  6,  11, 18,
    5,  6,  9,  14, 21,
    10, 11, 14, 19, 26,
    17, 18, 21, 26, 33,

    4,  5,  8,  13, 20,
    5,  6,  9,  14, 21,
    8,  9,  12, 17, 24,
    13, 14, 17, 22, 29,
    20, 21, 24, 29, 36,

    9,  10, 13, 18, 25,
    10, 11, 14, 19, 26,
    13, 14, 17, 22, 29,
    18, 19, 22, 27, 34,
    25, 26, 29, 34, 41,

    16, 17, 20, 25, 32,
    17, 18, 21, 26, 33,
    20, 21, 24, 29, 36,
    25, 26, 29, 34, 41,
    32, 33, 36, 41, 48,
]), dtype=int)

if __name__ == "__main__":
    x = 1.5
    y = 1.5
    z = 1.5
    nx = 1
    ny = 1
    nz = 1
    l = np.arange(0, 5, 1)

    newton = interpolate_newton_3d(x, y, z, nx, ny, nz, l, l, l, data)
    spline = interpolate_spline_3d(x, y, z, l, l, l, data)
    mix = interpolate_mix_3d(x, y, z, nx, ny, nz, l, l, l, data)

    print("{:<7}: {:.3f}".format("Newton", newton));
    print("{:<7}: {:.3f}".format("Spline", spline));
    print("{:<7}: {:.3f}".format("Mix", mix));

    print(data)
