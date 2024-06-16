<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri"
const props = defineProps<{
	path: string
	pathsToSkip: string[]
}>()

const paths = ref<string[]>([])

onMounted(async () => {
	paths.value = (await invoke("get_dir_files", {
		path: props.path,
		pathsToSkip: props.pathsToSkip,
	})) as string[]
	console.log(paths.value)
})

interface StructuredPath {
    path: string
    fileName: string
    children: StructuredPath[]
}

const structuredPaths = ref<StructuredPath[]>([])

watchEffect(() => {
    paths.value.forEach(path => {
        
    })
})
</script>

<template>
	<div>
		<!-- <div v-for="path in structuredPaths">
			<div>{{ path.fileName }}</div>
			<div>{{ path.folderPath }}</div>
		</div> -->
	</div>
</template>

<style scoped lang="scss"></style>
