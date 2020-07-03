#!/usr/bin/python3

import math
from random import randint, random
def cos(n):
    return math.cos(math.radians(n))

def sin(n):
    return math.sin(math.radians(n))

cos60 = cos(60)
sin60 = sin(60)

def sincos(theta):
    return (cos(theta), sin(theta))
def meanpoint(add, base, weight):
    return tuple(a*weight + b*(1-weight) for (a, b) in zip(add, base))

def random_color():
    return (randint(0, 100), randint(0, 100), randint(0, 100))
def color_adjust(rgb):
    return (max(0, min(100, c)) for c in rgb)

def color_variate(rgb, amount):
    return (c+randint(0, amount) for c in rgb)

# def draw_hex(size, centre, color, drawing):
#     adjactent = size * cos60
#     opposite = size * sin60
#     half = size / 2.0
#     top_left = (centre[0] - half, centre[1] - opposite)
#     top_right = (centre[0] + half, centre[1] - opposite)
#     left = (centre[0] - (half + adjactent), centre[1])
#     right = (centre[0] + (half + adjactent), centre[1])
#     bottom_left = (centre[0] - half, centre[1] + opposite)
#     bottom_right = (centre[0] + half, centre[1] + opposite)
#
#     points=[top_left, top_right, right, bottom_right, bottom_left, left]
#     hex = svg.shapes.Polygon(points,
#                 stroke=svg.rgb(0, 0, 0, '%'),
#                 stroke_width=1,
#                 stroke_opacity=100,
#                 fill=color,
#                 fill_opacity=100)
#     drawing.add(hex)

def fmt_hex(size, center, color):
    adjactent = size * cos60
    opposite = size * sin60
    half = size / 2.0
    top_left = (center[0] - half, center[1] - opposite)
    top_right = (center[0] + half, center[1] - opposite)
    left = (center[0] - (half + adjactent), center[1])
    right = (center[0] + (half + adjactent), center[1])
    bottom_left = (center[0] - half, center[1] + opposite)
    bottom_right = (center[0] + half, center[1] + opposite)

    return "<polygon fill=\"rgb({:.0f}%,{:.0f}%,{:.0f}%)\" fill-opacity=\"100\" points=\"{:.2f},{:.2f} {:.2f},{:.2f} {:.2f},{:.2f} {:.2f},{:.2f} {:.2f},{:.2f} {:.2f},{:.2f}\" stroke=\"rgb(0%,0%,0%)\" stroke-opacity=\"100\" stroke-width=\"1\" />".format(*color, *top_left, *top_right, *right, *bottom_right, *bottom_left, *left)
size = 14
I = 43
J = 94
def calc_i(i, j):
    return (2*i+(1 if j%2==0 else 0))*size*sin60

def calc_j(i, j):
    return 3*j*size*cos60

def draw_img(name, scene, theme):
    with open(name, 'w') as f:
        f.write("""<?xml version="1.0" encoding="utf-8" ?>
<svg baseProfile="full" height="1000" version="1.1" width="2100" xmlns="http://www.w3.org/2000/svg" xmlns:ev="http://www.w3.org/2001/xml-events" xmlns:xlink="http://www.w3.org/1999/xlink"><defs />""")
        #dwg = svg.Drawing(filename=name, debug=False, size=(2100, 1000))
        for i in range(I):
            for j in range(J):
                offset = 0 if j%2==0 else 1
                color = scene[i][j][0]
                pti, ptj = calc_i(i, j), calc_j(i, j)
                #draw_hex(size, (ptj, pti), svg.rgb(*color, '%'), dwg)
                f.write(fmt_hex(size, (ptj, pti), color))
        f.write("""</svg>""")
    #dwg.save()
def main():
    max_i = calc_i(I, J)
    max_j = calc_j(I, J)
    scene = [[[theme1, calc_i(i, j)/max_i, calc_j(i, j)/max_j] for j in range(J)] for i in range(I)]
    for i in range(50):
        scene[randint(0, I-1)][randint(0, J-1)][0] = random_color()

    for i in range(I):
        for j in range(J):
            scene[i][j][0] = color_adjust(meanpoint(color_variate(scene[i][j][0], 20), theme2, 0.3))
    print("Done rendering, now saving image")
    draw_img('img.svg', scene, theme2)

main()
