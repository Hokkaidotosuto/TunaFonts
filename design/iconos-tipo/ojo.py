"""Coloca el ojo: un punto rojo en el centro del trazo de la cabeza (lado izquierdo del bucle de la α)."""
import os, math
from fontTools.ttLib import TTFont
from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.pens.boundsPen import BoundsPen
from fontTools.pens.transformPen import TransformPen
from fontTools.pens.pointInsidePen import PointInsidePen

D = os.path.dirname(os.path.abspath(__file__))
F = TTFont(os.path.join(D, '..', 'fuente-noto', 'noto-serif-bold.ttf'))
gs = F.getGlyphSet()
g = gs['alpha']
b = BoundsPen(gs); g.draw(b)
x0, y0, x1, y1 = b.bounds
CREMA, ROJO = '#fbf6ec', '#b8132c'
FONDO = open(os.path.join(D, '1-alfa.svg'), encoding='utf-8').read().split('\n  <path')[0].split('256">\n  ', 1)[1]

def matriz(w, h, cx, cy):
    sx, sy = w / (x1 - x0), h / (y1 - y0)
    mx, my = (x0 + x1) / 2, (y0 + y1) / 2
    A, B, C, Dd = sx, 0, 0, -sy
    return (A, B, C, Dd, cx - (A * mx + C * my), cy - (B * mx + Dd * my))

def dentro(m, X, Y):
    A, B, C, Dd, e, f = m
    gx, gy = (X - e) / A, (Y - f) / Dd
    p = PointInsidePen(gs, (gx, gy)); g.draw(p)
    return p.getResult()

def ojo(m):
    """El punto del tercio izquierdo más lejos del borde del trazo (y en la mitad de arriba)."""
    mejor = None
    for X in range(30, 120, 2):
        for Y in range(80, 135, 2):
            if not dentro(m, X, Y):
                continue
            r = 0
            while r < 30 and all(dentro(m, X + r * math.cos(t), Y + r * math.sin(t)) for t in [k * math.pi / 8 for k in range(16)]):
                r += 1
            if not mejor or r > mejor[2]:
                mejor = (X, Y, r)
    return mejor

def path(m):
    pen = SVGPathPen(gs); g.draw(TransformPen(pen, m)); return pen.getCommands()

for nombre, (w, h) in {'1b-alfa-ojo': (None, None), '1c-alfa-ojo': (200, 112)}.items():
    if w is None:  # 1b: proporciones de la letra, como la 1
        s = min(190 / (x1 - x0), 150 / (y1 - y0)); w, h = s * (x1 - x0), s * (y1 - y0)
    m = matriz(w, h, 128, 132 if nombre.startswith('1b') else 130)
    X, Y, r = ojo(m)
    radio = max(6, min(11, r * 0.55))
    svg = f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" width="256" height="256">\n  {FONDO}\n  <path d="{path(m)}" fill="{CREMA}"/>\n  <circle cx="{X}" cy="{Y}" r="{radio:.1f}" fill="{ROJO}"/>\n</svg>\n'
    open(os.path.join(D, nombre + '.svg'), 'w', encoding='utf-8').write(svg)
    print(nombre, X, Y, r, round(radio, 1))
