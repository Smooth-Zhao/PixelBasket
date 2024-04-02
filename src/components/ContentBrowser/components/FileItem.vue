<script setup lang="ts">

defineProps<{
  src:string
}>()

</script>

<template>
  <div class="file-item">
    <div class="cover">
      <img :src="src" alt="">
    </div>
    <div class="info">
      <span class="file-name">JSON.JPG</span>
      <span class="resolution">4096 * 1080</span>
    </div>
  </div>
</template>

<style scoped lang="scss">
.file-item {
  aspect-ratio: 1/1;

  .cover {
    display: block;
    width: 100%;
    aspect-ratio: 1/1;

    img {
      width: 100%;
      height: 100%;
      object-fit: contain;
    }
  }

  .info {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-top: 16px;

    span {
      width: auto;
    }

    .file-name {
      color: var(--color-light-1);
      padding: 0 4px;
    }

    .resolution {
      font-size: 12px;
    }
  }

  &.selected {
    .cover {
      position: relative;
    }

    .cover::after {
      content: "";
      position: absolute;
      width: calc(100% - 4px);
      height: calc(100% - 4px);
      left: 0;
      top: 0;
      border: solid 2px #12b444;
      border-radius: 8px;
      pointer-events: none;
    }

    .file-name {
      background-color: #12b444;
    }
  }
}
</style>
