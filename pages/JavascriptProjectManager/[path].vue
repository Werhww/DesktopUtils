<script setup lang="ts">
import type {
	Projects,
	PackageJson,
} from "@/utils/modules/JavascriptProjectManager"
import { readPackageJson } from "@/utils/modules/JavascriptProjectManager"
import { show } from "@tauri-apps/api/app"

const router = useRouter()
const { path } = useRoute().params

const fetchCopy = ref<Projects>()
const project = ref<Projects>()
const loading = ref(true)

const sync = computed(() => {
	return {
		state: false,
		label: "Last sync 5s ago",
	}
})

async function main() {
	loading.value = true

	const content = await readPackageJson(path as string)
	if (content === "error") {
		loading.value = false
		return
	}

	project.value = content
	fetchCopy.value = content

	loading.value = false
}

const mainPropsEdit = ref({
    show: false,
    label: "",
    info: "",
    value: ref(),
    openState: ref(false),
})

const mainPropsTooltip = ref({
	show: false,
	message: "",
})

function openMainPropsEdit(value: Ref<string | undefined>, label: string, open: Ref<boolean>, info: string = "") {
    mainPropsEdit.value.openState = false

    mainPropsEdit.value.show = true
    mainPropsEdit.value.label = label
    mainPropsEdit.value.info = info
    // @ts-ignore
    mainPropsEdit.value.value = value
    // @ts-ignore
    mainPropsEdit.value.openState = open
    open.value = true
}

function closeMainPropsEdit() {
    mainPropsEdit.value.show = false
    mainPropsEdit.value.openState = false
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
		<QBtn icon="settings" dense flat rounded> </QBtn>

		<div class="flex items-end pt-3">
			<QIcon
				v-if="!sync.state"
				name="sync"
				size="16px"
				color="grey-5"
				class="pb-1"
			/>
			<div v-else class="pb-1">
				<QSpinner size="16px" color="grey-5" />
			</div>
			<div class="text-caption text-weight-medium text-grey-5">
				{{ sync.label }}
			</div>
			<QTooltip>Changes are automatically update to Json file</QTooltip>
		</div>
	</div>
	<div v-if="project">
		<div class="text-h4 text-capitalize">{{ project.data.name }}</div>
		<div class="text-underline text-grey-5 text-weight-medium">
			{{ project.path }}
		</div>

		<div class="grid grid-cols-5 mt-3 relative">
			<JavascriptManagerBasicProperty
				icon="sym_r_description"
				label="Description"
				v-model="project.data.description"
                @editProperty="openMainPropsEdit"
                @close="closeMainPropsEdit"
                info="A short description of the project"
			/>
			<JavascriptManagerBasicProperty
				defaultValue="Your Name <example@email.com> (https://yourwebsite.com)"
				icon="sym_r_signature"
				label="Author"
				v-model="project.data.author"
                @editProperty="openMainPropsEdit"
                @close="closeMainPropsEdit"
                info="The author of the project"
			/>
			<JavascriptManagerBasicProperty
				icon="sym_r_license"
				label="License"
				v-model="project.data.license"
                @editProperty="openMainPropsEdit"
                @close="closeMainPropsEdit"
                info="The license type of the project"
			/>
			<JavascriptManagerBasicProperty
				icon="sym_r_house"
				label="Homepage"
				v-model="project.data.homepage"
                @editProperty="openMainPropsEdit"
                @close="closeMainPropsEdit"
                info="The URL to the project homepage"
			/>

			<!-- TODO - Change to open file explorer, plus get relative path, eks: "./main.js" -->
			<JavascriptManagerBasicProperty
               
                
				icon="sym_r_draft"
				label="Main"
				v-model="project.data.main"
			/>
		</div>
		<QSlideTransition>
            <div v-show="mainPropsEdit.show" class="mx-6">
                <div class="text-h6">{{ mainPropsEdit.label }}</div>
                <QInput dense v-model="mainPropsEdit.value" :hint="mainPropsEdit.info" />
            </div>
        </QSlideTransition>
	</div>

	<!-- {{ project }} -->
</template>

<style scoped lang="scss"></style>
