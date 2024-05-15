<script setup lang="ts">

</script>

<template>
  <div class="category">
    <div class="header">像素篮子</div>
    <ul class="category-list">
      <li>
        <span>全部</span>
        <span class="category-item-count">2553</span>
      </li>
      <li>未分类</li>
      <li>未标签</li>
      <li>随机模式</li>
      <li>标签管理</li>
      <li>回收站</li>
    </ul>
  </div>
</template>

<style scoped lang="scss">
.category{
  .header{
    height: 40px;
    padding: 0 16px;
    display: flex;
    align-items: center;
  }
}
.category-list{
  list-style: none;
  padding: 8px 0;
  margin: 0;
  li{
    line-height: 24px;
    display: flex;
    justify-content: space-between;
    letter-spacing: 0.05em;
    cursor: pointer;
    padding: 0 16px;
    .category-item-count{
      color: var(--color-gray-1);
      font-size: 12px;
    }
  }
}
</style>