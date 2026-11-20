<script>
  // Estado vacío: un icono fino y gris, el título, una frase y, si hace falta, un botón.
  import { SearchX } from "@lucide/svelte";

  let { icono = SearchX, titulo, texto, boton = "", onboton = null, girando = false } = $props();
  const Icono = $derived(icono);
</script>

<div class="vacio">
  <span class="icono" class:girando><Icono size={36} strokeWidth={1.5} /></span>
  <div class="txt">
    <span class="titulo">{titulo}</span>
    {#if texto}<span class="texto">{texto}</span>{/if}
  </div>
  {#if boton}<button type="button" onclick={onboton}>{boton}</button>{/if}
</div>

<style>
  .vacio { flex: 1; min-height: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 16px; text-align: center; padding-bottom: 60px; animation: tf-up 0.3s ease-out; }
  .icono { color: var(--ink3); display: flex; }
  .girando { animation: girar 1.1s linear infinite; }
  @keyframes girar { to { rotate: 360deg; } }
  @media (prefers-reduced-motion: reduce) { .girando { animation: none; } }
  .txt { display: flex; flex-direction: column; gap: 6px; max-width: 360px; }
  .titulo { font-size: var(--fs-title); font-weight: 700; letter-spacing: -0.01em; }
  .texto { font-size: var(--fs-body); line-height: 1.5; color: var(--ink2); text-wrap: pretty; }
  button { height: 30px; padding: 0 14px; border-radius: 999px; border: 0; background: var(--ink); color: var(--win); font: 600 var(--fs-small) var(--sans); }
</style>
