<script setup lang="ts">
interface SkippedFolder {
	name: string
	active: boolean
}

interface SkippedFolderListProps {
	defaults: SkippedFolder[]
	custom: SkippedFolder[]
}

const skipped_folders = defineModel()

</script>

<template>

</template>

<style scoped lang="scss">

</style>