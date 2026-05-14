<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { app, type MediaMetadata } from './state.svelte';
  import MapPicker from './MapPicker.svelte';

  let looking = $state(false);
  let lookupMessage = $state<{ kind: 'ok' | 'err'; text: string } | null>(null);

  function currentMeta(): MediaMetadata {
    if (app.mode === 'shared') return app.shared;
    if (app.selectedIndex === null) return app.shared; // fallback never used
    return app.files[app.selectedIndex].metadata;
  }

  let meta = $derived(currentMeta());

  // Clear the transient "Found: ..." confirmation when the editor switches to a
  // different file or mode. The address text itself lives on meta.address and persists
  // per-file alongside lat/lon.
  let editTarget = $derived(app.mode === 'shared' ? 'shared' : `file:${app.selectedIndex}`);
  $effect(() => {
    editTarget;
    lookupMessage = null;
  });

  let activeLabel = $derived.by(() => {
    if (app.mode === 'shared') return 'Applies to every image';
    if (app.selectedIndex !== null && app.files[app.selectedIndex])
      return `Editing: ${app.files[app.selectedIndex].name}`;
    return 'Select an image on the left';
  });

  function parseCoord(s: string): number | null {
    const v = parseFloat(s);
    return Number.isFinite(v) ? v : null;
  }

  async function lookUp() {
    lookupMessage = null;
    if (!meta.address.trim()) {
      lookupMessage = { kind: 'err', text: 'Type an address first.' };
      return;
    }
    looking = true;
    try {
      const res: { latitude: number; longitude: number; display_name: string } =
        await invoke('geocode', { address: meta.address });
      meta.latitude = res.latitude;
      meta.longitude = res.longitude;
      lookupMessage = {
        kind: 'ok',
        text: `Found: ${res.display_name}`
      };
    } catch (e) {
      lookupMessage = { kind: 'err', text: `Lookup failed: ${e}` };
    } finally {
      looking = false;
    }
  }

  let disabled = $derived(
    app.mode === 'per-image' &&
      (app.selectedIndex === null || app.files.length === 0)
  );
</script>

<div class="editor" class:disabled>
  <div class="active-row">{activeLabel}</div>

  <label class="field">
    <span class="label">Title</span>
    <input
      type="text"
      placeholder="e.g. Modern Hallandale Beach condo"
      bind:value={meta.title}
      {disabled}
    />
  </label>

  <label class="field">
    <span class="label">Description (alt text)</span>
    <textarea
      rows="3"
      placeholder="Used for accessibility and SEO. e.g. Living room with floor-to-ceiling ocean view, white sectional sofa, hardwood floors."
      bind:value={meta.alt}
      {disabled}
    ></textarea>
  </label>

  <fieldset class="gps" {disabled}>
    <legend>Location (GPS)</legend>

    <MapPicker
      latitude={meta.latitude}
      longitude={meta.longitude}
      onChange={(lat, lon) => {
        meta.latitude = lat;
        meta.longitude = lon;
      }}
    />

    <div class="latlon">
      <label class="field small">
        <span class="label">Latitude</span>
        <input
          type="number"
          step="any"
          placeholder="25.9812"
          value={meta.latitude ?? ''}
          oninput={(e) => (meta.latitude = parseCoord((e.target as HTMLInputElement).value))}
          {disabled}
        />
      </label>

      <label class="field small">
        <span class="label">Longitude</span>
        <input
          type="number"
          step="any"
          placeholder="-80.1484"
          value={meta.longitude ?? ''}
          oninput={(e) => (meta.longitude = parseCoord((e.target as HTMLInputElement).value))}
          {disabled}
        />
      </label>
    </div>

    <div class="address-row">
      <label class="field grow">
        <span class="label">Address (auto-fill from text)</span>
        <input
          type="text"
          placeholder="e.g. 100 South Pointe Dr, Miami Beach, FL"
          bind:value={meta.address}
          onkeydown={(e) => e.key === 'Enter' && lookUp()}
          {disabled}
        />
      </label>
      <button class="lookup" type="button" onclick={lookUp} disabled={disabled || looking}>
        {looking ? 'Looking…' : 'Look up'}
      </button>
    </div>

    {#if lookupMessage}
      <p class="msg" class:err={lookupMessage.kind === 'err'}>{lookupMessage.text}</p>
    {/if}
  </fieldset>
</div>

<style>
  .editor {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .editor.disabled {
    opacity: 0.55;
  }

  .active-row {
    font-size: 0.78rem;
    color: #5a5a66;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    font-weight: 600;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  .field .label {
    font-size: 0.85rem;
    font-weight: 500;
    color: #2c2f36;
  }

  .field input,
  .field textarea {
    font: inherit;
    font-size: 0.95rem;
    padding: 0.55rem 0.7rem;
    border-radius: 8px;
    border: 1px solid #d6d9e0;
    background: #fff;
    color: #1a1a1a;
    transition: border-color 0.15s, box-shadow 0.15s;
    outline: none;
  }

  .field input::placeholder,
  .field textarea::placeholder {
    color: #aab0b8;
  }

  .field input:focus,
  .field textarea:focus,
  .field input:focus-visible,
  .field textarea:focus-visible {
    border-color: #2b5fd9;
    box-shadow: 0 0 0 3px rgba(43, 95, 217, 0.18);
  }

  .field textarea {
    resize: vertical;
    min-height: 78px;
    line-height: 1.45;
  }

  .gps {
    border: 1px solid #eaecf0;
    border-radius: 12px;
    padding: 0.85rem 1rem 1rem;
    margin: 0;
    background: #fafbfd;
  }

  .gps legend {
    padding: 0 0.4rem;
    font-size: 0.85rem;
    font-weight: 600;
    color: #2c2f36;
  }

  .latlon {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.6rem;
    margin-bottom: 0.65rem;
  }

  .field.small input {
    font-variant-numeric: tabular-nums;
  }

  .address-row {
    display: flex;
    align-items: end;
    gap: 0.5rem;
  }

  .field.grow {
    flex: 1;
  }

  .lookup {
    font: inherit;
    padding: 0.55rem 0.95rem;
    border-radius: 8px;
    border: 1px solid #2b5fd9;
    background: #2b5fd9;
    color: #fff;
    cursor: pointer;
    white-space: nowrap;
    transition: background 0.15s, border-color 0.15s;
  }

  .lookup:hover:not(:disabled) {
    background: #224dbd;
    border-color: #224dbd;
  }

  .lookup:disabled {
    background: #aab8e0;
    border-color: #aab8e0;
    cursor: not-allowed;
  }

  .msg {
    margin: 0.6rem 0 0;
    font-size: 0.85rem;
    color: #2c6b3f;
  }

  .msg.err {
    color: #a13e3e;
  }
</style>
