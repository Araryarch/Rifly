<script setup lang="ts">
defineProps<{
  playing?: boolean
  size?: 'sm' | 'md'
}>()
</script>

<template>
  <div :class="['eq-bars', size === 'sm' ? 'eq-sm' : 'eq-md']">
    <span :class="['eq-bar eq-bar-1', { paused: !playing }]" />
    <span :class="['eq-bar eq-bar-2', { paused: !playing }]" />
    <span :class="['eq-bar eq-bar-3', { paused: !playing }]" />
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
  width: 4px;
  background: var(--main);
  will-change: height;
  border-top-left-radius: 0;
  border-top-right-radius: 0;
}

.eq-bar-1 { animation: eqBar 0.8s ease-in-out infinite; }
.eq-bar-2 { animation: eqBar2 0.7s ease-in-out infinite 0.1s; }
.eq-bar-3 { animation: eqBar3 0.9s ease-in-out infinite 0.2s; }

.eq-bar.paused {
  animation-play-state: paused;
  height: 4px !important;
}
</style>
