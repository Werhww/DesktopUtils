<script setup lang="ts">
import type { Projects, PackageJson } from "@/utils/modules/JavascriptProjectManager"
import { readPackageJson } from "@/utils/modules/JavascriptProjectManager"

const router = useRouter()
const { path } = useRoute().params

const fetchCopy = ref<Projects>()
const project = ref<Projects>()
const loading = ref(true)

const sync = computed(() => {
    return {
        state: false,
        label: "Last sync 5s ago"
    }
})

async function main() {
    loading.value = true

	const content = await readPackageJson(path as string)
    if(content === "error") {
        loading.value = false
        return
    }

    project.value = content
    fetchCopy.value = content

    loading.value = false
}

async function addProperty(property: string) {
    if(!project.value) return

    // @ts-ignore
    project.value.data[property] = "Placeholder value"
}

main()
</script>

<template>
<div class="row">
    <QBtn icon="sym_r_first_page" dense flat rounded @click="router.back()">
        <QTooltip>Back</QTooltip>
    </QBtn>
    <QBtn icon="sym_r_refresh" dense flat rounded>
        <QTooltip>Fetch json file</QTooltip>
    </QBtn>
    <QBtn icon="settings" dense flat rounded>
    </QBtn>

    <div class="flex items-end pt-3">
        <QIcon v-if="!sync.state" name="sync" size="16px" color="grey-5" class="pb-1"/>
        <div v-else class="pb-1">
            <QSpinner size="16px" color="grey-5" />
        </div>
        <div class="text-caption text-weight-medium text-grey-5">{{ sync.label }}</div>
        <QTooltip>Changes are automatically update to Json file</QTooltip>
    </div>
</div>
<div v-if="project">
    <div class="text-h4 text-capitalize">{{ project.data.name }}</div>
    <div class="text-underline text-grey-5 text-weight-medium">{{ project.path }}</div>
    
    <div class="text-overline text-grey-5 text-weight-bold">Main propertys</div>
    <QList dense separator >
        <JavascriptManagerBasicProperty icon="sym_r_description" overline="Description" property="description" v-model="project.data.description" @add-property="addProperty" />
        <JavascriptManagerBasicProperty icon="sym_r_new_releases" overline="Version" property="version" v-model="project.data.version" @add-property="addProperty" />
        <JavascriptManagerBasicProperty icon="sym_r_signature" overline="Author" property="author" v-model="project.data.author" @add-property="addProperty" />
        <JavascriptManagerBasicProperty icon="sym_r_license" overline="License" property="license" v-model="project.data.license" @add-property="addProperty" />
        <JavascriptManagerBasicProperty icon="sym_r_house" overline="Homepage" property="homepage" v-model="project.data.homepage" @add-property="addProperty" />
        <JavascriptManagerBasicProperty info="This is the primary entry point to your program" property="main" icon="sym_r_draft" overline="Main" v-model="project.data.main" @add-property="addProperty" />
    </QList>
</div>

    <!-- {{ project }} -->
</template>

<style scoped lang="scss">

</style>