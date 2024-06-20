<script setup lang="ts">
const props = withDefaults(defineProps<{
	item: FileExplorerItem
    first?: boolean
}>(), {
    first: false
})

const selectedPath = defineModel<FileExplorerItem[]>()

const emit = defineEmits<{
    select: [item: FileExplorerItem, selectRef: Ref<boolean>]
}>()

function selectFile(item: FileExplorerItem, selectRef: Ref<boolean>) {
    emit("select", item, selectRef)
}

const open = ref(false)
</script>

<template>
	<div>
        <div class="p-1 row items-center gap-1 cursor-pointer hover:bg-zinc-700 duration-200" @click="open = !open">
            <QIcon name="sym_r_reply" size="xs" class="rotate-180" v-if="!first" />
            <QIcon name="sym_r_folder" size="xs" />
            <div>{{ item.name }}</div>
        </div>
        <QSlideTransition :duration="200">
            <div v-if="open" class="pl-3">
                <div v-for="child in item.children">
                    <FileExplorerFile v-if="child.children.length == 0" :item="child" v-model="selectedPath" @select="selectFile" />
                    <FileExplorerFolder v-else :item="child" v-model="selectedPath" @select="selectFile" />
                </div>
            </div>
        </QSlideTransition>
    </div>
</template>

<style scoped lang="scss"></style>
