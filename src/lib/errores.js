// Los mensajes de error que vienen de Rust están en español. Aquí se pasan al
// idioma de la interfaz los que conocemos; lo demás (errores de Windows, que ya
// salen en el idioma del sistema) se deja tal cual.
import { t, idioma } from "./i18n.svelte.js";

const FRASES = [
  ["no se ha podido mandar a la papelera", "errR.papelera"],
  ["algún programa la tiene abierta", "errR.abierta"],
  ["windows la tiene en uso", "errR.enUso"],
  ["windows la necesita", "errR.necesaria"],
  ["ruta no permitida", "errR.ruta"],
  ["no es un archivo de fuente", "errR.noFuente"],
  ["no es una fuente", "errR.noFuente"],
  ["el archivo no trae ninguna fuente", "errR.sinFuentes"],
  ["no trae ninguna fuente", "errR.sinFuentes"],
  ["el .zip está dañado", "errR.zipRoto"],
  ["el .zip es demasiado grande", "errR.zipGrande"],
  ["el .zip tiene demasiados archivos", "errR.zipGrande"],
  ["el archivo es demasiado grande", "errR.grande"],
  ["no se ha podido leer", "errR.leer"],
  ["sin tabla name", "errR.leer"],
  ["sin tabla cmap", "errR.leer"],
  ["cabecera corta", "errR.leer"],
  ["no se ha podido terminar con permisos de administrador", "errR.admin"],
  ["solo se abren direcciones web", "errR.web"],
];

export function traducirError(texto) {
  if (typeof texto !== "string" || !texto || idioma() === "es") return texto;
  let s = texto;
  for (const [frase, clave] of FRASES) if (s.includes(frase)) s = s.replace(frase, t(clave));
  return s;
}
