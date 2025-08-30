<template>
  <div class="grid" v-if="averageCpuUsage">
      <div style="display: flex; gap: 24px; width: 100%;">
        <gaugeChart :percentage="`${averageCpuUsage}%`" label="CPU" style="flex: 1;" />
        <gaugeChart :percentage="`${averageMemoryUsage}%`" label="Memory" :value="memoryText" style="flex: 1;" valueSize="1.4rem" />
      </div>
  </div>
</template>

<script setup>
import { inject, computed } from "vue";
const sys_info = inject('sys_info');

const averageCpuUsage = computed(() => {
  if (sys_info.value && sys_info.value.cpus) {
    return Math.round(sys_info.value.cpus.reduce((acc, cpu) => acc + cpu.usage, 0) / sys_info.value.cpus.length);
  }
  return 0;
});

const averageMemoryUsage = computed(() => {
  if (sys_info.value && sys_info.value.memory) {
    return Math.round(sys_info.value.memory.used_memory / sys_info.value.memory.total_memory * 100);
  }
  return 0;
});

const memoryText = computed(() => {
  if (sys_info.value && sys_info.value.memory) {
    return `${(sys_info.value.memory.used_memory / (1024 ** 3)).toFixed(2)} / ${(sys_info.value.memory.total_memory / (1024 ** 3)).toFixed(2)} GB`;
  }
  return '0';
});
</script>
