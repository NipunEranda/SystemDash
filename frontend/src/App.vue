<template>
  <div>Index</div>
</template>

<script setup>
import { onMounted, onUnmounted } from "vue";
onMounted(() => {
  setInterval(async () => {
    try {
      const response = await fetch("/api/info");
      const data = await response.text();
      console.log(JSON.parse(data));
    } catch (error) {
      console.error("Error fetching data:", error);
    }
  }, 5000); // 5000ms = 5 seconds
});
</script>
