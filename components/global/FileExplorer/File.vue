<script setup lang="ts">
const props = defineProps<{
    item: FileExplorerItem
}>()

const selectedPath = defineModel<FileExplorerItem[]>()

const emit = defineEmits<{
    select: [item: FileExplorerItem, selectRef: Ref<boolean>]
}>()

const selected = computed(() => {
    if (!selectedPath.value) return false
    return selectedPath.value.some((path) => path.path === props.item.path)
})

function selectFile() {
    emit("select", props.item, selected)
}
</script>

<template>
<div @click="selectFile()" class="row gap-3 cursor-pointer group duration-300" :class="{
    'gap-4': selected
}">
    <QSeparator vertical  />
    <div class="duration-300" :class="{
        'bg-zinc-700 p-2 rounded-md mb-1 hover:-translate-y-1 hover:drop-shadow-md': selected
    }">
        <div class="group-hover:opacity-70 duration-100 select-none">{{ item.name }}</div>
    </div>
</div>
</template>

<style scoped lang="scss">

</style>