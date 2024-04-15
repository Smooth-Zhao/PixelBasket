<script setup lang="ts">
import {ref} from "vue";
import {throttle} from "../../utils";

defineProps<{
  src: string
}>()
const videoPlayProgress = ref(0)
const videoRef = ref<HTMLVideoElement>()

const play = () => {
  if (videoRef.value) {
    videoRef.value.play();
    const updateProgress = () => {
      if (!videoRef.value?.paused && videoRef.value) {
        requestAnimationFrame(updateProgress)
        videoPlayProgress.value = (videoRef.value.currentTime / videoRef.value.duration * 100)
      }
    }
    updateProgress()
  }
}

const stop = () => {
  if (videoRef.value) {
    setTimeout(()=>{
      videoRef.value?.pause()
      videoRef.value.currentTime = 0;
      videoPlayProgress.value = 0;
    },20)
  }
}

const setVideoProgress = throttle((time:number) => {
  if (videoRef.value) {
    videoRef.value.currentTime = time;
    videoPlayProgress.value = (videoRef.value.currentTime / videoRef.value.duration * 100)
  }
}, 16)

const handleProgress = (e:MouseEvent) => {
  // 根据offsetX设置视频播放进度
  if (videoRef.value) {
    setVideoProgress(e.offsetX / e.target?.clientWidth * videoRef.value.duration)
  }
}
const pause = () => {
  if (videoRef.value) {
    videoRef.value?.pause()
  }
}
</script>

<template>
  <div class="video-viewer" @mouseenter="play" @mouseleave="stop">
    <video ref="videoRef" muted :src="src"/>
    <div class="video-progress" @mousemove.self="handleProgress"  @mouseenter="pause" @mouseleave="play">
      <span :style="{ width:videoPlayProgress + '%' }"></span>
    </div>
  </div>
</template>

<style scoped lang="scss">
.video-viewer {
  width: 100%;
  height: 100%;
  position: relative;

  &:hover .video-progress{
    height: 16px;
    border-radius: 2px;
  }

  .video-progress {
    position: absolute;
    bottom: 0;
    left: 4px;
    width: calc(100% - 8px);
    height: 0;
    border-radius: 2px;
    cursor: pointer;
    &::before{
      position: absolute;
      content: "";
      width: 100%;
      height: 4px;
      top: 6px;
      background-color: var(--color-gray-1);
    }
    span{
      position: absolute;
      display: block;
      height: 4px;
      left: 0;
      top: 6px;
      border-radius: 2px;
      background-color: var(--color-light-1);
      pointer-events: none;
    }
    //&:hover{
    //  height: 10px;
    //  span {
    //    height: 10px;
    //    border-radius: 5px;
    //  }
    //}
  }

  video {
    object-fit: contain;
    width: 100%;
    height: 100%;
    display: block;
  }
}
</style>
