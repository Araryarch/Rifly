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
  <span :class="['badge', hiRes ? 'badge--hires' : 'badge--std']">{{ label }}</span>
</template>

<style scoped>
.badge {
  display: inline-flex;
  align-items: center;
  font-family: var(--font);
  font-size: 9px;
  font-weight: 700;
  padding: 1px 5px;
  letter-spacing: 0.05em;
  white-space: nowrap;
  text-transform: uppercase;
}

.badge--hires {
  color: var(--color-gold);
  border: 1.5px solid var(--color-gold);
}

.badge--std {
  color: var(--color-text-tertiary);
  border: 1px solid var(--color-border);
}
</style>
