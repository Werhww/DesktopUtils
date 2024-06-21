<script setup lang="ts">
const props = withDefaults(
	defineProps<{
		item: FileExplorerItem
		first?: boolean
		hideFiles?: boolean
	}>(),
	{
		first: false,
		hideFiles: false,
	}
)

const selectedPath = defineModel<FileExplorerItem[]>()
const containsFolder = props.item.children.some(
	(child) => child.type === "folder"
)

const emit = defineEmits<{
	select: [item: FileExplorerItem, selectRef: Ref<boolean>]
}>()

function selectFile(item: FileExplorerItem, selectRef: Ref<boolean>) {
	emit("select", item, selectRef)
}

function onClick() {
	if (props.hideFiles) {
		if (!containsFolder) {
			selectFile(props.item, ref(selected.value))
			return
		}
	}
	open.value = !open.value
}

const selected = computed(() => {
	if (!selectedPath.value) return false
	return selectedPath.value.some((path) => path.path === props.item.path)
})

const open = ref(false)
</script>

<template>
	<div>
		<div
			class=" border-zinc-800 row items-center justify-between cursor-pointer group hover:bg-zinc-700 duration-200 select-none"
			:class="{
				'border-t-[1px] bg-zinc-700 rounded-md mb-1 hover:-translate-y-1 hover:drop-shadow-md':
					selected,
			}"
		>
			<div
				@click="onClick"
				class="p-1 row items-center gap-1 duration-200 flex-1"
				:class="{
					'p-2 ': selected,
				}"
			>
				<QIcon
					name="sym_r_reply"
					size="xs"
					class="rotate-180"
					v-if="!first"
				/>
				<QIcon name="sym_r_folder" size="xs" />
				<div>{{ item.name }}</div>
			</div>

			<QIcon
				@click="selectFile(item, ref(selected))"
				v-if="hideFiles && containsFolder"
				name="add"
				size="xs"
				class="opacity-0 group-hover:opacity-100 duration-200 p-1"
                :class="{
                    'rotate-45': selected
                }"
			/>
		</div>
		<QSlideTransition :duration="200">
			<div v-if="open" class="pl-3">
				<div v-for="child in item.children">
					<FileExplorerFile
						v-if="child.type == 'file' && !hideFiles"
						:item="child"
						v-model="selectedPath"
						@select="selectFile"
					/>
					<FileExplorerFolder
						v-else-if="child.type == 'folder'"
						:item="child"
						v-model="selectedPath"
						:hideFiles="hideFiles"
						@select="selectFile"
					/>
				</div>
			</div>
		</QSlideTransition>
	</div>
</template>

<style scoped lang="scss"></style>
