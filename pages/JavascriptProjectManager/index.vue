<script setup lang="ts">
import FolderSearch from "@/assets/svg/folder-search.svg"
import { invoke } from "@tauri-apps/api/tauri"
import { useStorage } from '@vueuse/core'

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

interface PackageJsonItem {
	key: string
	value: string
}

interface Projects {
	path: string
	name: string
	scripts: PackageJsonItem[]
	dependencies: PackageJsonItem[]
	devDependencies: PackageJsonItem[]
}

const search = ref("")
const results = ref<Projects[]>([])

async function searchComputer() {
	const packageJsons = await invoke("find_package_jsons_entier_computer", { pathsToSkip: getSkipFoldersList(skipped_folders.value) }) as string[]

	const projects: Projects[] = []

	for(const path in packageJsons) {
		const fileContent = await invoke("read_package_json", { filePath: path }) as string
		console.log(fileContent)
	}
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

	<div class="q-px-xs" ><!-- 
		<JavascriptManagerProjectCard v-for="path in results" :path="path" /> -->
	</div>
</template>

<style scoped lang="scss"></style>
