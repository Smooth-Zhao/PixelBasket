<script setup lang="ts">
import {NButton, NIcon} from "naive-ui";
import {
  ZoomIn24Regular,
  ZoomOut24Regular,
  ArrowRotateCounterclockwise24Filled,
  ArrowRotateClockwise24Filled,
  PageFit20Regular
} from '@vicons/fluent'
import {onMounted, ref, watch} from "vue";
import PBFile from "../../entities/PBFile.ts";
import {convertFileSrc} from "@tauri-apps/api/tauri";
import * as PIXI from 'pixi.js';
import Tween from "@tweenjs/tween.js";

const props = defineProps<{
  file: PBFile,
  controls?: boolean
}>()

const scale = ref(1)
const angle = ref(0)
const initialScaling = ref(1)

const pixiContainerRef = ref<HTMLDivElement>()
const pixiApp = new PIXI.Application()
const imageSprite = new PIXI.Sprite();
const imageViewPadding = 2

pixiApp.stage.addChild(imageSprite)
pixiApp.stage.interactive = true
const loadImage = async () => {
  if (props.file) {
    imageSprite.texture = await PIXI.Assets.load(convertFileSrc(props.file.fullPath));

    const scaleX = (pixiApp.renderer.width - imageViewPadding) / imageSprite.texture.width
    const scaleY = (pixiApp.renderer.height - imageViewPadding) / imageSprite.texture.height

    imageSprite.scale.set(0)
    scale.value = Math.min(scaleX, scaleY)
    initialScaling.value = scale.value
    zoom()
    // 缩放图片以适应 Canvas 大小
    imageSprite.anchor.set(0.5); // 设置锚点为图片中心点
    imageSprite.position.set(pixiApp.renderer.width / 2, pixiApp.renderer.height / 2); // 设置图片位置为画布中心
  }
}

watch(() => props.file.fullPath, () => {
  loadImage()

})

onMounted(async () => {
  await pixiApp.init({
    backgroundAlpha: 0,
    resizeTo: pixiContainerRef.value,
  });

  if (pixiContainerRef.value) {
    pixiContainerRef.value?.appendChild(pixiApp.canvas)
  }
  pixiApp.renderer.addListener("resize", () => {
    imageSprite.position.set(
      pixiApp.renderer.width / 2,
      pixiApp.renderer.height / 2
    );
  })

  // 更新 TweenJS 动画
  function animate() {
    Tween.update()
    requestAnimationFrame(animate);
  }

  animate();
  loadImage()
})
const handleWheel = (e: WheelEvent) => {
  scale.value = Math.max(Math.min(50, scale.value - e.deltaY * (scale.value / 90) / 10), 0.1)
  zoom()
}
const zoom = () => {
  // 创建 TweenJS 补间动画
  new Tween.Tween(imageSprite.scale)
    .to({
      x: scale.value,
      y: scale.value
    }, 300)
    .easing(Tween.Easing.Quadratic.Out)
    .start();
}
const handleRotate = (number: number) => {
  angle.value += number
  new Tween.Tween(imageSprite)
    .to({rotation: angle.value * Math.PI / 180}, 200)
    .easing(Tween.Easing.Quadratic.Out)
    .start();
  handleZoomTo(initialScaling.value)
}

let dragging = false
let startPosition = [0, 0]
const handleMousemove = (e: MouseEvent) => {
  if (!dragging) return

  const deltaX = e.clientX - startPosition[0]
  const deltaY = e.clientY - startPosition[1]
  startPosition = [e.clientX, e.clientY]
  imageSprite.position.x += deltaX
  imageSprite.position.y += deltaY
}
const handleMousedown = (e: MouseEvent) => {
  dragging = true
  startPosition = [e.clientX, e.clientY]
}
const handleMouseup = () => {
  dragging = false
}
const handleZoom = (number: number) => {
  scale.value += number
  zoom()
}
const handleZoomTo = (number: number) => {
  scale.value = number
  zoom()
  resetPosition()
}
const resetPosition = () => {
  new Tween.Tween(imageSprite.position)
    .to({
      x: pixiApp.renderer.width / 2,
      y: pixiApp.renderer.height / 2
    }, 200)
    .easing(Tween.Easing.Quadratic.Out)
    .start();
}
</script>

<template>
  <div class="image-viewer" ref="targetRef">
    <div
      ref="pixiContainerRef"
      v-once
      class="pixi-container"
      @wheel.prevent.stop="handleWheel"
      @mousedown="handleMousedown"
      @mouseup="handleMouseup"
      @mousemove="handleMousemove"
      @mouseleave="handleMouseup"
    ></div>
    <div class="scale" @click="handleZoomTo(1)">
      {{ (scale * 100) ^ 0 }}%
    </div>
    <div class="controls" v-if="controls">
      <n-button strong secondary circle @click="handleZoom(-0.1)">
        <template #icon>
          <n-icon :size="24">
            <ZoomIn24Regular/>
          </n-icon>
        </template>
      </n-button>

      <n-button strong secondary circle @click="handleZoom(0.1)">
        <template #icon>
          <n-icon :size="24">
            <ZoomOut24Regular/>
          </n-icon>
        </template>
      </n-button>

      <n-button strong secondary :disabled="scale === 1" circle @click="handleZoomTo(1)">
        1:1
      </n-button>

      <n-button strong secondary :disabled="scale === initialScaling" circle @click="handleZoomTo(initialScaling)">
        <template #icon>
          <n-icon :size="24">
            <PageFit20Regular/>
          </n-icon>
        </template>
      </n-button>

      <n-button strong secondary circle @click="handleRotate(-90)">
        <template #icon>
          <n-icon :size="24">
            <ArrowRotateCounterclockwise24Filled/>
          </n-icon>
        </template>
      </n-button>
      <n-button strong secondary circle @click="handleRotate(90)">
        <template #icon>
          <n-icon :size="24">
            <ArrowRotateClockwise24Filled/>
          </n-icon>
        </template>
      </n-button>
    </div>
  </div>
</template>

<style scoped lang="scss">
.image-viewer {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;

  canvas, .pixi-container {
    width: 100%;
    height: 100%;
    display: block;
  }

  .controls, .scale {
    position: absolute;
    bottom: 16px;
    left: 50%;
    transform: translateX(-50%);
    background-color: rgba(0, 0, 0, .6);
    border-radius: 32px;
    padding: 8px 24px;
    box-shadow: 0 0 15px rgba(0, 0, 0, .3);
    display: flex;
    gap: 8px;

    button[disabled] {
      pointer-events: none;
    }
  }

  .scale {
    bottom: 76px;
    padding: 8px 12px;
    font-size: 12px;
  }
}
</style>
