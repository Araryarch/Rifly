<script setup lang="ts">
import type { Track } from '../types'
import { isHiRes } from '../types'

const props = defineProps<{
  track: Track
  compact?: boolean
}>()

const hiRes = isHiRes(props.track)
const label = props.compact
  ? (hiRes ? 'HI-RES' : props.track.format.toUpperCase())
  : (hiRes
    ? `${props.track.sample_rate / 1000}kHz/${props.track.bit_depth}bit`
    : props.track.format.toUpperCase())
</script>

<template>
  <span :class="['badge', hiRes ? 'badge-hr' : 'badge-std']">{{ label }}</span>
</template>

<style scoped>
.badge {
  display: inline-flex;
  align-items: center;
  font-family: var(--font);
  font-size: 10px;
  font-weight: 700;
  padding: 2px 6px;
  letter-spacing: 0.05em;
  white-space: nowrap;
  text-transform: uppercase;
  border: 2px solid var(--border);
  border-radius: var(--radius-base);
}

.badge-hr {
  color: var(--gold);
  border-color: var(--gold);
  background: var(--secondary-background);
}

.badge-std {
  color: color-mix(in srgb, var(--foreground) 60%, transparent);
  background: transparent;
}
</style>
