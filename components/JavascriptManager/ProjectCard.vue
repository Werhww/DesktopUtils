<script setup lang="ts">
import type { Projects } from "@/types/JavascriptProjectManager"
import { invoke } from "@tauri-apps/api/tauri"
import {
  Notify,
} from 'quasar'

const props = defineProps<{
    path: string
    search: string
}>()

const searchStates = ref({
    name: false,
    path: false,
})

const project = ref<Projects>()
const loading = ref(true)

const name = computed(() => {
    const projectName = project.value?.data.name || ""

    const { html, isMatch } = highlightSearchTerm(projectName, props.search)
    searchStates.value.name = isMatch
    return html
})

const path = computed(() => {
    const path = props.path

    const { html, isMatch } = highlightSearchTerm(path, props.search)
    searchStates.value.path = isMatch
    return html
})

const show = computed(() => {
    if(loading.value) return false

    if(project.value === undefined) return false
    if(props.search === "") return true

    return searchStates.value.name || searchStates.value.path
})

async function readAndTransformPackageJsonFile() {
    loading.value = true

	const fileContent = await invoke("read_file", { filePath: props.path }) as string

    if(fileContent === "") {
        Notify.create({
			message: "Error reading file, file may be empty or not found.",
			color: "negative",
			position: "top",
			icon: "error"
		})
        
        return 
    }

	const packageJson = JSON.parse(fileContent)

	project.value = {
        path: props.path,
        data: packageJson,
    }

    loading.value = false
}


readAndTransformPackageJsonFile()
</script>

<template>
<JavascriptManagerProjectCardSkeleton v-if="loading" />
<QCard v-show="show" flat class="cursor-pointer bg-neutral-900 hover:bg-slate-700 ease-out duration-300">
    <QCardSection>
        <div class="row items-center q-gutter-x-sm"><div class="text-h6" v-html="name"></div> <QBadge v-if="!!project?.data.version" style="height: fit-content;" color="red-10">{{ project?.data.version }}</QBadge></div>
        <div class="text-caption text-grey-5 text-weight-medium" v-html="path"></div>
    </QCardSection>
</QCard>
</template>