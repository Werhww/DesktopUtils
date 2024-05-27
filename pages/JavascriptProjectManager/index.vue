<script setup lang="ts">
import FolderSearch from "@/assets/svg/folder-search.svg"
import { invoke } from "@tauri-apps/api/tauri"
import { useStorage } from '@vueuse/core'

interface SkippedFolder {
	name: string
	active: boolean
}

interface SkippedFolderListProps {
	defaults: SkippedFolder[]
	custom: SkippedFolder[]
}

const skipped_folders = useStorage<SkippedFolderListProps>("skipped_folders", { 
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

const search = ref("")
const results = ref<string[]>([])

async function searchComputer() {
	const packageJsons = await invoke("find_package_jsons_entier_computer", { pathsToSkip: getSkipFoldersList(skipped_folders.value) }) as string[]

	results.value = packageJsons
}

function searchProjectFolder() {
	console.log("searchProjectFolder")
}
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
		</QBtn>
		<QBtn icon="settings" dense flat rounded>
			<QMenu>
				<SkippedFolder v-model="skipped_folders" />
			</QMenu>
		</QBtn>
	</div>

	<div class="q-px-xs" >
		<JavascriptManagerProjectCard v-for="path in results" :path="path" />
	</div>
</template>

<style scoped lang="scss"></style>
