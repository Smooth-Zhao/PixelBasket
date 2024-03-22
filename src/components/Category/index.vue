<script setup lang="ts">
import {appWindow} from "@tauri-apps/api/window";
import {
  ArrowRepeatAll20Filled,
  BinFull20Regular,
  BoxDismiss20Regular, Delete20Regular,
  TagDismiss20Regular, TagMultiple20Regular
} from "@vicons/fluent"
import {NIcon, TreeOption,NTree} from "naive-ui"
import {ref,h} from "vue";
import {FolderOpenOutline, FolderOutline} from "@vicons/ionicons5";
import MenuButton from "./components/HeaderButton/Menu.vue"

const categories = ref([{
  key: "",
  name: "全部",
  count: 2553,
  displayCount:true,
  icon: () => BinFull20Regular
}, {
  key: "",
  name: "未分类",
  count: 0,
  displayCount:true,
  icon: () => BoxDismiss20Regular
}, {
  key: "",
  name: "未标签",
  count: 0,
  displayCount:true,
  icon: () => TagDismiss20Regular
}, {
  key: "",
  name: "随机模式",
  count: 0,
  displayCount:false,
  icon: () => ArrowRepeatAll20Filled
}, {
  key: "",
  name: "标签管理",
  count: 0,
  displayCount:false,
  icon: () => TagMultiple20Regular
}, {
  key: "",
  name: "回收站",
  count: 40,
  displayCount:true,
  icon: () => Delete20Regular
}])

const renderSwitcherIconWithExpaned = ({ expanded }: { expanded: boolean }) =>
  h(NIcon, null, {
    default: () => h(expanded ? FolderOpenOutline : FolderOutline)
  })
function createData (level = 4, baseKey = ''): TreeOption[] | undefined {
  if (!level) return undefined
  return new Array(6 - level).fill(undefined).map((_, index) => {
    const key = '' + baseKey + level + index
    return {
      label: createLabel(level),
      key,
      children: createData(level - 1, key)
    }
  })
}

function createLabel (level: number): string {
  if (level === 4) return '新建文件夹'
  if (level === 3) return '新建文件夹'
  if (level === 2) return '新建文件夹'
  if (level === 1) return '新建文件夹'
  return ''
}
const selectedKeys = ref<string[]>([])
const nodeProps = ({ option }: { option: TreeOption }) => {
  return {
    onClick () {
      selectedKeys.value = [option.key as string]
    }
  }
}
const handleDragWindow = (e: MouseEvent) => {
  appWindow.startDragging()
  e.stopPropagation()
  e.preventDefault()
}
</script>

<template>
  <div class="category">
    <div class="header" @mousemove="handleDragWindow">
      像素篮子
      <div class="action-group">
        <MenuButton/>
      </div>
    </div>
    <ul class="category-list">
      <li v-for="item in categories">
        <span class="category-name">
            <n-icon size="18">
                <component :is="item.icon()"></component>
            </n-icon>
            {{ item.name }}
        </span>
        <span v-if="item.displayCount" class="category-item-count">{{ item.count }}</span>
      </li>
    </ul>

    <div class="folder">
      <n-tree
        show-line
        :selected-keys="selectedKeys"
        :data="createData()"
        :node-props="nodeProps"
        :render-switcher-icon="renderSwitcherIconWithExpaned"
      />
    </div>
  </div>
</template>

<style scoped lang="scss">
.category {
  .header {
    height: 40px;
    padding: 0 16px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    .action-group{
      height: 40px;
      display: flex;
      align-items: center;
      &>.n-icon{
        cursor: pointer;
      }
    }
  }
  ::v-deep(.n-tree .n-tree-node-switcher.n-tree-node-switcher--expanded){
    transform:rotate(0deg);
  }
}
.folder{
  padding: 8px;
}
.category-list {
  list-style: none;
  padding: 8px 0;
  margin: 0;

  li {
    line-height: 24px;
    display: flex;
    justify-content: space-between;
    letter-spacing: 0.05em;
    cursor: pointer;
    padding: 0 16px;
    align-items: center;

    .category-name {
      display: inline-flex;
      align-items: center;
      gap: 4px;
    }

    .category-item-count {
      color: var(--color-gray-1);
      font-size: 12px;
    }
  }
}
</style>
