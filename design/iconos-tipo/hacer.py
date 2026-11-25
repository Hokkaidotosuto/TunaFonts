"""Atunes tipográficos: la letra ES el atún. Glifos de Noto Serif Bold (OFL)."""
import os
from fontTools.ttLib import TTFont
from fontTools.pens.svgPathPen import SVGPathPen
from fontTools.pens.boundsPen import BoundsPen
from fontTools.pens.transformPen import TransformPen

D = os.path.dirname(os.path.abspath(__file__))
F = TTFont(os.path.join(D, '..', 'fuente-noto', 'noto-serif-bold.ttf'))
gs = F.getGlyphSet()

CREMA, OJO = '#fbf6ec', '#b8132c'
FONDO = '''<defs>
    <linearGradient id="bg" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="#ef3b4f"/><stop offset="1" stop-color="#9b0b22"/></linearGradient>
    <linearGradient id="shine" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#fff" stop-opacity="0.18"/><stop offset="1" stop-color="#fff" stop-opacity="0"/></linearGradient>
  </defs>
  <rect x="8" y="8" width="240" height="240" rx="58" fill="url(#bg)"/>
  <rect x="8" y="8" width="240" height="118" rx="58" fill="url(#shine)"/>
  <rect x="8.5" y="8.5" width="239" height="239" rx="57.5" fill="none" stroke="#fff" stroke-opacity="0.12"/>'''

def glifo(nombre, ancho=None, alto=None, cx=128, cy=128, giro=0):
    """Trazado SVG del glifo, girado `giro` grados (múltiplos de 90), escalado y centrado."""
    import math
    g = gs[nombre]
    b = BoundsPen(gs); g.draw(b)
    x0, y0, x1, y1 = b.bounds
    w, h = x1 - x0, y1 - y0
    if giro % 180:
        w, h = h, w
    s = min((ancho or 1e9) / w, (alto or 1e9) / h)
    # Fuente: y hacia arriba → SVG: y hacia abajo. Se centra el glifo en el origen, se gira, se escala y se lleva a (cx, cy).
    mx, my = (x0 + x1) / 2, (y0 + y1) / 2
    a = math.radians(giro)
    ca, sa = round(math.cos(a)), round(math.sin(a))
    # p' = S * R * flip * (p - m) + c
    # flip: (x, y) -> (x, -y); R(giro, en el sentido de las agujas en pantalla): (x, y) -> (x ca - y sa, x sa + y ca)
    xx, xy = s * ca, s * sa
    yx, yy = -s * -sa, -s * ca
    # Matriz afín (a, b, c, d, e, f) para TransformPen: x' = a x + c y + e ; y' = b x + d y + f
    t = (xx, xy, yx, yy, 0, 0)
    e = cx - (t[0] * mx + t[2] * my)
    f = cy - (t[1] * mx + t[3] * my)
    pen = SVGPathPen(gs)
    g.draw(TransformPen(pen, (t[0], t[1], t[2], t[3], e, f)))
    return pen.getCommands(), s

def svg(cuerpo):
    return f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" width="256" height="256">\n  {FONDO}\n  {cuerpo}\n</svg>\n'

op = {}
# 1 · α: el bucle es el cuerpo y las dos puntas cruzadas, la cola.
alfa, _ = glifo('alpha', ancho=190, alto=150, cx=128, cy=132)
op['1-alfa'] = svg(f'<path d="{alfa}" fill="{CREMA}" fill-rule="nonzero"/>')
# 1b · la misma α con ojo.
op['1b-alfa-ojo'] = svg(f'<path d="{alfa}" fill="{CREMA}"/><circle cx="74" cy="118" r="9" fill="{CREMA}"/>')
# 2 · la «a» de siempre (Noto Serif) con cola en media luna: el ojo de la «a» es el ojo del pez.
a, _ = glifo('a', alto=150, cx=104, cy=132)
cola = 'M150 132 C164 112 180 96 204 84 C200 108 192 122 182 132 C192 142 200 156 204 180 C180 168 164 152 150 132 Z'
op['2-a-cola'] = svg(f'<path d="{a}" fill="{CREMA}"/><path d="{cola}" fill="{CREMA}"/>')
# 3 · la «t» tumbada: el palo es el cuerpo, la cruz son las aletas y el remate curvo, la cola.
tt, _ = glifo('t', ancho=196, alto=120, cx=128, cy=128, giro=-90)
op['3-t-tumbada'] = svg(f'<path d="{tt}" fill="{CREMA}"/><circle cx="58" cy="124" r="8" fill="{OJO}"/>')

for n, s in op.items():
    open(os.path.join(D, n + '.svg'), 'w', encoding='utf-8').write(s)
actual = open(r'C:\Users\tun1x\Documents\APPS\tuna-fonts\public\icon.svg', encoding='utf-8').read()
open(os.path.join(D, 'actual.svg'), 'w', encoding='utf-8').write(actual)

filas = [('actual', 'actual (la «a»)'), ('1-alfa', '1 · α'), ('1b-alfa-ojo', '1b · α con ojo'), ('2-a-cola', '2 · «a» con cola'), ('3-t-tumbada', '3 · «t» tumbada')]
html = '''<!doctype html><meta charset="utf-8"><style>
body{margin:0;font:600 14px "Segoe UI",system-ui;background:#fff;color:#222}
table{border-collapse:collapse}
td{padding:8px 16px;vertical-align:middle;border-bottom:1px solid #eee}
th{font:600 12px "Segoe UI";color:#888;text-align:left;padding:10px 16px}
.barra{display:flex;align-items:center;gap:10px;padding:6px 12px;border-radius:8px}
.clara{background:#eef1f6}.oscura{background:#1c1c1c}
.n{width:150px} img{display:block}
</style><table><tr><th></th><th>256</th><th>48</th><th>32</th><th>24</th><th>16</th><th>barra clara</th><th>barra oscura</th></tr>'''
for fi, n in filas:
    html += f'<tr><td class="n">{n}</td>' + ''.join(f'<td><img src="{fi}.svg" width="{s}" height="{s}"></td>' for s in (96, 48, 32, 24, 16))
    for c in ('clara', 'oscura'):
        html += f'<td><div class="barra {c}">' + ''.join(f'<img src="{x}" width="24" height="24">' for x in ('otro1.svg', f'{fi}.svg', 'otro2.svg')) + '</div></td>'
    html += '</tr>'
open(os.path.join(D, 'hoja.html'), 'w', encoding='utf-8').write(html + '</table>')
open(os.path.join(D, 'otro1.svg'), 'w').write('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><rect x="2" y="2" width="20" height="20" rx="5" fill="#2f6fde"/><rect x="7" y="7" width="10" height="10" rx="2" fill="#fff"/></svg>')
open(os.path.join(D, 'otro2.svg'), 'w').write('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10" fill="#f2b233"/><circle cx="12" cy="12" r="4" fill="#fff"/></svg>')
print('ok')
