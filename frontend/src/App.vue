<template>
  <div>
    <div
      class="fixed flex inset-0 items-center justify-center bg-white dark:bg-dark-theme-primary h-screen w-screen z-50 transition-opacity duration-500"
      :class="{ 'opacity-100': !sys_info, 'opacity-0': sys_info }"
      style="pointer-events: all"
    >
      <img
        src="/app-dark.png"
        alt="Loading..."
        width="400"
        class="blink"
      />
    </div>
    <RouterView />
  </div>
</template>

<script setup>
import { onMounted, ref, computed, provide, onUnmounted } from "vue";

const sys_info = ref(null);
provide("sys_info", sys_info);

let socket = null;
let reconnectTimeout = null;
let connectionCheckInterval = null;

function connectSocket() {
  if (
    socket &&
    (socket.readyState === WebSocket.OPEN ||
      socket.readyState === WebSocket.CONNECTING)
  ) {
    return;
  }
  socket = new WebSocket("ws://localhost:8000/ws");

  socket.onopen = () => {
    if (reconnectTimeout) {
      clearTimeout(reconnectTimeout);
      reconnectTimeout = null;
    }
  };

  socket.onmessage = (event) => {
    try {
      sys_info.value = JSON.parse(event.data);
      console.log(sys_info.value);
    } catch (e) {
      sys_info.value = {};
    }
  };

  socket.onclose = () => {
    // Connection closed, no log
  };

  socket.onerror = () => {
    if (socket && socket.readyState !== WebSocket.CLOSED) {
      socket.close();
    }
  };
}

onMounted(() => {
  connectSocket();
  connectionCheckInterval = setInterval(() => {
    if (
      !socket ||
      socket.readyState === WebSocket.CLOSED ||
      socket.readyState === WebSocket.CLOSING
    ) {
      connectSocket();
    }
  }, 2000);
});

onUnmounted(() => {
  if (connectionCheckInterval) {
    clearInterval(connectionCheckInterval);
    connectionCheckInterval = null;
  }
  if (reconnectTimeout) {
    clearTimeout(reconnectTimeout);
    reconnectTimeout = null;
  }
  if (socket) {
    socket.close();
    socket = null;
  }
});
</script>

<style scoped>
@keyframes blink {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.3;
  }
}
.blink {
  animation: blink 1s infinite;
}
</style>
