<script setup lang="ts">
</script>

<template>
  <div class="search-view">
    <div class="sv-header">
      <h1 class="sv-title">search</h1>
    </div>
    <div class="sv-empty">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="11" cy="11" r="8"/><path d="M21 21l-4.35-4.35"/></svg>
      <p>Search your library</p>
    </div>
  </div>
</template>

<style scoped>
.search-view {
  height: 100%;
  display: flex;
  flex-direction: column;
}
.sv-header {
  padding: 24px 24px 16px;
  border-bottom: 2px solid var(--border);
}
.sv-title {
  font-size: 28px;
  font-weight: 800;
  text-transform: lowercase;
}
.sv-empty {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  color: var(--text-muted);
}
.sv-empty svg {
  width: 64px;
  height: 64px;
  opacity: 0.3;
}
.sv-empty p {
  font-size: 14px;
  font-weight: 600;
}
</style>
