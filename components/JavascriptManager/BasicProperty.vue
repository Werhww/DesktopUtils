<script setup lang="ts">
defineProps<{
    icon: string
    overline: string
}>()

const value = defineModel<string | undefined>({ required: true })
</script>

<template>
<QItem clickable class="group">
    <QItemSection side>
        <QIcon :name="icon" class="group-hover:text-blue-300"/>
    </QItemSection>
    <QItemSection>
        <QItemLabel overline class="group-hover:text-blue-300">{{ overline }}</QItemLabel>
        <QItemLabel class="group-hover:text-blue-300">{{ value ? value : "Missing property" }}</QItemLabel>
    </QItemSection>
    <QItemSection side class="duration-300 opacity-0 group-hover:opacity-100">
        <QIcon v-if="value" name="edit">
            <QTooltip>Edit property</QTooltip>
        </QIcon>
        <QIcon v-else name="add">
            <QTooltip>Add property</QTooltip>
        </QIcon>
    </QItemSection>
</QItem>
</template>

<style scoped lang="scss">

</style>