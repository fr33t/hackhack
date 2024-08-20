import { createRouter, createWebHashHistory } from "vue-router";
const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      name: "Index",
      component: () => import("../views/index.vue"),
    },
    {
      path: "/panel",
      name: "Panel",
      meta: { requiresAuth: true },
      component: () => import("../views/panel.vue"),
    },
    { path: "/:pathMatch(.*)*", redirect: "/" },
  ],
});

router.beforeEach((to, _, next) => {
  const token = localStorage.getItem("token");
  if (to.meta.requiresAuth) {
    if (token) {
      next(); // 允许访问
    } else {
      next("/"); // 重定向到登录页面
    }
  } else {
    next();
  }
});
export default router;
