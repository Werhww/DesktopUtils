<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri"
import FolderSearch from "@/assets/svg/folder-search.svg"
import { useStorage } from '@vueuse/core'
import {
  Loading,
  Notify,
  QSpinnerGears
} from 'quasar'

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
const ticked = ref<string[]>([])
const expanded = ref([])

interface NodeModule {
	label: string
	icon: string
	children: NodeModule[]
}

const node_modules_tree = ref<NodeModule[]>([])

interface NodeModuleInfo {
	label: string
	path: string
	size: number
}

const node_modules_info = ref<NodeModuleInfo[]>([])


async function getFolders() {

	Loading.show({
		spinner: QSpinnerGears,
	})

	const node_modules = await invoke("list_node_modules", { pathsToSkip: getSkippedFolders() }).catch((error) => {
		Loading.hide()
		Notify.create({
			message: error,
			color: "negative",
			position: "top",
			icon: "error"
		})

		return ["error"]
	}) as string[]
	if(node_modules[0] === "error") return

	let tree: any[] = []

	node_modules_info.value = []
	node_modules_tree.value = []
	ticked.value = []
	expanded.value = []

	node_modules.forEach((path, bigIndex) => {
		const pathArr = path.split(/[\\\/]/)
		let current: any = tree

		const name = `${pathArr[pathArr.length - 2]}#--#${bigIndex}`
		invoke("folder_size", { path }).then((size: any) => {
			const folder_index = node_modules_info.value.findIndex(folder => folder.label === name)
			node_modules_info.value[folder_index].size = size
		})

		node_modules_info.value.push({
			label: name,
			path: path,
			size: 0
		})

		ticked.value.push(name)

		pathArr.forEach((folder, index) => {
			if(folder === "node_modules") return

			if (!current.find((node: any) => node.label === folder)) {
				const content = index === pathArr.length - 2 ? {
					label: `${folder}#--#${bigIndex}`,
					icon: "folder",
				} : index === 0 ? {
					label: folder,
					icon: "storage",
				} : {
					label: folder,
					icon: "account_tree"
				}
				const node = { ...content, children: [] }
				current.push(node)
			}

			if (index !== pathArr.length - 2) {
				current = current.find((node: any) => node.label === folder).children
			}
		})
	})

	node_modules_tree.value = tree
	Loading.hide()

	if(tree.length == 0) {
		Notify.create({
			message: "No node_modules folders found",
			color: "warning",
			position: "top",
			icon: "warning"
		})
	}
}

function getSkippedFolders() {
	let skipped_folder_names = []

	for(const folder of skipped_folders.value.defaults) {
		if(folder.active) skipped_folder_names.push(folder.name)
	}

	for(const folder of skipped_folders.value.custom) {
		if(folder.active) skipped_folder_names.push(folder.name)
	}

	return skipped_folder_names
}

const ticked_folder_count = computed(() => {
	const node_modules = node_modules_info.value.length
	if(node_modules === 0) return "?"
	return `${ticked.value.length} / ${node_modules_info.value.length}`
})

const ticked_folder_size = computed(() => {	
	const node_modules = node_modules_info.value
	if(node_modules.length === 0) return "?"


	let ticked_folders: NodeModuleInfo[] = []

	for(const folder_name of ticked.value) {
		const folder = node_modules.find(folder => folder.label === folder_name)
		if(folder) ticked_folders.push(folder)
	}


	const ticked_size = ticked_folders.reduce((acc, folder) => acc + folder.size, 0)
	const full_size = node_modules.reduce((acc, folder) => acc + folder.size, 0)

	let size_type = "MB"
	let full_size_formated = full_size / 1024 / 1024
	
	if(full_size_formated > 1024 * 2) {
		size_type = "GB"
		full_size_formated = full_size_formated / 1024
	}
	
	let ticked_size_formated = size_type === "MB" ? ticked_size / 1024 / 1024 : ticked_size / 1024 / 1024 / 1024

	return `${ticked_size_formated.toFixed(size_type == "MB" ? 0 : 2)} / ${full_size_formated.toFixed(size_type == "MB" ? 0 : 2)} ${size_type}`
})

async function deleteFolders() {

	if(ticked.value.length == 0) {
		return Notify.create({
			message: "No folders selected",
			color: "negative",
			position: "top",
			icon: "error"
		})
	}

	Loading.show({
		spinner: QSpinnerGears,
	})

	const folders = ticked.value.map(folder => {
		const folder_info = node_modules_info.value.find(node => node.label === folder)
		return {
			path: folder_info?.path || "",
			label: folder_info?.label|| ""
		}

	})

	for(const item of folders) {
		const completed = await invoke("delete_folder", { path: item.path }) as boolean
		if(!completed) {
			Notify.create({
				message: `Failed to delete ${item.label}`,
				color: "negative",
				position: "top",
				icon: "error"
			})
			continue
		}

 		removeNodeModuleByLabel(node_modules_tree.value, item.label)
		const index = node_modules_info.value.findIndex(node => node.label === item.label)
		node_modules_info.value.splice(index, 1)

		const ticked_index = ticked.value.findIndex(ticked_folder => ticked_folder === item.label)
		ticked.value.splice(ticked_index, 1)
	}

	Loading.hide()
}

function removeNodeModuleByLabel(tree: NodeModule[], targetLabel: string): boolean {
    if (!tree || !tree.length) return false

    for (let i = 0; i < tree.length; i++) {
		const node = tree[i]

        if (node.label === targetLabel) {
            tree.splice(i, 1);
			return true
        }

        const foundInChildren = removeNodeModuleByLabel(node.children, targetLabel);
		if (foundInChildren) return true
    }

    return false
}
</script>

<template>
	<div class="text-h5 nico-moji" @click="() => {
		console.log(skipped_folders)
	}">Node Module Cleanser</div>
	<div>
		<QChip
			icon="folder_copy"
			:label="ticked_folder_count"
			color="white"
			text-color="primary"
			class="text-weight-bold"
			size="md"
			style="min-width: 3.5rem"
		/>
		<QChip
			icon="sym_r_storage"
			:label="ticked_folder_size"
			color="white"
			text-color="primary"
			class="text-weight-bold"
			size="md"
			style="min-width: 3.5rem"
		/>
		<QBtn icon="delete" dense flat rounded @click="deleteFolders" >
			<QTooltip>Remove selected folders</QTooltip>
		</QBtn>
		<QBtn :icon="`img:${FolderSearch}`" dense flat rounded @click="getFolders" >
			<QTooltip>Search for node_modules folders</QTooltip>
		</QBtn>
		<QBtn icon="settings" dense flat rounded>
			<QMenu>
				<NMCleanserSkippedFolderList v-model="skipped_folders" />
			</QMenu>
		</QBtn>
	</div>
	<QTree
		class="col-12 col-sm-6"
		tick-strategy="leaf"
		node-key="label"
		:nodes="node_modules_tree"
		control-color="grey-8"
		:duration="100"
		v-model:ticked="ticked"
		v-model:expanded="expanded"
	>
		<template v-slot:default-header="prop">
			<QIcon :name="prop.node.icon"/>
			<div class="q-pl-xs">{{ prop.node.label.split("#--#")[0] }}</div>
			
		</template>

	</QTree>
</template>

<style scoped lang="scss"></style>
