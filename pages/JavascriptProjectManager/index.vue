<script setup lang="ts">
import FolderSearch from "@/assets/svg/folder-search.svg"

const search = ref("")

</script>

<template>
<QInput v-model="search" rounded standout dense class="absolute-top-right q-mt-sm q-mr-sm" style="max-height: none;" placeholder="Search"> 
    <template v-slot:append>
        <QIcon name="search" />
    </template>
</QInput>
<div class="text-h5 jacquard24">Javascript Manager</div>
<div class="row">
    <QBtn icon="sym_r_travel_explore" dense flat rounded >
        <QTooltip>Search entire computer</QTooltip>
    </QBtn>
    <QBtn :icon="`img:${FolderSearch}`" dense flat rounded >
        <QTooltip>Search specific project folder</QTooltip>
    </QBtn>
    <QBtn icon="sym_r_history" dense flat rounded >
        <QTooltip>Search history</QTooltip>
    </QBtn>
</div>
</template>

<style scoped lang="scss">

</style>