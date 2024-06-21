<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri"
import { Notify } from "quasar"

interface Props {
	path: string
	undertitle: string
	pathsToSkip: string[]
	folderSelect?: boolean
	multiple?: boolean
	maxFiles?: number
}

const props = withDefaults(defineProps<Props>(), {
	multiple: false,
	folderSelect: false,
})

const emit = defineEmits<{
	close: []
	file: [path: FileExplorerItem]
	files: [paths: FileExplorerItem[]]
}>()

const folderName = computed(() => {
	return props.path.split("/").pop()
})

const loading = ref(false)
const structuredPaths = ref<FileExplorerItem[]>([])
const selectedPath = ref<FileExplorerItem[]>([])

function selectFile(item: FileExplorerItem, selectRef: Ref<boolean>) {
	if (
		props.multiple &&
		props.maxFiles &&
		selectedPath.value.length >= props.maxFiles
	) {
		Notify.create({
			message: `You can only select ${props.maxFiles} files`,
			color: "negative",
		})

		return
	}

	if (props.multiple) {
		if (selectRef.value) {
			selectedPath.value = selectedPath.value.filter(
				(path) => path.path !== item.path
			)
		} else {
			selectedPath.value.push(item)
		}
	} else {
		selectedPath.value = [item]
	}
}

function removeFile(item: FileExplorerItem) {
	selectedPath.value = selectedPath.value.filter(
		(path) => path.path !== item.path
	)
}

const ScrollHeight = computed(() => {
	return totalHeight.value - headerHeight.value - footerHeight.value - 12
})

const headerHeight = ref(0)
const footerHeight = ref(0)
const totalHeight = ref(0)

function headerResize(size: { height: number }) {
	headerHeight.value = size.height
}

function footerResize(size: { height: number }) {
	footerHeight.value = size.height
}

function totalResize(size: { height: number }) {
	totalHeight.value = size.height
}

function orderStructerdPaths(items: FileExplorerItem[]) {
	let orderedPaths: FileExplorerItem[] = []
	let laterPaths: FileExplorerItem[] = []

	items.forEach((item) => {
		const realItem = JSON.parse(JSON.stringify(item))

		const isFile = realItem.children.length == 0
		if (isFile) {
			orderedPaths.push(realItem)
		} else {
			laterPaths.push(realItem)
		}
	})

	laterPaths.forEach((item) => {
		const realItem = JSON.parse(JSON.stringify(item))

		const folder = orderStructerdPaths(realItem.children)

		orderedPaths.push({
			name: realItem.name,
			path: realItem.path,
			type: realItem.type,
			children: folder,
		})
	})

	return orderedPaths
}

/* 
    Walk dir to get all file paths 
    and create a structured paths
*/
onMounted(async () => {
	const paths = (await invoke("get_dir_files", {
		path: props.path,
		pathsToSkip: props.pathsToSkip,
	})) as string[]

	const skippedPaths = splitFilePath(props.path)

	Promise.all(
		paths.map(async (path) => {
			let currentLevel = structuredPaths.value
			const pathParts = splitFilePath(path)

			pathParts.forEach((part, index) => {
				if (skippedPaths.includes(part)) {
					return
				}

				let existingPart = currentLevel.find((p) => p.name === part)

				if (!existingPart) {
					const newPath = {
						name: part,
						path: pathParts.slice(0, index + 1).join("/"),
						type: "file" as "file" | "folder",
						children: [] as FileExplorerItem[],
					}
					currentLevel.push(newPath)
					existingPart = newPath
				}

				if (index < pathParts.length - 1) {
					existingPart.type = "folder"

					currentLevel = existingPart.children
				}
			})
		})
	)

	const orderedPaths = orderStructerdPaths(structuredPaths.value)

	structuredPaths.value = orderedPaths

	loading.value = true
})
</script>

<template>
	<div class="bg-zinc-800 pt-3 overflow-hidden w-full h-full scroller">
		<div class="row items-center px-3">
			<div class="flex-1">
				<div class="row items-center">
					<QIcon name="sym_r_folder" size="lg" />
					<div class="text-lg">{{ folderName }}</div>
				</div>
				<div class="text-sm">
					{{ undertitle }}
				</div>
			</div>

			<div
				class="p-2 rounded-full h-fit bg-red-900 cursor-pointer hover:-translate-y-2 hover:drop-shadow-xl duration-300"
			>
				<QIcon name="sym_r_close" class="rotate-180" size="sm" />
			</div>

			<QResizeObserver @resize="headerResize" />
		</div>

		<QSeparator />

		<div
			v-if="loading"
			class="overflow-y-auto px-3"
			:style="{
				height: ScrollHeight + 'px',
			}"
		>
			<div v-for="item in structuredPaths" class="">
				<FileExplorerFile
					v-if="item.type == 'file' && !folderSelect"
					:item="item"
					v-model="selectedPath"
					@select="selectFile"
				/>
				<FileExplorerFolder
					v-else-if="item.type == 'folder'"
					:item="item"
					first
					v-model="selectedPath"
					:hideFiles="folderSelect"
					@select="selectFile"
				/>
			</div>
		</div>

		<div v-if="loading" class="row py-2 items-center gap-2 px-3">
			<div class="row gap-2 flex-1" v-if="multiple">
				<div
					class="p-2 bg-zinc-700 rounded-md cursor-pointer hover:-translate-y-2 hover:drop-shadow-xl duration-300"
					@click="removeFile(item)"
					v-for="item in selectedPath"
				>
					{{ item.name }}
				</div>
			</div>

			<div class="row flex-1" v-else>
				<FileExplorerSingleSelect
					v-for="item in selectedPath"
					:item="item"
				/>
			</div>

			<div
				class="p-2 rounded-full bg-green-900 cursor-pointer hover:-translate-y-2 hover:drop-shadow-xl duration-300"
			>
				<QIcon name="sym_r_reply" class="rotate-180" size="sm" />
			</div>

			<QResizeObserver @resize="footerResize" />
		</div>

		<div
			v-if="!loading"
			class="column gap-3 items-center justify-center w-full"
			:style="{
				height: ScrollHeight + 'px',
			}"
		>
		<div class="text-lg" v-if="folderSelect">Loading folder...</div>
			<div class="text-lg" v-else>Loading files...</div>
		</div>

		<QResizeObserver @resize="totalResize" />
	</div>
</template>

<style scoped lang="scss"></style>
