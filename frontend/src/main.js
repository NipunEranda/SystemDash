import { createApp } from 'vue';
import './assets/style.css';
import App from './App.vue';
import router from "./router";

import { library } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { faApple, faLinux, faWindows } from "@fortawesome/free-brands-svg-icons";
import GaugeChart from './components/charts/gaugeChart.vue';
import SysInfo from './components/SysInfo.vue';

library.add(faApple, faLinux, faWindows);

createApp(App)
    .use(router)
    .component("fai", FontAwesomeIcon)
    .component("gaugeChart", GaugeChart)
    .component("sysInfo", SysInfo)
    .mount('#app')
