// Toasts: abajo a la derecha, como mucho tres a la vez, se van solos.
// tipo: ok | warn | err
import { traducirError } from "./errores.js";

export const avisos = $state([]);

let siguiente = 0;

export function avisar(tipo, titulo, texto = "") {
  const id = ++siguiente;
  avisos.push({ id, tipo, titulo, texto: tipo === "err" ? traducirError(texto) : texto });
  while (avisos.length > 3) avisos.shift();
  setTimeout(() => quitarAviso(id), 3600);
}

export function quitarAviso(id) {
  const i = avisos.findIndex((a) => a.id === id);
  if (i >= 0) avisos.splice(i, 1);
}
