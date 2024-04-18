import {createRouter, createWebHistory} from "vue-router";

const router = createRouter({
  history:createWebHistory(),
  routes:[{
    name:"main",
    path:"/",
    component:()=>import("../windows/Main.vue")
  },{
    name:"file",
    path:"/file/:id",
    component:()=>import("../windows/File.vue"),
    props:(route)=>({id:route.params.id})
  }]
})
export default router
