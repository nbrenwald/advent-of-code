from sympy import *
import sys

def main():
    lines = [line.strip() for line in sys.stdin.readlines()]

    hail_stones = []
    for line in lines:
        a, b = line.split("@")
        p = list(map(int, a.strip().split(", ")))
        q = list(map(int, b.strip().split(", ")))
        hail_stones.append((p, q))

    x0, y0, z0, xv, yv, zv, t1, t2, t3 = symbols('x0 y0 z0 xv yv zv t1 t2 t3')
    equations = []
    for (p, q), t in zip(hail_stones, [t1, t2, t3]):
        equations.append(p[0] + q[0]*t - (x0 + xv*t))
        equations.append(p[1] + q[1]*t - (y0 + yv*t))
        equations.append(p[2] + q[2]*t - (z0 + zv*t))

    res = solve(equations, x0, y0, z0, xv, yv, zv, t1, t2, t3, dict=True)[0]
    print(res[x0] + res[y0] + res[z0])
    print("Hello World!")

if __name__ == "__main__":
    main()
