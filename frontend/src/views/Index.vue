<template>
  <div class="grid">
    <div class="flex w-screen">
      <div class="w-full">
        <CPU-GRAPH v-if="sys_info.primary" :cpus="sys_info.cpus ? sys_info.cpus : []" />
      </div>
      <div class="w-full">
        <MEMORY-GRAPH />
      </div>
      <div class="w-full">
        <DISK-GRAPH />
      </div>
    </div>
  </div>
</template>

<script setup>
import { onMounted, onUnmounted, ref } from "vue";

let sys_info = ref({});

onMounted(() => {
  const socket = new WebSocket("ws://localhost:8000/ws");

  socket.onopen = () => {
    console.log("WebSocket connection established");
  };

  socket.onmessage = (event) => {
    try {
      sys_info.value = JSON.parse(event.data);
      // console.log(sys_info.value);
    } catch (e) {
      sys_info.value = {};
    }
  };

  socket.onclose = () => {
    console.log("WebSocket connection closed");
  };

  socket.onerror = (error) => {
    console.error("WebSocket error:", error);
  };
});
</script>
