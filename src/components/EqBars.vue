<script setup lang="ts">
defineProps<{
  playing?: boolean
  color?: string
  size?: 'sm' | 'md'
}>()
</script>

<template>
  <div :class="['eq-bars', size === 'sm' ? 'eq-sm' : 'eq-md']">
    <span :class="['eq-bar eq-bar-1', { paused: !playing }]" :style="color ? { background: color } : {}" />
    <span :class="['eq-bar eq-bar-2', { paused: !playing }]" :style="color ? { background: color } : {}" />
    <span :class="['eq-bar eq-bar-3', { paused: !playing }]" :style="color ? { background: color } : {}" />
  </div>
</template>

<style scoped>
.eq-bars {
  display: flex;
  align-items: flex-end;
  gap: 2px;
}

.eq-sm {
  height: 14px;
}

.eq-md {
  height: 18px;
}

.eq-bar {
  width: 3px;
  background: var(--color-accent);
  will-change: height;
}

.eq-bar-1 { animation: eqBar 0.8s ease-in-out infinite; }
.eq-bar-2 { animation: eqBar2 0.7s ease-in-out infinite 0.1s; }
.eq-bar-3 { animation: eqBar3 0.9s ease-in-out infinite 0.2s; }

.eq-bar.paused {
  animation-play-state: paused;
  height: 3px !important;
}
</style>
