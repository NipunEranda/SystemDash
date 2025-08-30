<template>
  <div
    class="flex items-center justify-center relative w-full h-full max-w-4xl mx-auto"
  >
    <svg
      width="100%"
      height="100%"
      viewBox="0 0 300 300"
      xmlns="http://www.w3.org/2000/svg"
      class="w-full h-full"
    >
      <!-- Gradient -->
      <defs>
        <linearGradient
          id="g"
          x1="270"
          y1="150"
          x2="30"
          y2="150"
          gradientUnits="userSpaceOnUse"
        >
          <stop offset="0%" stop-color="#2ecc71" />
          <stop offset="50%" stop-color="#f1c40f" />
          <stop offset="100%" stop-color="#fb2f03" />
        </linearGradient>
      </defs>

      <!-- Placeholder half-circle (background) -->
      <!-- #2b2f36 -->
      <circle
        cx="150"
        cy="150"
        r="120"
        fill="none"
        :stroke="props.color"
        stroke-width="30"
        stroke-dasharray="754 754"
        stroke-dashoffset="300"
        stroke-linecap="butt"
        transform="rotate(161.5 150 150)"
      />

      <!-- Colored half-circle (foreground, fills the whole half) -->
      <circle
        cx="150"
        cy="150"
        r="120"
        fill="none"
        stroke="url(#g)"
        stroke-width="30"
        stroke-linecap="butt"
        :stroke-dasharray="754"
        :stroke-dashoffset="300 + dashoffset"
        transform="rotate(162 150 150)"
        class="gauge-foreground"
      />

      <!-- Center label -->
      <text
        x="150"
        y="130"
        font-family="Arial, sans-serif"
        text-anchor="middle"
        :fill="labelColor"
        font-weight="bold"
        :font-size="`1.5rem`"
        v-if="label"
      >
        {{ label }}
      </text>

      <!-- Center label -->
      <text
        x="150"
        :y="value ? '180' : '195'"
        font-family="Arial, sans-serif"
        text-anchor="middle"
        :fill="percentColor"
        font-weight="bold"
        :font-size="value ? valueSize : `4rem`"
      >
        {{ value ? value : percentage }}
      </text>
    </svg>
  </div>
</template>

<script setup>
import { computed, defineProps } from "vue";

const props = defineProps({
  percentage: {
    type: String,
    default: 100,
  },
  color: {
    type: String,
    default: "#2b2f36",
  },
  label: {
    type: String,
    required: false,
  },
  labelColor: {
    type: String,
    default: "#909193",
    required: false,
  },
  value: {
    type: String,
  },
  valueSize: {
    type: String,
    default: "3rem",
  },
});

const FULL_CIRCUMFERENCE = 2 * Math.PI * 144.6; // ≈ 754
const HALF_CIRCUMFERENCE = FULL_CIRCUMFERENCE / 2; // ≈ 377
const dasharray = HALF_CIRCUMFERENCE;
const dashoffset = computed(() => {
  // 0% fill = full offset (empty), 100% fill = 0 offset (full)
  return dasharray * (1 - parseInt(props.percentage) / 100);
});

const percentColor = computed(() => {
  if (parseInt(props.percentage) <= 50) return "#2ecc71"; // green
  if (parseInt(props.percentage) <= 80) return "#f1c40f"; // yellow
  return "#fb2f03"; // red
});
</script>

<style scoped>
/* All styles handled by Tailwind and SVG */
.gauge-foreground {
  transition: stroke-dashoffset 0.7s cubic-bezier(0.4, 0, 0.2, 1);
}
</style>
