#!/usr/bin/python3

from datetime import datetime
from random import randint, random
from math import acos, asin, pi
from cmath import phase
import sys

sys.exit(255)

# This file is meant to provide an estimate of how much of the Python prototype
# functionality has already been ported to Rust
# All functions for which an equivalent has already implemented in Rust are to
# be deleted from this file.
# Hence, as long as this file is not empty, the Rust version lacks some of the
# functionality provided by the prototype (though it already provides things
# that the prototype does not include, e.g. Delaunay tiling / non-hexagonal
# periodic tilings)

try:
    import psutil
    cpu = psutil.cpu_percent()
    print("Current CPU usage: {}".format(cpu))
    if cpu > 20:
        print("Delaying wallpaper to later")
        sys.exit(2)

def inside_triangle(pt, v1, v2, v3):
    d1 = sign(pt, v1, v2)
    d2 = sign(pt, v2, v3)
    d3 = sign(pt, v3, v1)
    has_neg = (d1 < 0) or (d2 < 0) or (d3 < 0)
    has_pos = (d1 > 0) or (d2 > 0) or (d3 > 0)
    return not (has_neg and has_pos);

def fill_triangle(a, b, c, scene):
    I = len(scene)
    J = len(scene[0])
    color = random_color()
    for i in range(I):
        for j in range(J):
            _, pti, ptj = scene[i][j]
            if inside_triangle((pti, ptj), a, b, c):
                scene[i][j][0] = color

def random_triangle(f):
    center = (random(), random())
    rad = random() * f + 0.1
    theta1 = randint(0, 360)
    theta2 = randint(80, 150)
    theta3 = randint(80, 150)
    def polar(c, t, r):
        return add(c, mul(r, sincos(t)))
    return (polar(center, theta1, rad), polar(center, theta1+theta2, rad), polar(center, theta1+theta2+theta3, rad))

def random_circle(f):
    center = (random(), random())
    rad = random() * f + 0.1
    return (center, rad)

def random_stripe(midpt, horiz=False):
    if horiz:
        return ((midpt, 0.5), randint(-10, 10) + 90)
    else:
        return ((0.5, midpt), randint(-8, 8))

def right_of_stripe(midpt, tilt, pt):
    line = add(midpt, sincos(tilt))
    return sign(midpt, line, pt) < 0

def fill_stripe(midpt, tilt, scene):
    I = len(scene)
    J = len(scene[0])
    color = random_color()
    for i in range(I):
        for j in range(J):
            _, pti, ptj = scene[i][j]
            if right_of_stripe(midpt, tilt, (pti, ptj)):
                scene[i][j][0] = color

def random_spiral():
    r = random() * 0.1 + 0.05
    c = (randint(3, 7)/10, randint(3, 7)/10)
    return (c, r)

def inside_spiral(c, r, ratio, pt):
    ci, cj = c
    pti, ptj = pt
    di, dj = ci - pti, (cj - ptj)/ratio
    theta = phase(dj + di*1j)
    radius = (di**2 + dj**2)**.5 + theta / pi * r
    return int(radius/r) % 2 == 0

def fill_spiral(c, r, ratio, scene):
    I = len(scene)
    J = len(scene[0])
    color = random_color()
    for i in range(I):
        for j in range(J):
            _, pti, ptj = scene[i][j]
            if inside_spiral(c, r, ratio, (pti, ptj)):
                scene[i][j][0] = color

def random_lines():
    r = 0.1
    c = (0.5, 0.5)
    return (c, r)

def inside_lines(c, r, ratio, pt, fn, x):
    ci, cj = c
    pti, ptj = pt
    di, dj = ci - pti, (cj - ptj)/ratio
    theta = fn((di if x == 0 else dj)/((di**2 + dj**2)**.5))
    radius = (di**2 + dj**2)**.5 + theta
    return int(radius/r) % 2 == 0

def fill_lines(c, r, ratio, scene):
    I = len(scene)
    J = len(scene[0])
    if random() < 0.5:
        fn = [acos, asin][randint(0, 1)]
        x = randint(0, 1)
        color = random_color()
        for i in range(I):
            for j in range(J):
                _, pti, ptj = scene[i][j]
                if inside_lines(c, r, ratio, (pti, ptj), fn, x):
                    scene[i][j][0] = color
    else:
        x = randint(0, 1)
        color = random_color()
        for i in range(I):
            for j in range(J):
                _, pti, ptj = scene[i][j]
                if inside_lines(c, r, ratio, (pti, ptj), [acos, asin][randint(0, 1)], x):
                    scene[i][j][0] = color

HGT = 9
WTH = 5
DIG = {
    '0': (" ### \n#   #\n#   #\n#   #\n# # #\n#   #\n#   #\n## ##\n  #  ", 5),
    '1': (" ##  \n# #  \n  #  \n  #  \n  #  \n  #  \n  #  \n#####\n  #  ", 5),
    '2': (" ### \n#   #\n#   #\n   ##\n ##  \n#    \n#    \n## ##\n  #  ", 5),
    '3': (" ### \n#   #\n    #\n   ##\n  ## \n    #\n    #\n## ##\n  #  ", 5),
    '4': ("     \n#   #\n#   #\n##  #\n  ###\n    #\n    #\n    #\n     ", 5),
    '5': (" ### \n#   #\n#    \n##   \n  ## \n    #\n#   #\n## ##\n  #  ", 5),
    '6': (" ### \n#   #\n#   #\n#    \n#### \n#   #\n#   #\n## ##\n  #  ", 5),
    '7': (" ### \n#   #\n    #\n   ##\n    #\n    #\n    #\n    #\n     ", 5),
    '8': (" ### \n#   #\n#   #\n## ##\n ### \n#   #\n#   #\n## ##\n  #  ", 5),
    '9': (" ### \n#   #\n#   #\n## ##\n  # #\n    #\n#   #\n## ##\n  #  ", 5),
    'a': ("     \n     \n     \n ### \n#   #\n ####\n#   #\n## ##\n  #  ", 5),
    'A': (" ### \n#   #\n#   #\n#   #\n#####\n#   #\n#   #\n#   #\n#   #", 5),
    'b': ("#    \n#    \n#    \n#    \n#### \n#   #\n#   #\n## ##\n  #  ", 5),
    'c': ("     \n     \n     \n     \n ### \n#   #\n#    \n## ##\n  #  ", 5),
    'd': ("    #\n    #\n    #\n    #\n ####\n#   #\n#   #\n## ##\n  #  ", 5),
    'D': ("##   \n# ## \n#   #\n#   #\n#   #\n#   #\n#  ##\n###  \n#    ", 5),
    'e': ("     \n     \n     \n ### \n#   #\n## ##\n# #  \n## ##\n  #  ", 5),
    'F': (" ### \n#   #\n#    \n#    \n###  \n#    \n#    \n#    \n#    ", 5),
    'g': ("     \n     \n     \n     \n ### \n#   #\n#   #\n## ##\n  # #\n    #\n## ##\n  #", 5),
    'h': ("#    \n#    \n#    \n#    \n#### \n#   #\n#   #\n#   #\n#   #", 5),
    'i': ("     \n     \n  #  \n #   \n  #  \n  #  \n  #  \n#####\n  #  ", 5),
    'J': (" ### \n#   #\n    #\n    #\n #  #\n#   #\n#   #\n## ##\n  #  ", 5),
    'l': ("##   \n #   \n #   \n #   \n #   \n #   \n #   \n # ##\n  #  ", 5),
    'M': ("     \n## ##\n# # #\n# # #\n#   #\n#   #\n#   #\n#   #\n#   #", 5),
    'n': ("     \n     \n     \n     \n#### \n#   #\n#   #\n#   #\n#   #", 5),
    'N': ("#   #\n#   #\n##  #\n# ###\n#   #\n#   #\n#   #\n#   #\n#   #", 5),
    'o': ("     \n     \n     \n     \n ### \n#   #\n#   #\n## ##\n  #  ", 5),
    'O': (" ### \n#   #\n#   #\n#   #\n#   #\n#   #\n#   #\n## ##\n  #  ", 5),
    'p': ("     \n     \n     \n     \n ### \n#   #\n#   #\n## ##\n# #  \n#\n#\n#", 5),
    'r': ("     \n     \n     \n     \n#### \n#   #\n#    \n#    \n#    ", 5),
    'S': (" ### \n#   #\n#   #\n##   \n  ## \n    #\n#   #\n## ##\n  #  ", 5),
    't': ("#    \n#  # \n###  \n#    \n#    \n#    \n#    \n## ##\n  #  ", 5),
    'T': (" ### \n# # #\n  #  \n  #  \n  #  \n  #  \n  #  \n  #  \n  #  ", 5),
    'u': ("     \n     \n     \n     \n#   #\n#   #\n#   #\n## ##\n  #  ", 5),
    'v': ("     \n     \n     \n     \n#   #\n## ##\n # # \n  #  \n  #  ", 5),
    'W': ("#   #\n#   #\n#   #\n#   #\n#   #\n#   #\n# # #\n#####\n#   #", 5),
    'y': ("     \n     \n     \n     \n#   #\n#   #\n#   #\n## ##\n  # #\n    #\n## ##\n  #", 5),
    ':': (" \n \n \n#\n \n#\n \n \n ", 1),
    '.': (" \n \n \n \n \n \n \n \n#", 1),
    ' ': (" \n \n \n \n \n \n \n \n ", 1),
}

def draw_digit(scene, chr, i0, j0):
    i, j = i0, j0
    for c in chr:
        if c == '#':
            s = 100
            scene[i][j][0] = (s, s, s)
            j += 1
        elif c == '\n':
            i += 1
            j = j0
        else:
            j += 1

def add_timestamp(scene, time):
    I = len(scene)
    J = len(scene[0])
    Iref = I // 2 - HGT // 2 - 8
    Jref = J // 2 - 10
    for c in time:
        fmt, wth = DIG[c]
        draw_digit(scene, fmt, Iref, Jref)
        Jref += wth + 1

def make_date(date):
    w, m, d = list(map(int, date.strftime("%w %m %d").split(" ")))
    return "{}. {}. {}".format(
        ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"][w],
        ["", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"][m],
        d
        )


def add_date(scene, date):
    I = len(scene)
    J = len(scene[0])
    Iref = I // 2 + HGT // 2 + 4
    Jref = J // 2 - 24
    for c in date:
        fmt, wth = DIG[c]
        draw_digit(scene, fmt, Iref, Jref)
        Jref += wth + 1


size = 14
I = 43
J = 94

def choose_metatheme(now):
    if now < timetag(5, 0): return (0, 0, 30)
    elif now < timetag(5, 30): return (30, 0, 20)
    elif now < timetag(6, 0): return (60, 0, 20)
    elif now < timetag(6, 30): return (90, 50, 0)
    elif now < timetag(7, 0): return (80, 95, 0)
    elif now < timetag(8, 0): return (0, 75, 0)
    elif now < timetag(9, 0): return (60, 75, 0)
    elif now < timetag(10, 0): return (45, 100, 75)
    elif now < timetag(11, 0): return (0, 80, 100)
    elif now < timetag(12, 0): return (0, 100, 90)
    elif now < timetag(13, 0): return (0, 40, 100)
    elif now < timetag(14, 0): return (0, 100, 100)
    elif now < timetag(15, 0): return (20, 60, 100)
    elif now < timetag(16, 0): return (0, 0, 90)
    elif now < timetag(17, 0): return (30, 30, 80)
    elif now < timetag(18, 0): return (15, 15, 0)
    elif now < timetag(19, 0): return (90, 30, 0)
    elif now < timetag(20, 0): return (40, 0, 20)
    elif now < timetag(21, 0): return (0, 0, 50)
    elif now < timetag(22, 0): return (20, 0, 20)
    elif now < timetag(23, 0): return (0, 0, 20)
    else: return (0, 0, 0)

def main():
    now = timetag(int(datetime.now().strftime("%H")), int(datetime.now().strftime("%M")))

    metatheme = choose_metatheme(now)
    print("metatheme:", metatheme)

    theme1 = meanpoint(random_color(), metatheme, 0.3)
    theme2 = meanpoint(random_color(), metatheme, 0.3)
    print("themes:", theme1, theme2)

    max_i = calc_i(I, J)
    max_j = calc_j(I, J)
    scene = [[[theme1, calc_i(i, j)/max_i, calc_j(i, j)/max_j] for j in range(J)] for i in range(I)]

    style = randint(0, 2)
    if style == 0:
        # Triangles
        N = 20
        for n in range(N):
            fill_triangle(*random_triangle((1-n/N)/2), scene)
    elif style == 1:
        # Circles
        if random() < 0.33:
            # Everywhere
            N = 10
            for n in range(N):
                fill_circle(*random_circle(((1-n/N)/4 + 0.1)), max_i/max_j, scene)
        elif random() < 0.5:
            # Centered
            N = 10
            ci = randint(3, 7)/10
            cj = randint(3, 7)/10
            for n in range(N + N//2):
                fill_circle((ci, cj), 1-n/N, max_i/max_j, scene)
        else:
            N = 3
            for n in range(N):
                fill_spiral(*random_spiral(), max_i/max_j, scene)
    elif style == 2:
        # Stripes
        if random() < 0.5:
            # 1 direction
            N = 20
            h = random() < 0.5
            for n in range(N):
                fill_stripe(*random_stripe(1-n/N if h else n/N, horiz=h), scene)
        elif random() < 0.5:
            # Bidirectional
            N = 10
            for n in range(N):
                fill_stripe(*random_stripe(1-n/N, horiz=True), scene)
                fill_stripe(*random_stripe(n/N, horiz=False), scene)
        else:
            # Slanted
            N = 20
            for n in range(N):
                fill_stripe((n/N, n/N), randint(-50, -40), scene)
    else:
        print("Unimplemented")
        sys.exit(3)
    for i in range(50):
        scene[randint(0, I-1)][randint(0, J-1)][0] = random_color()

    for i in range(I):
        for j in range(J):
            scene[i][j][0] = color_adjust(meanpoint(color_variate(scene[i][j][0], 20), theme2, 0.3))

    add_timestamp(scene, datetime.now().strftime("%H:%M"))
    add_date(scene, make_date(datetime.today()))
    print("Done rendering, now saving image")
    draw_img('/tmp/wallpaper-random.svg', scene, theme2)

main()

# TODO:
# - waves
# - stripes
