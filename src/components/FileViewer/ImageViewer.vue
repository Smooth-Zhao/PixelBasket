<script setup lang="ts">
import {NButton, NIcon} from "naive-ui";
import {
  ZoomIn24Regular,
  ZoomOut24Regular,
  ArrowRotateCounterclockwise24Filled,
  ArrowRotateClockwise24Filled
} from '@vicons/fluent'
import {onMounted, ref, watch} from "vue";
import {useElementBounding} from "@vueuse/core";
import PBFile from "../../entities/PBFile.ts";
import {convertFileSrc} from "@tauri-apps/api/tauri";

const props = defineProps<{
  file: PBFile,
  controls?: boolean
}>()
const canvasRef = ref<HTMLCanvasElement>()
const targetRef = ref<HTMLDivElement>()
const angle = ref(0)
const bounding = useElementBounding(targetRef)
const scale = ref(1)
const offset = ref([0, 0])

let image: HTMLImageElement | null = null
let anchor: number[] = [0, 0]
let ctx: CanvasRenderingContext2D | null = null


const handleRotateImage = (number: number) => {
  angle.value = angle.value + number
  ctx?.rotate(number * Math.PI / 180)
  drawImage()
}
const drawImage = () => {
  if (canvasRef.value && image) {
    ctx?.clearRect(-canvasRef.value.width * scale.value, -canvasRef.value.height * scale.value, canvasRef.value.width * 2 * scale.value, canvasRef.value.height * 2 * scale.value)
    ctx?.drawImage(
      image,
      -image.naturalWidth * scale.value / 2,
      -image.naturalWidth * scale.value / 2,
      image.naturalWidth * scale.value,
      image.naturalHeight * scale.value
    )
  }
}

watch(() => props.file, async () => {
  await loadImage()
})

const loadImage = () => new Promise(resolve => {
  image = new Image()
  image.src = convertFileSrc(props.file.filePath)
  image.onload = () => {
    // 重置画布旋转
    ctx?.setTransform(1, 0, 0, 1, 0, 0)
    ctx?.translate(anchor[0], anchor[1])
    resolve(1)
  }
})
const reset = () => {
  scale.value = 1
  ctx?.setTransform(1, 0, 0, 1, 0, 0)
  ctx?.translate(anchor[0], anchor[1])
  drawImage()
}
watch(() => bounding, () => {
  if (canvasRef.value) {
    canvasRef.value.width = bounding.width.value
    canvasRef.value.height = bounding.height.value

    anchor = [canvasRef.value.width / 2, canvasRef.value.height / 2]
  }
}, {
  deep: true
})
onMounted(async () => {
  ctx = canvasRef.value ? canvasRef.value.getContext("2d") : null
  await loadImage()
  drawImage()
})
const handleWheel = (e: WheelEvent) => {
  scale.value = Math.max(Math.min(50, scale.value - e.deltaY * (scale.value / 90) / 10), 0.5)
  if (Math.abs(scale.value - 1) < 0.1) {
    scale.value = 1
  }

  drawImage()
}
let dragging = false
let startPosition = [0, 0]
const handleMousemove = (e: MouseEvent) => {
  if (!dragging) return

  const deltaX = e.clientX - startPosition[0]
  const deltaY = e.clientY - startPosition[1]

  offset.value = [offset.value[0] + deltaX, offset.value[1] + deltaY]

  startPosition = [e.clientX, e.clientY]

  ctx?.translate(deltaX, deltaY)
  drawImage()
}
const handleMousedown = (e: MouseEvent) => {
  dragging = true
  startPosition = [e.clientX, e.clientY]

}
const handleMouseup = () => {
  dragging = false
}
</script>

<template>
  <div class="image-viewer" ref="targetRef">
    <canvas
      ref="canvasRef"
      @wheel.prevent.stop="handleWheel"
      @mousedown="handleMousedown"
      @mouseup="handleMouseup"
      @mousemove="handleMousemove"
    ></canvas>
    <div class="controls" v-if="controls">
      <n-button strong secondary circle @click="reset">
        <template #icon>
          <n-icon :size="24">
            <ZoomIn24Regular/>
          </n-icon>
        </template>
      </n-button>

      <n-button strong secondary circle>
        <template #icon>
          <n-icon :size="24">
            <ZoomOut24Regular/>
          </n-icon>
        </template>
      </n-button>

      <n-button strong secondary circle @click="handleRotateImage(-90)">
        <template #icon>
          <n-icon :size="24">
            <ArrowRotateCounterclockwise24Filled/>
          </n-icon>
        </template>
      </n-button>

      <n-button strong secondary circle @click="handleRotateImage(90)">
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

  canvas {
    width: 100%;
    height: 100%;
    display: block;
  }

  .controls {
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
  }
}
</style>
