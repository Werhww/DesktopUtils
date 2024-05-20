<script setup lang="ts">
interface SkippedFolder {
	name: string
	active: boolean
}

interface SkippedFolderListProps {
	defaults: SkippedFolder[]
	custom: SkippedFolder[]
}

const skipped_folders = defineModel<SkippedFolderListProps>({ required: true })

</script>

<template>
<QList>
	<QItem dense>
		<QItemSection class="text-h6">Skipped Folders</QItemSection>
	</QItem>
	<QItem dense>
		<QItemSection class="text-overline">Defaults folders</QItemSection>
	</QItem>

	<QItem v-rippel clickable dense v-for="folder in skipped_folders.defaults">
		<QItemSection avatar><QToggle dense v-model="folder.active" color="light-green-13" /></QItemSection>
		<QItemSection @click="folder.active = !folder.active" :class="{'dimmed-real': !folder.active }" >{{ folder.name }}</QItemSection>
		<QItemSection side><QIcon name="delete" class="disabled"  /></QItemSection>
	</QItem>

	<QItem dense>
		<QItemSection class="text-overline">Custom folders</QItemSection>
		<QItemSection side><QIcon name="add_circle" class="cursor-pointer" @click="() => {
			skipped_folders.custom.push({ name: 'New Folder', active: true })
		}" /></QItemSection>
	</QItem>

	<QItem v-rippel clickable dense v-for="(folder, index) in skipped_folders.custom">
		<QItemSection avatar><QToggle dense v-model="folder.active" color="light-green-13" /></QItemSection>
		<QItemSection :class="{'dimmed-real': !folder.active }" >
			{{ folder.name }} 
			<QPopupEdit class="q-px-sm q-pt-sm" v-model="folder.name" auto-save v-slot="scope">
				<QInput dense v-model="scope.value" autofocus @keyup.enter="scope.set" /> 
			</QPopupEdit>
		</QItemSection>
		<QItemSection side><QIcon name="delete" class="hover-red" @click="() => {
			skipped_folders.custom.splice(index, 1)
		}"/></QItemSection>
	</QItem>
</QList>
</template>

<style scoped lang="scss">
.dimmed-real {
	opacity: 0.5;
}

.hover-red {
	transition: all 0.3s ease-in-out;

	&:hover {
		color: red;
	}
}
</style>