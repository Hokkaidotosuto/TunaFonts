<script>
  // Menú del clic derecho en una fuente (o en la selección, si la fuente está en ella).
  // Mismo aspecto que los menús de la barra de selección.
  import { ChevronRight } from "@lucide/svelte";
  import { t } from "../lib/i18n.svelte.js";

  let { x, y, titulo, opciones, oncerrar } = $props();

  let caja = $state();
  let abierto = $state(null);
  let seguro = $state(null);
  // Que no se salga de la ventana: si no cabe, se abre hacia la izquierda o hacia arriba.
  let pos = $state({ left: x, top: y });
  $effect(() => {
    if (!caja) return;
    const r = caja.getBoundingClientRect();
    pos = { left: Math.max(8, Math.min(x, innerWidth - r.width - 8)), top: Math.max(8, Math.min(y, innerHeight - r.height - 8)) };
  });

  function elegir(o) {
    if (o.sub) return (abierto = abierto === o.texto ? null : o.texto);
    if (o.confirmar && seguro !== o.texto) return (seguro = o.texto);
    oncerrar();
    o.accion();
  }
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && oncerrar()} onblur={oncerrar} />
<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="fondo" onclick={oncerrar} oncontextmenu={(e) => (e.preventDefault(), oncerrar())}></div>
<div class="menu" bind:this={caja} style:left="{pos.left}px" style:top="{pos.top}px" role="menu">
  {#if titulo}<span class="tit">{titulo}</span>{/if}
  {#each opciones as o (o.texto)}
    {#if o.separador}<span class="sep"></span>{/if}
    <button type="button" class="op" class:peligro={o.peligro} class:seguro={seguro === o.texto} role="menuitem" onclick={() => elegir(o)}>
      {#if o.icono}<o.icono size={14} strokeWidth={1.9} />{/if}
      <span class="txt">{seguro === o.texto ? t("desinstalar.seguro") : o.texto}</span>
      {#if o.sub}<ChevronRight size={13} strokeWidth={2} class="flecha" />{/if}
    </button>
    {#if o.sub && abierto === o.texto}
      <span class="sub">
        {#each o.sub as s (s.texto)}
          <button type="button" class="op" role="menuitem" onclick={() => (oncerrar(), s.accion())}>{s.texto}</button>
        {/each}
      </span>
    {/if}
  {/each}
</div>

<style>
  .fondo { position: fixed; inset: 0; z-index: 30; }
  .menu {
    position: fixed;
    z-index: 31;
    min-width: 210px;
    max-width: 280px;
    display: flex;
    flex-direction: column;
    padding: 5px;
    border-radius: 12px;
    background: var(--win);
    color: var(--ink);
    box-shadow: 0 12px 30px rgba(0, 0, 0, 0.18), 0 0 0 1px var(--line2);
    animation: tf-up 0.12s ease-out;
  }
  .tit { padding: 6px 10px 4px; font: 600 var(--fs-meta) var(--sans); letter-spacing: 0.08em; text-transform: uppercase; color: var(--ink3); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .sep { height: 1px; margin: 4px 6px; background: var(--line2); }
  .op { height: 30px; padding: 0 10px; border: 0; border-radius: 7px; background: transparent; color: var(--ink); font: 500 var(--fs-small) var(--sans); display: flex; align-items: center; gap: 9px; text-align: left; transition: background-color 0.12s; }
  .op:hover { background: var(--hov); }
  .op .txt { flex: 1; min-width: 0; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .op.peligro { color: var(--red); }
  .op.seguro, .op.seguro:hover { background: var(--red); color: #fff; }
  .sub { display: flex; flex-direction: column; margin: 0 0 4px 22px; padding-left: 6px; border-left: 1px solid var(--line2); }
  .sub .op { height: 28px; }
</style>
