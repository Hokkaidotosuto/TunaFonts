<script>
  // Espécimen: escala de tamaños, juegos de caracteres, el mapa de caracteres
  // real de la fuente (tabla cmap; clic para copiar), los estilos y la ruta.
  import { ChevronLeft, FolderOpen } from "@lucide/svelte";
  import Corazon from "../componentes/Corazon.svelte";
  import { estiloDe, caracteres, mostrarEnExplorador, llamar } from "../lib/fuentes.svelte.js";
  import { avisar } from "../lib/avisos.svelte.js";
  import { t } from "../lib/i18n.svelte.js";

  let { f, muestra, fav, bloqueada, volver, onvolver, onalternar, onfav } = $props();


  const e = $derived(estiloDe(f));

  // Lo que dice el archivo de sí mismo: autor, licencia, versión…
  let info = $state(null);
  let licenciaEntera = $state(false);
  $effect(() => {
    const ruta = f.archivo;
    info = null;
    licenciaEntera = false;
    if (f.error) return;
    llamar("info_fuente", { ruta }).then((i) => { if (ruta === f.archivo) info = i; }).catch(() => {});
  });
  // Alfabetos que cubre, contando los caracteres de verdad del archivo (cmap).
  const ESCRITURAS = [
    ["latino", 0x41, 0x7a, 40], ["latinoAmp", 0x100, 0x24f, 80], ["griego", 0x370, 0x3ff, 40], ["cirilico", 0x400, 0x4ff, 40],
    ["armenio", 0x530, 0x58f, 30], ["hebreo", 0x590, 0x5ff, 20], ["arabe", 0x600, 0x6ff, 30], ["devanagari", 0x900, 0x97f, 40],
    ["tailandes", 0xe00, 0xe7f, 40], ["georgiano", 0x10a0, 0x10ff, 30], ["kana", 0x3040, 0x30ff, 60], ["han", 0x4e00, 0x9fff, 200], ["hangul", 0xac00, 0xd7af, 200],
  ];
  const escrituras = $derived.by(() => {
    if (!glifos.length) return [];
    const n = {};
    for (const g of glifos) {
      const c = g.codePointAt(0);
      for (const [k, a, b] of ESCRITURAS) if (c >= a && c <= b) n[k] = (n[k] ?? 0) + 1;
    }
    return ESCRITURAS.filter(([k, , , min]) => (n[k] ?? 0) >= min).map(([k]) => k);
  });
  const comoSale = $derived([...new Set(f.estilos.map((s) => s.familia_win).filter(Boolean))]);
  const abrirEnlace = (url) => llamar("abrir_enlace_fuente", { url }).catch(() => {});
  const enlace = (s) => (s && /^https?:\/\//i.test(s.trim()) ? s.trim() : null);
  // Funciones OpenType con su demostración (las demás solo se nombran).
  const RASGOS = ["smcp", "c2sc", "onum", "lnum", "tnum", "pnum", "liga", "dlig", "swsh", "salt", "ss", "frac", "zero"];
  const rasgos = $derived(RASGOS.filter((r) => f.rasgos?.includes(r)));
  const demos = $derived(
    [
      f.rasgos?.includes("smcp") && { titulo: t("ras.smcp"), css: '"smcp"', texto: muestra },
      f.rasgos?.includes("onum") && { titulo: t("ras.onum"), css: '"onum"', texto: "0123456789" },
      f.rasgos?.includes("dlig") && { titulo: t("ras.dlig"), css: '"dlig"', texto: "ct st sp Th fi fl ffi" },
      f.rasgos?.includes("swsh") && { titulo: t("ras.swsh"), css: '"swsh"', texto: muestra },
      f.rasgos?.includes("frac") && { titulo: t("ras.frac"), css: '"frac"', texto: "1/2 3/4 5/8" },
    ].filter(Boolean),
  );
  const otros = $derived(f.categoria === "otros");
  const meta = $derived(
    `${f.estilos.length} ${f.estilos.length === 1 ? t("estilo") : t("estilos")} · ${t("origen." + f.origen)} · ${t("cat." + f.categoria)}` +
      (f.categoriaAuto ? ` (${t("categoria.aMano")})` : ""),
  );
  const juegos = $derived(
    otros
      ? [{ titulo: t("muestra.titulo"), chars: muestra }, { titulo: t("numeros.solo"), chars: "0123456789" }]
      : [
          { titulo: t("mayusculas"), chars: "ABCDEFGHIJKLMNÑOPQRSTUVWXYZ" },
          { titulo: t("minusculas"), chars: "abcdefghijklmnñopqrstuvwxyz" },
          { titulo: t("numeros"), chars: "0123456789 ¿?¡!@#€%&()«»" },
        ],
  );

  let glifos = $state([]);
  $effect(() => {
    const ruta = f.archivo;
    glifos = [];
    caracteres(ruta)
      .then((cs) => ruta === f.archivo && (glifos = cs.map((c) => String.fromCodePoint(c))))
      .catch(() => {});
  });

  async function copiar(c) {
    try {
      await navigator.clipboard.writeText(c);
    } catch {
      const a = document.createElement("textarea");
      a.value = c;
      document.body.appendChild(a);
      a.select();
      document.execCommand("copy");
      a.remove();
    }
    avisar("ok", t("ok.copiado", { c }), t("ok.pegalo"));
  }

</script>

<div class="esp">
  <button type="button" class="volver" onclick={onvolver}><ChevronLeft size={13} strokeWidth={2.2} />{volver}</button>

  <div class="cab">
    <div class="tit">
      <span class="meta">{meta}</span>
      <span class="nombre" style:font-family={e.familia} style:font-weight={e.peso} style:font-style={e.cursiva ? "italic" : "normal"}>{f.nombre}</span>
    </div>
    <button type="button" class="chip" onclick={onfav}>
      <span class="cor" class:fav><Corazon lleno={fav} tam={13} /></span>{fav ? t("fav.es") : t("fav.poner")}
    </button>
    {#if bloqueada}
      <span class="chip fijo">{t("alwaysOn")}</span>
    {:else}
      <button type="button" class="chip" class:principal={!f.activa} onclick={onalternar}>{f.activa ? t("deactivate") : t("activate")}</button>
    {/if}
  </div>

  <div class="escala">
    {#each [12, 18, 24, 36, 48, 72] as n (n)}
      <div class="paso">
        <span class="n num">{n}</span>
        <span class="linea" style:font-family={e.familia} style:font-weight={e.peso} style:font-style={e.cursiva ? "italic" : "normal"} style:font-size="{n}px">{muestra}</span>
      </div>
    {/each}
  </div>

  <div class="juegos">
    {#each juegos as j (j.titulo)}
      <div class="juego">
        <span class="etiqueta">{j.titulo}</span>
        <span class="chars" style:font-family={e.familia} style:font-weight={e.peso} style:font-style={e.cursiva ? "italic" : "normal"}>{j.chars}</span>
      </div>
    {/each}
  </div>

  {#if glifos.length}
    <div class="bloque">
      <span class="etiqueta">{t("allChars")} · {glifos.length}</span>
      <div class="glifos">
        {#each glifos as c, i (i)}
          <button type="button" class="glifo" title={t("copiar") + " U+" + c.codePointAt(0).toString(16).toUpperCase().padStart(4, "0")} style:font-family={e.familia} style:font-weight={e.peso} style:font-style={e.cursiva ? "italic" : "normal"} onclick={() => copiar(c)}>{c}</button>
        {/each}
      </div>
    </div>
  {/if}

  <div class="bloque">
    <span class="etiqueta">{t("estilos.titulo", { n: f.estilos.length })}</span>
    {#each f.estilos as s (s.nombre + s.peso + s.cursiva)}
      <div class="estilo">
        <span class="n">{s.nombre}</span>
        <span class="linea" style:font-family={estiloDe(f, s).familia} style:font-weight={estiloDe(f, s).peso} style:font-style={estiloDe(f, s).cursiva ? "italic" : "normal"} style:font-size="26px">{muestra}</span>
      </div>
    {/each}
  </div>

  {#if escrituras.length}
    <div class="bloque">
      <span class="etiqueta">{t("esc.titulo", { n: escrituras.length })}</span>
      <div class="rasgos">
        {#each escrituras as k (k)}<span class="rasgo">{t("esc." + k)}</span>{/each}
      </div>
    </div>
  {/if}

  {#if f.duplicados?.length}
    <div class="bloque aviso-dup">
      <span class="etiqueta">{t("dup.titulo")}</span>
      <span class="pista-dup">{t("dup.texto")}</span>
      {#each f.duplicados as a (a)}
        <button type="button" class="archivo" title={t("ruta.mostrar")} onclick={() => mostrarEnExplorador(a)}><span class="txt">{a}</span><FolderOpen size={13} strokeWidth={1.9} /></button>
      {/each}
    </div>
  {/if}

  {#if rasgos.length}
    <div class="bloque">
      <span class="etiqueta">{t("rasgos.titulo", { n: rasgos.length })}</span>
      <div class="rasgos">
        {#each rasgos as r (r)}<span class="rasgo" title={t("ras." + r + "Pista")}><code>{r}</code>{t("ras." + r)}</span>{/each}
      </div>
      {#each demos as d (d.titulo)}
        <div class="estilo">
          <span class="n">{d.titulo}</span>
          <span class="linea" style:font-family={e.familia} style:font-weight={e.peso} style:font-style={e.cursiva ? "italic" : "normal"} style:font-size="26px" style:font-feature-settings={d.css}>{d.texto}</span>
        </div>
      {/each}
    </div>
  {/if}

  {#if info}
    <div class="bloque">
      <span class="etiqueta">{t("info.titulo")}</span>
      <dl class="info">
        {#if comoSale.length}<dt>{t("info.comoSale")}</dt><dd>{comoSale.join(", ")}</dd>{/if}
        {#if info.disenador}<dt>{t("info.disenador")}</dt><dd>{info.disenador}</dd>{/if}
        {#if info.fabricante}<dt>{t("info.fabricante")}</dt><dd>{info.fabricante}</dd>{/if}
        {#if info.version}<dt>{t("info.version")}</dt><dd>{info.version}</dd>{/if}
        {#if enlace(info.url)}<dt>{t("info.web")}</dt><dd><button type="button" class="enlace" onclick={() => abrirEnlace(enlace(info.url))}>{enlace(info.url)}</button></dd>{/if}
        {#if info.licencia || enlace(info.licencia_url)}
          <dt>{t("info.licencia")}</dt>
          <dd>
            {#if info.licencia}
              <span class="largo" class:entero={licenciaEntera}>{info.licencia}</span>
              {#if info.licencia.length > 240}<button type="button" class="enlace" onclick={() => (licenciaEntera = !licenciaEntera)}>{licenciaEntera ? t("info.verMenos") : t("info.verMas")}</button>{/if}
            {/if}
            {#if enlace(info.licencia_url)}<button type="button" class="enlace" onclick={() => abrirEnlace(enlace(info.licencia_url))}>{enlace(info.licencia_url)}</button>{/if}
          </dd>
        {/if}
        {#if info.copyright}<dt>{t("info.copyright")}</dt><dd><span class="largo">{info.copyright}</span></dd>{/if}
      </dl>
    </div>
  {/if}

  <div class="ruta">
    {#each f.archivos as a (a)}
      <button type="button" class="archivo" title={t("ruta.mostrar")} onclick={() => mostrarEnExplorador(a)}>
        <span class="txt">{a}</span>
        <FolderOpen size={13} strokeWidth={1.9} />
      </button>
    {/each}
  </div>
</div>

<style>
  .esp { flex: 1; min-height: 0; overflow-y: auto; padding: 4px 40px 70px 24px; display: flex; flex-direction: column; gap: 34px; animation: tf-up 0.25s ease-out; }
  .esp > :global(*) { flex-shrink: 0; }
  .volver, .chip {
    height: 28px;
    padding: 0 12px 0 8px;
    border-radius: 999px;
    border: 0;
    background: var(--chip);
    color: var(--ink);
    font: 600 var(--fs-small) var(--sans);
    display: flex;
    align-items: center;
    gap: 4px;
    flex: none;
  }
  .volver { align-self: flex-start; }
  .chip { height: 30px; padding: 0 12px; gap: 6px; }
  .chip.principal { background: var(--ink); color: var(--win); padding: 0 14px; }
  .chip.fijo { color: var(--ink2); }
  .cor { display: flex; }
  .cor.fav { color: var(--red); }
  .cab { display: flex; align-items: flex-end; gap: 16px; }
  .tit { display: flex; flex-direction: column; gap: 6px; flex: 1; min-width: 0; }
  .meta { font-size: var(--fs-small); color: var(--ink3); }
  /* El título no se recorta: las caligráficas tienen adornos que suben y bajan
     mucho más que la línea. Interlineado amplio, sin overflow, y si no cabe a lo
     ancho pasa a la línea siguiente. */
  .nombre { font-size: 64px; line-height: 1.6; padding-top: 0.1em; overflow-wrap: anywhere; }
  .escala { display: flex; flex-direction: column; }
  .paso { display: grid; grid-template-columns: 40px minmax(0, 1fr); align-items: baseline; gap: 12px; padding: 4px 0; }
  .paso .n { font-size: var(--fs-meta); color: var(--ink3); }
  /* Recorta solo a lo ancho (para los «…»): por arriba y por abajo los adornos de las caligráficas pueden salirse. */
  .linea { line-height: 1.38; white-space: nowrap; overflow-x: clip; overflow-y: visible; text-overflow: ellipsis; }
  /* La escala no se corta por la derecha: lo que no cabe pasa a la línea siguiente. */
  .escala .linea { white-space: normal; overflow: visible; overflow-wrap: anywhere; line-height: 1.3; padding: 0.12em 0; margin: 0; }
  .juegos { display: flex; flex-direction: column; gap: 18px; }
  .juego { display: flex; flex-direction: column; gap: 4px; }
  .chars { font-size: 30px; line-height: 1.5; letter-spacing: 0.02em; word-break: break-all; }
  .bloque { display: flex; flex-direction: column; gap: 8px; }
  .glifos { display: grid; grid-template-columns: repeat(auto-fill, minmax(52px, 1fr)); gap: 4px; }
  .glifo {
    height: 56px;
    border: 0;
    border-radius: 8px;
    background: var(--chip);
    color: var(--ink);
    font-size: 26px;
    line-height: 1;
    content-visibility: auto;
    contain-intrinsic-size: 52px 56px;
    transition: background-color 0.12s, color 0.12s, transform 0.12s;
  }
  .glifo:hover { background: var(--ink); color: var(--win); transform: scale(1.06); }
  .estilo { display: flex; flex-direction: column; gap: 1px; padding: 6px 0; }
  .estilo .n { font-size: var(--fs-meta); color: var(--ink3); }
  .aviso-dup { padding: 12px 14px; border-radius: 12px; background: var(--chip); align-items: flex-start; }
  .pista-dup { font-size: var(--fs-body); color: var(--ink2); }
  .rasgos { display: flex; flex-wrap: wrap; gap: 6px; }
  .rasgo { display: flex; align-items: center; gap: 6px; height: 26px; padding: 0 10px 0 6px; border-radius: 999px; background: var(--chip); font: 600 var(--fs-small) var(--sans); }
  .rasgo code { padding: 1px 5px; border-radius: 5px; background: var(--win); color: var(--ink3); font: 500 var(--fs-meta) var(--mono); }
  .info { display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: 8px 18px; margin: 0; font-size: var(--fs-body); }
  .info dt { color: var(--ink3); }
  .info dd { margin: 0; display: flex; flex-direction: column; align-items: flex-start; gap: 4px; color: var(--ink2); overflow-wrap: anywhere; }
  .largo { white-space: pre-line; display: -webkit-box; -webkit-line-clamp: 4; -webkit-box-orient: vertical; overflow: hidden; }
  .largo.entero { display: block; }
  .enlace { padding: 0; border: 0; background: none; color: var(--ink); font: 500 var(--fs-body) var(--sans); text-decoration: underline; text-underline-offset: 2px; text-align: left; overflow-wrap: anywhere; }
  .ruta { display: flex; flex-direction: column; align-items: flex-start; gap: 2px; padding-top: 14px; border-top: 1px solid var(--line); }
  /* Cada ruta abre el Explorador con el archivo seleccionado; la carpeta aparece al pasar el ratón. */
  .archivo {
    max-width: 100%;
    display: flex;
    align-items: center;
    gap: 8px;
    margin-left: -8px;
    padding: 4px 8px;
    border: 0;
    border-radius: 6px;
    background: transparent;
    color: var(--ink2);
    font: 400 var(--fs-small) var(--mono);
    text-align: left;
    transition: background-color 0.15s, color 0.15s;
  }
  .archivo .txt { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .archivo :global(svg) { opacity: 0; transition: opacity 0.15s; }
  .archivo:hover { background: var(--hov); color: var(--ink); }
  .archivo:hover :global(svg), .archivo:focus-visible :global(svg) { opacity: 1; }
</style>
