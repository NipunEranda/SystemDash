<template>
  <div class="cpu-gauge-container w-full px-16 pt-5">
    <canvas ref="cpuGaugeCanvas" class="cpu-gauge-canvas w-full"></canvas>
    <canvas ref="cpuUsageBarCanvas"></canvas>
  </div>
</template>

<script setup>
import { ref, watch, onMounted } from "vue";
import {
  Chart,
  DoughnutController,
  ArcElement,
  Tooltip,
  Legend,
  BarController,
  BarElement,
  CategoryScale,
  LinearScale,
  Title,
} from "chart.js";
import annotationPlugin from "chartjs-plugin-annotation";
import { round } from "../utils";

Chart.register(
  DoughnutController,
  ArcElement,
  Tooltip,
  BarController,
  BarElement,
  CategoryScale,
  LinearScale,
  Title,
  Tooltip,
  Legend
);

const props = defineProps({
  cpuUsage: {
    type: Number,
    required: true,
    validator: (value) => value >= 0 && value <= 100,
  },
  cpus: {
    type: Array,
    required: true
  }
});

const cpuGaugeCanvas = ref(null);
const cpuUsageBarCanvas = ref(null)
let cpuChart = null,
cpuBarChart = null;

onMounted(() => {
  const canvas = cpuGaugeCanvas.value;
  const ctx = canvas.getContext("2d");

  // Ensure the canvas has proper dimensions
  const devicePixelRatio = window.devicePixelRatio || 1;
  const width = canvas.offsetWidth;
  const height = canvas.offsetHeight;

  // Set canvas resolution for high DPI
  canvas.width = width * devicePixelRatio;
  canvas.height = height * devicePixelRatio;
  ctx.scale(devicePixelRatio, devicePixelRatio);

  cpuChart = new Chart(ctx, {
    type: "doughnut",
    data: {
      labels: ["Used", "Available"],
      datasets: [
        {
          data: [props.cpuUsage, 100 - props.cpuUsage],
          backgroundColor: ["#00bba7", "#18181b"],
          borderWidth: 3,
          borderColor: "#27272a",
        },
      ],
    },
    options: {
      responsive: false, // Disable automatic resizing to avoid conflicts
      maintainAspectRatio: false, // Allow custom scaling
      rotation: -90, // Start angle for half-doughnut
      circumference: 180, // Half-doughnut
      plugins: {
        legend: {
          display: false,
        },
        tooltip: {
          enabled: true,
        },
        customText: {
          fontSize: 70,
          fontColor: "#00bba7",
        },
      },
      cutout: "80%", // Creates a gauge-like appearance
    },
    plugins: [
      {
        id: "customText",
        beforeDraw(chart) {
          const { width } = chart;
          const { height } = chart;
          const ctx = chart.ctx;
          const fontSize = chart.options.plugins.customText.fontSize || 18;
          const fontColor =
            chart.options.plugins.customText.fontColor || "#000";
          const text = `${round(props.cpuUsage)}%`;
          ctx.save();
          ctx.font = `${fontSize}px sans-serif`;
          ctx.fillStyle = fontColor;
          ctx.textAlign = "center";
          ctx.textBaseline = "middle";
          ctx.fillText(text, width / 2, height / 1.4); // Adjust position as needed
          ctx.restore();
        },
      },
    ],
  });

  cpuBarChart = new Chart(cpuUsageBarCanvas.value, {
    type: 'bar',
    data: {
      labels: props.cpus.map(cpu => cpu = `CPU${cpu.index}`),
      datasets: [{
        label: 'CPU Usage',
        data: props.cpus.map(cpu => cpu = cpu.usage),
        backgroundColor: 'rgba(75, 192, 192, 0.5)',
        borderColor: 'rgba(75, 192, 192, 1)',
        borderWidth: 1
      }]
    },
    options: {
      indexAxis: 'y',
      responsive: true,
      scales: {
        x: {
          beginAtZero: true,
          max: 100, // Set maximum value to 100
        },
      },
    },
  });
});

watch(
  () => props.cpuUsage,
  (newUsage) => {
    if (cpuChart) {
      cpuChart.data.datasets[0].data = [newUsage, 100 - newUsage];
      cpuChart.update();
    }
  }
);

watch(
  () => props.cpus,
  (newUsage) => {
    cpuBarChart.data.datasets[0].data = newUsage.map(cpu => cpu = cpu.usage);
    console.log(cpuBarChart.data.datasets[0].data);
    cpuBarChart.update();
  }
);
</script>
