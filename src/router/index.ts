import {createRouter, createWebHistory} from "vue-router";

const router = createRouter({
  history:createWebHistory(),
  routes:[{
    name:"main",
    path:"/",
    component:()=>import("../windows/Main.vue")
  },{
    name:"file",
    path:"/file/:src",
    component:()=>import("../windows/File.vue"),
    props:(route)=>({src:route.params.src})
  }]
})
export default router
