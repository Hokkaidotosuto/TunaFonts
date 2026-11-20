<script>
  // Vista previa grande de una familia de Google Fonts antes de instalarla:
  // todos sus estilos, con el texto que quieras. Se abre con doble clic.
  import { X, Check } from "@lucide/svelte";
  import { t } from "../lib/i18n.svelte.js";

  let { g, muestra, instalada, pct, oninstalar, oncerrar } = $props();

  let texto = $state(muestra);
  let tamano = $state(40);

  // «400», «700i»… → peso y cursiva, ordenados como en Google Fonts.
  const estilos = $derived(
    (g.variantes?.length ? g.variantes : ["400"])
      .map((v) => ({ peso: parseInt(v) || 400, cursiva: v.endsWith("i") }))
      .sort((a, b) => a.cursiva - b.cursiva || a.peso - b.peso),
  );

  // La familia entera (sin &text=, que aquí se escribe lo que se quiera).
  $effect(() => {
    const ejes = estilos.map((e) => `${e.cursiva ? 1 : 0},${e.peso}`);
    const unicos = [...new Set(ejes)].sort((a, b) => a.localeCompare(b, undefined, { numeric: true }));
    const l = document.createElement("link");
    l.rel = "stylesheet";
    l.href = `https://fonts.googleapis.com/css2?family=${encodeURIComponent(g.nombre).replace(/%20/g, "+")}:ital,wght@${unicos.join(";")}&display=swap`;
    document.head.appendChild(l);
    return () => l.remove();
  });

  const NOMBRES = { 100: "Thin", 200: "ExtraLight", 300: "Light", 400: "Regular", 500: "Medium", 600: "SemiBold", 700: "Bold", 800: "ExtraBold", 900: "Black" };
  const nombre = (e) => (e.peso === 400 && e.cursiva ? "Italic" : (NOMBRES[e.peso] ?? e.peso) + (e.cursiva ? " Italic" : ""));
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="fondo" onclick={oncerrar}>
  <div class="panel" onclick={(e) => e.stopPropagation()}>
    <div class="cab">
      <div class="tit">
        <h2>{g.nombre}</h2>
        <span class="meta">{t("cat." + g.categoria)} · {g.estilos} {g.estilos === 1 ? t("estilo") : t("estilos")}{#if g.disenadores?.length} · {g.disenadores.join(", ")}{/if}</span>
      </div>
      {#if pct != null}
        <span class="pct num">{Math.round(pct)} %</span>
      {:else if instalada}
        <span class="hecha"><Check size={13} strokeWidth={2} />{t("g.instalada")}</span>
      {:else}
        <button type="button" class="instalar" onclick={oninstalar}>{t("g.instalar")}</button>
      {/if}
      <button type="button" class="cerrar" title={t("close")} aria-label={t("close")} onclick={oncerrar}><X size={15} strokeWidth={2} /></button>
    </div>
    <div class="controles">
      <input bind:value={texto} placeholder={t("typeSomething")} spellcheck="false" />
      <input type="range" min="16" max="96" bind:value={tamano} aria-label={t("tamano")} />
    </div>
    <div class="lista">
      {#each estilos as e (e.peso + (e.cursiva ? "i" : ""))}
        <div class="estilo">
          <span class="nom">{nombre(e)}</span>
          <span
            class="muestra"
            style:font-family={`"${g.nombre}", var(--sans)`}
            style:font-weight={e.peso}
            style:font-style={e.cursiva ? "italic" : "normal"}
            style:font-size="{tamano}px">{texto || muestra}</span
          >
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .fondo { position: fixed; inset: 0; z-index: 20; display: flex; align-items: center; justify-content: center; padding: 32px; background: rgba(0, 0, 0, 0.32); animation: tf-fade 0.15s; }
  .panel { width: min(860px, 100%); max-height: 100%; display: flex; flex-direction: column; border-radius: 16px; background: var(--win); color: var(--ink); box-shadow: 0 24px 60px rgba(0, 0, 0, 0.25), 0 0 0 1px var(--line2); overflow: hidden; }
  .cab { display: flex; align-items: center; gap: 12px; padding: 18px 18px 12px 24px; }
  .tit { flex: 1; min-width: 0; display: flex; flex-direction: column; gap: 3px; }
  h2 { margin: 0; font: 600 var(--fs-title) var(--sans); }
  .meta { font-size: var(--fs-small); color: var(--ink3); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .instalar { height: 30px; padding: 0 14px; border: 0; border-radius: 999px; background: var(--ink); color: var(--win); font: 600 var(--fs-small) var(--sans); }
  .hecha { display: flex; align-items: center; gap: 6px; font: 600 var(--fs-small) var(--sans); color: var(--ink3); }
  .pct { font: 600 var(--fs-small) var(--sans); color: var(--ink3); }
  .cerrar { width: 30px; height: 30px; border: 0; border-radius: 50%; background: var(--chip); color: var(--ink2); display: flex; align-items: center; justify-content: center; }
  .cerrar:hover { background: var(--hov); color: var(--ink); }
  .controles { display: flex; align-items: center; gap: 14px; padding: 0 24px 12px; border-bottom: 1px solid var(--line); }
  .controles input:not([type="range"]) { flex: 1; height: 32px; padding: 0 10px; border-radius: 8px; border: 0; box-shadow: 0 0 0 1px var(--line2); background: var(--field); font: 500 var(--fs-body) var(--sans); color: var(--ink); outline: 0; }
  .controles input[type="range"] { width: 140px; accent-color: var(--red); }
  .lista { overflow-y: auto; padding: 6px 24px 20px; }
  .estilo { display: flex; flex-direction: column; gap: 2px; padding: 12px 0; border-bottom: 1px solid var(--line); }
  .estilo:last-child { border-bottom: 0; }
  .nom { font-size: var(--fs-meta); color: var(--ink3); }
  .muestra { line-height: 1.3; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; padding-top: 0.1em; }
</style>
