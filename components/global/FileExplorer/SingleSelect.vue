<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri"

const props = defineProps<{
    item: FileExplorerItem
}>()

const fileSize = computedAsync(async () => {
    const size = await invoke("path_size", { path: props.item.path }) as number
    
    let size_type = "Bytes"
	let full_size_formated = size
	
    if (size >= 1024) {
        size_type = "KB"
        full_size_formated = size / 1024
    }

    if (size >= 1024 * 1024) {
        size_type = "MB"
        full_size_formated = size / 1024 / 1024
    }

    if (size >= 1024 * 1024 * 1024) {
        size_type = "GB"
        full_size_formated = size / 1024 / 1024 / 1024
    }



	return `${full_size_formated.toFixed(size_type == "MB" || size_type == "Bytes" ? 0 : 2)} ${size_type}`
})

</script>

<template>
<div class="row justify-between items-center flex-1">
    <div>
        <div class="text-lg">{{ item.name }}</div>
        <div class="text-sm text-zinc-300">{{ item.path }}</div>
    </div>
    <div class="text-sm text-zinc-500 pr-5">
        {{ fileSize }}
    </div>
</div>
</template>

<style scoped lang="scss">

</style>