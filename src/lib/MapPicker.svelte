<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import L from 'leaflet';
  import 'leaflet/dist/leaflet.css';
  import iconUrl from 'leaflet/dist/images/marker-icon.png';
  import iconRetinaUrl from 'leaflet/dist/images/marker-icon-2x.png';
  import shadowUrl from 'leaflet/dist/images/marker-shadow.png';

  // Workaround for the well-known Leaflet+bundler default-icon path issue: clear the
  // prototype _getIconUrl override so leaflet uses the explicit option URLs (which Vite
  // resolves to absolute paths) instead of prepending its auto-detected imagePath.
  delete (L.Icon.Default.prototype as unknown as { _getIconUrl?: unknown })._getIconUrl;
  L.Icon.Default.mergeOptions({ iconUrl, iconRetinaUrl, shadowUrl });

  type Props = {
    latitude: number | null;
    longitude: number | null;
    onChange: (lat: number, lon: number) => void;
  };

  let { latitude, longitude, onChange }: Props = $props();

  let mapEl: HTMLDivElement;
  let map: L.Map | null = null;
  let marker: L.Marker | null = null;

  const DEFAULT_CENTER: L.LatLngTuple = [26.0, -80.2]; // South Florida
  const DEFAULT_ZOOM = 9;

  function hasCoords(): boolean {
    return (
      typeof latitude === 'number' &&
      Number.isFinite(latitude) &&
      typeof longitude === 'number' &&
      Number.isFinite(longitude)
    );
  }

  function ensureMarker(lat: number, lon: number) {
    if (!map) return;
    if (!marker) {
      marker = L.marker([lat, lon], { draggable: true }).addTo(map);
      marker.on('dragend', () => {
        if (!marker) return;
        const ll = marker.getLatLng();
        onChange(ll.lat, ll.lng);
      });
    } else {
      marker.setLatLng([lat, lon]);
    }
  }

  function removeMarker() {
    if (marker && map) {
      map.removeLayer(marker);
      marker = null;
    }
  }

  onMount(() => {
    map = L.map(mapEl, {
      center: hasCoords() ? [latitude!, longitude!] : DEFAULT_CENTER,
      zoom: hasCoords() ? 14 : DEFAULT_ZOOM,
      zoomControl: true,
      attributionControl: true,
      worldCopyJump: true
    });

    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      maxZoom: 19,
      attribution: '© OpenStreetMap contributors'
    }).addTo(map);

    map.on('click', (e: L.LeafletMouseEvent) => {
      onChange(e.latlng.lat, e.latlng.lng);
    });

    if (hasCoords()) {
      ensureMarker(latitude!, longitude!);
    }
  });

  $effect(() => {
    if (!map) return;
    if (hasCoords()) {
      ensureMarker(latitude!, longitude!);
      map.setView([latitude!, longitude!], Math.max(map.getZoom(), 13));
    } else {
      removeMarker();
    }
  });

  onDestroy(() => {
    if (map) {
      map.remove();
      map = null;
      marker = null;
    }
  });
</script>

<div class="map-wrap">
  <div bind:this={mapEl} class="map"></div>
  <p class="hint">Click anywhere on the map to drop a pin, or drag the pin to fine-tune.</p>
</div>

<style>
  .map-wrap {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    margin-bottom: 0.75rem;
  }

  .map {
    height: 220px;
    border-radius: 8px;
    border: 1px solid #d6d9e0;
    overflow: hidden;
    background: #e7e9ee;
  }

  .hint {
    margin: 0;
    font-size: 0.8rem;
    color: #6c707a;
  }

  /* Leaflet adds its own controls; tone down the default attribution font to fit */
  :global(.leaflet-container .leaflet-control-attribution) {
    font-size: 10px;
    background: rgba(255, 255, 255, 0.85);
  }
</style>
