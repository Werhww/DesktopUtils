<script setup lang="ts">
import type { SavedSearch } from "@/utils/modules/JavascriptProjectManager"
import FolderSearch from "@/assets/svg/folder-search.svg"
import { invoke } from "@tauri-apps/api/tauri"
import { useStorage } from '@vueuse/core'
import {
	Notify,
	Loading,
	QSpinnerGears
} from 'quasar'

const skipped_folders = useStorage<SkipFolderLists>("skipped_folders", { 
	defaults: [
		{
			name: ".vscode",
			active: true,
		},
		{
			name: "AppData",
			active: true,
		},
		{
			name: "Program Files",
			active: true,
		},
		{
			name: "Program Files (x86)",
			active: true,
		},
		{
			name: "$RECYCLE.BIN",
			active: true,
		},
		{
			name: "$Recycle.Bin",
			active: true,
		},
		{
			name: "NVIDIA Corporation",
			active: true,
		}
	],

	custom: []
})

const saved_search = useStorage<SavedSearch[]>("JSManager_saved_search", [])
const last_search_id = useStorage<string>("JSManager_last_search_id", '')

const search = ref("")

const results = ref<string[]>([])

async function searchComputer() {
	const loadingTimer = Loading.show({
		spinner: QSpinnerGears,
	})

	let secondsTimer = 0
	const interval = setInterval(() => {
		secondsTimer += 0.1
		loadingTimer({ message: `Searching for package.json... ${secondsTimer.toFixed(1)}s` })
	}, 100)
	
	cleansSavedSearches()
	results.value = []
	const packageJsons = await invoke("find_package_jsons_entier_computer", { pathsToSkip: getSkipFoldersList(skipped_folders.value) }) as string[]
	
	results.value = packageJsons
	saveSearch(packageJsons, true)

	clearInterval(interval)
	Loading.hide()
}

function saveSearch(paths: string[], fullSearch: boolean) {
	const id = Math.random().toString(36).substring(7)
	last_search_id.value = id
	saved_search.value.push({
		id: id,
		name: `${fullSearch ? "Full pc search" : "Folder search" } ${new Date().toISOString()}`,
		datetime: new Date().toISOString(),
		keep: false,
		projectPaths: paths
	})
}

function cleansSavedSearches() {
	const now = new Date()
	const savedSearches = saved_search.value
	for(const search of savedSearches) {
		const datetime = new Date(search.datetime)
		const diff = now.getTime() - datetime.getTime()
		if(diff > 1000 * 60 * 60 * 24 * 7) {
			saved_search.value = savedSearches.filter(s => s.id !== search.id)
		}
	}

}

function searchProjectFolder() {
	console.log("searchProjectFolder")
}

onMounted(() => {
	if(last_search_id.value) {
		const search = saved_search.value.find(s => s.id === last_search_id.value)
		if(search) {
			Notify.create({
				message: `Opened: ${search.name}`,
				color: "primary",
				position: "top",
				timeout: 2000
			})
			results.value = search.projectPaths
		}
	}
})
cleansSavedSearches()

</script>

<template>
	<QInput
		v-model="search"
		rounded
		standout
		dense
		class="absolute-top-right q-mt-sm q-mr-sm"
		style="max-height: none"
		placeholder="Search"
	>
		<template v-slot:append>
			<QIcon name="search" />
		</template>
	</QInput>
	<div class="text-h5 jacquard24">Javascript Manager</div>
	<div class="row">
		<QBtn icon="sym_r_travel_explore" dense flat rounded @click="searchComputer">
			<QTooltip>Search entire computer</QTooltip>
		</QBtn>
		<QBtn :icon="`img:${FolderSearch}`" dense flat rounded @click="searchProjectFolder">
			<QTooltip>Search specific project folder</QTooltip>
		</QBtn>
		<QBtn icon="sym_r_history" dense flat rounded>
			<QTooltip>Search history</QTooltip>
			<QMenu max-width="430px">
				<JavascriptManagerSearchHistory @openSearch="(v) => {
					results = v.projectPaths
					last_search_id = v.id
				}" />
			</QMenu>
		</QBtn>
		<QBtn icon="settings" dense flat rounded>
			<QMenu>
				<SkippedFolder v-model="skipped_folders" />
			</QMenu>
		</QBtn>
	</div>

	<div class="q-px-xs q-pt-sm q-gutter-y-sm" >
		<JavascriptManagerProjectCard v-for="path in results" :path="path" :search="search" />
	</div>
</template>

<style scoped lang="scss"></style>
