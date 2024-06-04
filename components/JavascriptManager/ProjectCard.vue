<script setup lang="ts">
import type { Projects } from "@/utils/modules/JavascriptProjectManager"
import { readPackageJson } from "@/utils/modules/JavascriptProjectManager"
import {
  Notify,
} from 'quasar'

const router = useRouter()

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

const nameEl = computed(() => {
    const projectName = project.value?.data.name || ""

    if(projectName === "") return "<-- No name found -->"

    const { html, isMatch } = highlightSearchTerm(projectName, props.search)
    searchStates.value.name = isMatch
    return html
})

const pathEl = computed(() => {
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

async function main() {
    loading.value = true

	const content = await readPackageJson(props.path)
    if(content === "error") {
        loading.value = false
        return
    }

    project.value = content
    loading.value = false
}

main()
</script>

<template>
<JavascriptManagerProjectCardSkeleton v-if="loading" />
<QCard v-show="show" flat class="cursor-pointer bg-neutral-900 hover:bg-slate-700 ease-out duration-300" @click="router.push(`/JavascriptProjectManager/${encodeURIComponent(path)}`)">
    <QCardSection>
        <div class="row items-center q-gutter-x-sm"><div class="text-h6" v-html="nameEl"></div> <QBadge v-if="!!project?.data.version" style="height: fit-content;" color="red-10">{{ project?.data.version }}</QBadge></div>
        <div class="text-caption text-grey-5 text-weight-medium" v-html="pathEl"></div>
    </QCardSection>
</QCard>
</template>