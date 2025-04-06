import { createApp } from 'vue';
import './assets/style.css';
import App from './App.vue';
import router from "./router";

import { Chart } from 'chart.js';
import annotationPlugin from 'chartjs-plugin-annotation';

import { library } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { faApple, faLinux, faWindows } from "@fortawesome/free-brands-svg-icons";
import { faHome } from "@fortawesome/free-solid-svg-icons";

import CpuGraph from './components/CpuGraph.vue';
import System from './components/System.vue';
import DiskGraph from './components/DiskGraph.vue';

Chart.register(annotationPlugin);

library.add(faApple, faLinux, faWindows);

createApp(App)
    .use(router)
    .component("fai", FontAwesomeIcon)
    .component("CPU-GRAPH", CpuGraph)
    .component("System", System)
    .component("DISK-GRAPH", DiskGraph)
    .component(Chart, Chart)
    .mount('#app')
