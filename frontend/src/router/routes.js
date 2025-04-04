import Index from "../views/Index.vue";
import DefaultLayout from "../layouts/DefaultLayout.vue";
import ErrorLayout from "../layouts/ErrorLayout.vue";

const routes = [
    {
        path: '/:catchAll(.*)',
        component: ErrorLayout
    },
    {
        path: '/',
        component: DefaultLayout,
        children: [
            {
                path: "/",
                name: "index",
                component: Index,
            }
        ]
    },
];

export default routes;