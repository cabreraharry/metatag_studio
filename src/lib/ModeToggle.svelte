<script lang="ts">
  import { app } from './state.svelte';
</script>

<div class="segmented" role="radiogroup" aria-label="Editing mode">
  <button
    type="button"
    role="radio"
    aria-checked={app.mode === 'shared'}
    class:active={app.mode === 'shared'}
    onclick={() => (app.mode = 'shared')}
  >
    Same for every image
  </button>
  <button
    type="button"
    role="radio"
    aria-checked={app.mode === 'per-image'}
    class:active={app.mode === 'per-image'}
    onclick={() => (app.mode = 'per-image')}
  >
    Edit each separately
  </button>
</div>

<style>
  .segmented {
    display: inline-flex;
    background: #eef0f4;
    border-radius: 10px;
    padding: 3px;
    gap: 3px;
  }

  .segmented button {
    appearance: none;
    background: transparent;
    border: none;
    padding: 0.45rem 0.9rem;
    border-radius: 7px;
    font: inherit;
    font-size: 0.88rem;
    color: #54585f;
    cursor: pointer;
    transition: background 0.15s, color 0.15s, box-shadow 0.15s;
    white-space: nowrap;
  }

  .segmented button:hover {
    color: #2c2f36;
  }

  .segmented button.active {
    background: #fff;
    color: #1a3da3;
    font-weight: 600;
    box-shadow: 0 1px 3px rgba(20, 30, 70, 0.08);
  }

  .segmented button:focus-visible {
    outline: 2px solid #5b8def;
    outline-offset: 1px;
  }
</style>
