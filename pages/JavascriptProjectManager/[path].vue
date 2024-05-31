<script setup lang="ts">
import type { Projects } from "@/utils/modules/JavascriptProjectManager"
import { readPackageJson } from "@/utils/modules/JavascriptProjectManager"

const router = useRouter()
const { path } = useRoute().params

const project = ref<Projects>()
const loading = ref(true)

async function main() {
    loading.value = true

	const content = await readPackageJson(path as string)
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
<div class="text-h1" @click="router.back()">back</div>
{{ project }}
</template>

<style scoped lang="scss">

</style>