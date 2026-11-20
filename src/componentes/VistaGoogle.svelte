<script>
  // Ficha de una familia de Google Fonts antes de instalarla, con el mismo
  // diseño que la de las instaladas (Especimen). Se abre con doble clic.
  import { ChevronLeft, Check } from "@lucide/svelte";
  import Corazon from "./Corazon.svelte";
  import { t } from "../lib/i18n.svelte.js";

  let { g, muestra, volver, instalada, pct, fav, onfav, oninstalar, onvolver } = $props();

  // «400», «700i»… → peso y cursiva, ordenados como en Google Fonts.
  const estilos = $derived(
    (g.variantes?.length ? g.variantes : ["400"])
      .map((v) => ({ peso: parseInt(v) || 400, cursiva: v.endsWith("i") }))
      .sort((a, b) => a.cursiva - b.cursiva || a.peso - b.peso),
  );
  const normal = $derived(estilos.find((e) => e.peso === 400 && !e.cursiva) ?? estilos[0]);
  const familia = $derived(`"${g.nombre}", var(--sans)`);
  const meta = $derived(
    `${g.estilos} ${g.estilos === 1 ? t("estilo") : t("estilos")} · ${t("cat." + g.categoria)}` + (g.disenadores?.length ? ` · ${g.disenadores.join(", ")}` : ""),
  );
  const juegos = $derived([
    { titulo: t("muestra.titulo"), chars: "ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz" },
    { titulo: t("numeros.solo"), chars: "0123456789 !?&@#%()" },
  ]);

  // La familia entera (sin &text=, que aquí se ve todo).
  $effect(() => {
    const ejes = [...new Set(estilos.map((e) => `${e.cursiva ? 1 : 0},${e.peso}`))].sort((a, b) => a.localeCompare(b, undefined, { numeric: true }));
    const l = document.createElement("link");
    l.rel = "stylesheet";
    l.href = `https://fonts.googleapis.com/css2?family=${encodeURIComponent(g.nombre).replace(/%20/g, "+")}:ital,wght@${ejes.join(";")}&display=swap`;
    document.head.appendChild(l);
    return () => l.remove();
  });

  const NOMBRES = { 100: "Thin", 200: "ExtraLight", 300: "Light", 400: "Regular", 500: "Medium", 600: "SemiBold", 700: "Bold", 800: "ExtraBold", 900: "Black" };
  const nombre = (e) => (e.peso === 400 && e.cursiva ? "Italic" : (NOMBRES[e.peso] ?? e.peso) + (e.cursiva ? " Italic" : ""));
</script>

<div class="esp">
  <button type="button" class="volver" onclick={onvolver}><ChevronLeft size={13} strokeWidth={2.2} />{volver}</button>

  <div class="cab">
    <div class="tit">
      <span class="meta">{meta}</span>
      <span class="nombre" style:font-family={familia} style:font-weight={normal.peso} style:font-style={normal.cursiva ? "italic" : "normal"}>{g.nombre}</span>
    </div>
    <button type="button" class="chip" onclick={onfav}>
      <span class="cor" class:fav><Corazon lleno={fav} tam={13} /></span>{fav ? t("fav.es") : t("fav.deseo")}
    </button>
    {#if pct != null}
      <span class="chip fijo num">{Math.round(pct)} %</span>
    {:else if instalada}
      <span class="chip fijo"><Check size={13} strokeWidth={2} />{t("g.instalada")}</span>
    {:else}
      <button type="button" class="chip principal" onclick={oninstalar}>{t("g.instalar")}</button>
    {/if}
  </div>

  <div class="escala">
    {#each [12, 18, 24, 36, 48, 72] as n (n)}
      <div class="paso">
        <span class="n num">{n}</span>
        <span class="linea" style:font-family={familia} style:font-weight={normal.peso} style:font-style={normal.cursiva ? "italic" : "normal"} style:font-size="{n}px">{muestra}</span>
      </div>
    {/each}
  </div>

  <div class="juegos">
    {#each juegos as j (j.titulo)}
      <div class="juego">
        <span class="etiqueta">{j.titulo}</span>
        <span class="chars" style:font-family={familia} style:font-weight={normal.peso}>{j.chars}</span>
      </div>
    {/each}
  </div>

  <div class="bloque">
    <span class="etiqueta">{t("estilos.titulo", { n: estilos.length })}</span>
    {#each estilos as e (e.peso + (e.cursiva ? "i" : ""))}
      <div class="estilo">
        <span class="n">{nombre(e)}</span>
        <span class="linea" style:font-family={familia} style:font-weight={e.peso} style:font-style={e.cursiva ? "italic" : "normal"} style:font-size="26px">{muestra}</span>
      </div>
    {/each}
  </div>
</div>

<style>
  /* Los mismos estilos que vistas/Especimen.svelte. */
  .esp { flex: 1; min-height: 0; overflow-y: auto; padding: 4px 40px 70px 24px; display: flex; flex-direction: column; gap: 34px; animation: tf-up 0.25s ease-out; }
  .esp > :global(*) { flex-shrink: 0; }
  .volver, .chip { height: 28px; padding: 0 12px 0 8px; border-radius: 999px; border: 0; background: var(--chip); color: var(--ink); font: 600 var(--fs-small) var(--sans); display: flex; align-items: center; gap: 4px; flex: none; }
  .volver { align-self: flex-start; }
  .chip { height: 30px; padding: 0 12px; gap: 6px; }
  .chip.principal { background: var(--ink); color: var(--win); padding: 0 14px; }
  .chip.fijo { color: var(--ink2); }
  .cor { display: flex; }
  .cor.fav { color: var(--red); }
  .cab { display: flex; align-items: flex-end; gap: 16px; }
  .tit { display: flex; flex-direction: column; gap: 6px; flex: 1; min-width: 0; }
  .meta { font-size: var(--fs-small); color: var(--ink3); }
  .nombre { font-size: 64px; line-height: 1.6; padding-top: 0.1em; overflow-wrap: anywhere; }
  .escala { display: flex; flex-direction: column; }
  .paso { display: grid; grid-template-columns: 40px minmax(0, 1fr); align-items: baseline; gap: 12px; padding: 4px 0; }
  .paso .n { font-size: var(--fs-meta); color: var(--ink3); }
  .linea { line-height: 1.38; white-space: nowrap; overflow-x: clip; overflow-y: visible; text-overflow: ellipsis; }
  .escala .linea { white-space: normal; overflow: visible; overflow-wrap: anywhere; line-height: 1.3; padding: 0.12em 0; margin: 0; }
  .juegos { display: flex; flex-direction: column; gap: 18px; }
  .juego { display: flex; flex-direction: column; gap: 4px; }
  .chars { font-size: 30px; line-height: 1.5; letter-spacing: 0.02em; word-break: break-all; }
  .bloque { display: flex; flex-direction: column; gap: 8px; }
  .estilo { display: flex; flex-direction: column; gap: 1px; padding: 6px 0; }
  .estilo .n { font-size: var(--fs-meta); color: var(--ink3); }
</style>
