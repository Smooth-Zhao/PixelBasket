import {createRouter, createWebHistory} from "vue-router";

const router = createRouter({
  history:createWebHistory(),
  routes:[{
    name:"main",
    path:"/",
    component:()=>import("../windows/Main.vue")
  },{
    name:"file",
    path:"/file/:path",
    component:()=>import("../windows/File.vue"),
    props:(route)=>({path:route.params.path})
  }]
})
export default router
