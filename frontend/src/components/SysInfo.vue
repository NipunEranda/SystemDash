<template>
<div class="p-4 flex flex-col items-center justify-center min-h-full">
    <h2 class="text-2xl font-bold mb-4 text-center">System Info</h2>
    <div class="grid grid-cols-2 gap-y-3 gap-x-4">
        <div class="font-semibold text-right">Host Name:</div>
        <div class="truncate">{{ props.info.primary.host_name }}</div>
        <div class="font-semibold text-right">System Name:</div>
        <div>{{ props.info.primary.system_name }}</div>
        <div class="font-semibold text-right">OS Version:</div>
        <div>{{ props.info.primary.os_version }}</div>
        <div class="font-semibold text-right">Kernel Version:</div>
        <div>{{ props.info.primary.kernel_version }}</div>
        <div class="font-semibold text-right">CPU Count:</div>
        <div>{{ props.info.primary.cpu_count }}</div>
        <div class="font-semibold text-right">Process Count:</div>
        <div>{{ props.info.primary.process_count }}</div>
    </div>
</div>
</template>

<script setup>
import { defineProps } from "vue";
const props = defineProps({
  info: {
    type: Object
  }
})
</script>
