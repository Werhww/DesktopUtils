<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri"

import FolderSearch from "@/assets/svg/folder-search.svg"

const selected = ref()
const ticked = ref([])
const expanded = ref([])

const simple = [
	{
		label: "C:",
		children: [
			{
				label: "/code",
				children: [
					{ label: "/constructopia" },
					{ label: "/kys-pizzakalulator" },
				],
			},
		],
	},
]

async function searchTest() {
	const test = await invoke("list_node_modules")
	console.log(test)
}
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
		:nodes="simple"
		control-color="grey-8"
		:duration="100"
		v-model:selected="selected"
		v-model:ticked="ticked"
		v-model:expanded="expanded"
	/>
</template>

<style scoped lang="scss"></style>
