<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri"
import FolderSearch from "@/assets/svg/folder-search.svg"

const ticked = ref([])
const expanded = ref([])

const node_modules_tree = ref<any[]>([])

async function searchTest() {
	const node_modules = await invoke("list_node_modules") as string[]
	console.log(node_modules)
	let tree: any[] = []

	node_modules.forEach((path) => {
		const pathArr = path.split(/[\\\/]/)
		let current: any = tree

		pathArr.forEach((folder, index) => {
			if(folder === "node_modules") return

			if (!current.find((node: any) => node.label === folder)) {
				const content = index === pathArr.length - 2 ? {
					label: folder,
					folderPath: path,
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

	console.log(tree)
	node_modules_tree.value = tree
}

watchEffect(() => {
	console.log("checked: ", ticked.value)
	console.log("expaneded: ", expanded.value)
})

</script>

<template>
	<div class="text-h5 nico-moji">Node Module Cleanser</div>
	<div>
		<QChip
			icon="folder_copy"
			label="?"
			color="white"
			text-color="primary"
			class="text-weight-bold"
			size="md"
			style="min-width: 3.5rem"
		/>
		<QChip
			icon="sym_r_storage"
			label="?"
			color="white"
			text-color="primary"
			class="text-weight-bold"
			size="md"
			style="min-width: 3.5rem"
		/>
		<QBtn icon="delete" dense flat rounded />
		<QBtn :icon="`img:${FolderSearch}`" dense flat rounded @click="searchTest" />
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
			<div class="q-pl-xs">{{ prop.node.label }}</div>
			
		</template>

	</QTree>
</template>

<style scoped lang="scss"></style>
