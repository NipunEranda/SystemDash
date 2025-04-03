<template>
  <div>{{ sys_info }}</div>
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
