<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import { app, addFiles, clearFiles, selectFile } from './state.svelte';

  let { dragOver = false }: { dragOver?: boolean } = $props();

  const IMAGE_EXTS = ['jpg', 'jpeg', 'png', 'tif', 'tiff', 'heic', 'heif', 'webp'];

  async function pickFiles() {
    const picked = await open({
      directory: false,
      multiple: true,
      filters: [{ name: 'Images', extensions: IMAGE_EXTS }]
    });
    if (!picked) return;
    const paths = Array.isArray(picked) ? picked : [picked];
    const added: { path: string; name: string }[] = await invoke('add_paths', { paths });
    addFiles(added);
  }
</script>

<div class="filelist-pane">
  <button
    class="dropzone"
    class:active={dragOver}
    onclick={pickFiles}
    type="button"
    aria-label="Add images"
  >
    <div class="icon">📁</div>
    <div class="primary">Browse files…</div>
    <div class="secondary">or drag images / folders here</div>
  </button>

  {#if app.files.length > 0}
    <div class="counts">
      {app.files.length} {app.files.length === 1 ? 'image' : 'images'}
    </div>
    <ul class="rows">
      {#each app.files as f, i}
        <li
          class:selected={app.mode === 'per-image' && app.selectedIndex === i}
          class:dim={app.mode === 'shared'}
        >
          <button class="row-btn" type="button" onclick={() => selectFile(i)}>
            <span class="dot" aria-hidden="true"></span>
            <span class="name" title={f.path}>{f.name}</span>
          </button>
        </li>
      {/each}
    </ul>
    <button class="clear" type="button" onclick={clearFiles}>Clear all</button>
  {:else}
    <div class="empty">No images yet</div>
  {/if}
</div>

<style>
  .filelist-pane {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    height: 100%;
    min-height: 0;
  }

  .dropzone {
    width: 100%;
    border: 2px dashed #c8ccd4;
    border-radius: 12px;
    background: #fff;
    padding: 1.5rem 1rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.3rem;
    color: #4a4f5b;
    cursor: pointer;
    font: inherit;
    transition: border-color 0.15s, background 0.15s, transform 0.05s;
  }

  .dropzone:hover {
    border-color: #99a3c4;
    background: #fafbff;
  }

  .dropzone:active {
    transform: scale(0.997);
  }

  .dropzone.active {
    border-color: #2b5fd9;
    background: #eef3ff;
  }

  .dropzone .icon {
    font-size: 1.8rem;
  }

  .dropzone .primary {
    font-weight: 600;
    color: #2b5fd9;
  }

  .dropzone .secondary {
    font-size: 0.85rem;
    color: #7d8295;
  }

  .counts {
    font-size: 0.8rem;
    color: #5a5a66;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    font-weight: 600;
  }

  .rows {
    list-style: none;
    margin: 0;
    padding: 0;
    overflow-y: auto;
    flex: 1;
    min-height: 0;
    background: #fff;
    border-radius: 10px;
    border: 1px solid #eaecf0;
  }

  .rows li {
    border-bottom: 1px solid #f1f2f5;
  }

  .rows li:last-child {
    border-bottom: none;
  }

  .row-btn {
    display: flex;
    align-items: center;
    gap: 0.55rem;
    width: 100%;
    text-align: left;
    background: transparent;
    border: none;
    padding: 0.55rem 0.75rem;
    font: inherit;
    color: #2c2f36;
    cursor: pointer;
    border-radius: 0;
  }

  .row-btn:hover {
    background: #f6f7fb;
  }

  .rows li.selected .row-btn {
    background: #eaf0ff;
    color: #1a3da3;
    font-weight: 600;
  }

  .rows li.dim .row-btn {
    color: #54585f;
  }

  .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #c8ccd4;
    flex-shrink: 0;
  }

  .rows li.selected .dot {
    background: #2b5fd9;
  }

  .name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 0.92rem;
  }

  .clear {
    align-self: flex-start;
    background: transparent;
    border: none;
    color: #5b8def;
    cursor: pointer;
    padding: 0;
    font: inherit;
    text-decoration: underline;
  }

  .clear:hover {
    color: #2b5fd9;
  }

  .empty {
    color: #94999f;
    text-align: center;
    padding: 1.5rem 0;
    font-size: 0.92rem;
    border: 1px dashed #eaecf0;
    border-radius: 10px;
  }
</style>
