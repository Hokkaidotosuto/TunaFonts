// Favoritas y colecciones, guardadas con tauri-plugin-store en biblioteca.json.
import { LazyStore } from "@tauri-apps/plugin-store";
import { enTauri } from "./ajustes.svelte.js";

export const biblioteca = $state({
  favoritas: [],
  // [{ id, nombre, fuentes: [familia] }]
  colecciones: [],
  listo: false,
});

const store = enTauri ? new LazyStore("biblioteca.json", { autoSave: 150 }) : null;

export async function iniciarBiblioteca() {
  if (store) {
    biblioteca.favoritas = (await store.get("favoritas")) ?? [];
    biblioteca.colecciones = (await store.get("colecciones")) ?? [];
  }
  biblioteca.listo = true;
}

$effect.root(() => {
  $effect(() => {
    const f = $state.snapshot(biblioteca.favoritas);
    if (biblioteca.listo) store?.set("favoritas", f);
  });
  $effect(() => {
    const c = $state.snapshot(biblioteca.colecciones);
    if (biblioteca.listo) store?.set("colecciones", c);
  });
});

export const esFavorita = (n) => biblioteca.favoritas.includes(n);

/** La biblioteca como texto, para guardarla en un archivo y llevarla a otro PC. */
export const copiaBiblioteca = () =>
  JSON.stringify({ app: "tuna-fonts", tipo: "biblioteca", version: 1, favoritas: $state.snapshot(biblioteca.favoritas), colecciones: $state.snapshot(biblioteca.colecciones).map(({ nombre, fuentes }) => ({ nombre, fuentes })) }, null, 2);

/** Una sola colección, con el mismo formato (se importa igual, desde Ajustes). */
export function copiaColeccion(id) {
  const c = biblioteca.colecciones.find((x) => x.id === id);
  return JSON.stringify({ app: "tuna-fonts", tipo: "coleccion", version: 1, favoritas: [], colecciones: c ? [{ nombre: c.nombre, fuentes: $state.snapshot(c.fuentes) }] : [] }, null, 2);
}

/**
 * Mezcla una copia con lo que ya hay: suma favoritas y, en las colecciones con
 * el mismo nombre, suma fuentes; las demás se crean. Nunca borra nada.
 * Devuelve { colecciones, favoritas } añadidas, o null si no es una copia válida.
 */
export function mezclarCopia(texto) {
  let d;
  try {
    d = JSON.parse(texto);
  } catch {
    return null;
  }
  if (d?.app !== "tuna-fonts" || !Array.isArray(d.colecciones) || !Array.isArray(d.favoritas)) return null;
  const nombres = (l) => (Array.isArray(l) ? l.filter((x) => typeof x === "string" && x.length < 300) : []);
  let favoritas = 0;
  for (const n of nombres(d.favoritas)) if (!biblioteca.favoritas.includes(n)) (biblioteca.favoritas.push(n), favoritas++);
  let colecciones = 0;
  for (const c of d.colecciones) {
    if (typeof c?.nombre !== "string" || !c.nombre.trim()) continue;
    let mia = biblioteca.colecciones.find((x) => x.nombre.toLowerCase() === c.nombre.trim().toLowerCase());
    if (!mia) {
      nuevaColeccion(c.nombre.trim().slice(0, 80));
      mia = biblioteca.colecciones.at(-1);
      colecciones++;
    }
    for (const n of nombres(c.fuentes)) if (!mia.fuentes.includes(n)) mia.fuentes.push(n);
  }
  return { colecciones, favoritas };
}

export function alternarFavorita(n) {
  const i = biblioteca.favoritas.indexOf(n);
  if (i >= 0) biblioteca.favoritas.splice(i, 1);
  else biblioteca.favoritas.push(n);
}

export function aFavoritas(nombres) {
  for (const n of nombres) if (!biblioteca.favoritas.includes(n)) biblioteca.favoritas.push(n);
}

export function nuevaColeccion(nombre) {
  const id = "c" + Date.now().toString(36);
  biblioteca.colecciones.push({ id, nombre, fuentes: [] });
  return id;
}

export function renombrar(id, nombre) {
  const c = biblioteca.colecciones.find((c) => c.id === id);
  if (c && nombre.trim()) c.nombre = nombre.trim();
}

export function eliminarColeccion(id) {
  const i = biblioteca.colecciones.findIndex((c) => c.id === id);
  if (i >= 0) biblioteca.colecciones.splice(i, 1);
}

/** Añade fuentes a una colección; devuelve cuántas eran nuevas. */
export function aColeccion(id, nombres) {
  const c = biblioteca.colecciones.find((c) => c.id === id);
  if (!c) return 0;
  let n = 0;
  for (const x of nombres) {
    if (!c.fuentes.includes(x)) {
      c.fuentes.push(x);
      n++;
    }
  }
  return n;
}
