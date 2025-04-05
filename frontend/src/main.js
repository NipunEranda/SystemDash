import { createApp } from 'vue';
import './assets/style.css';
import App from './App.vue';
import router from "./router";

import { Chart } from 'chart.js';
import annotationPlugin from 'chartjs-plugin-annotation';

import { library } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import CpuGraph from './components/CpuGraph.vue';
import MemoryGraph from './components/MemoryGraph.vue';
import DiskGraph from './components/DiskGraph.vue';

Chart.register(annotationPlugin);

createApp(App)
.use(router)
.component("fai", FontAwesomeIcon)
.component("CPU-GRAPH", CpuGraph)
.component("MEMORY-GRAPH", MemoryGraph)
.component("DISK-GRAPH", DiskGraph)
.component(Chart, Chart)
.mount('#app')
