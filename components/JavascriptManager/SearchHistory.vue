<script setup lang="ts">
import type { SavedSearch } from "@/types/JavascriptProjectManager"
import { useStorage } from '@vueuse/core'
const saved_search = useStorage<SavedSearch[]>("JSManager_saved_search", [])
const saved_searches_checkbox_list = ref<boolean[]>([])

const select_all = ref(false)
const selected_searches = computed(() => {
    const ids = saved_search.value.filter((_, index) => saved_searches_checkbox_list.value[index])
    
    if(ids.length === 0) {
        select_all.value = false
    } else if(ids.length === saved_search.value.length) {
        select_all.value = true
    } else {
        select_all.value = false
    }

    return ids
})

onMounted(() => {
    saved_searches_checkbox_list.value = saved_search.value.map(() => false)
})

function markAll() {
    if(select_all.value) {
        saved_searches_checkbox_list.value = saved_search.value.map(() => true)
    } else {
        saved_searches_checkbox_list.value = saved_search.value.map(() => false)
    }
}

function deleteMarked() {
    saved_search.value = saved_search.value.filter((_, index) => !saved_searches_checkbox_list.value[index])
    saved_searches_checkbox_list.value = saved_search.value.map(() => false)
}

defineEmits<{
    openSearch: [search: SavedSearch]
}>()
</script>

<template>
<QList style="min-width: 430px;">
    <QItem dense class="q-pb-none">
        <QItemSection class="text-h6">
            Search history
        </QItemSection>
    </QItem>
    <QItem dense class="q-pt-none">
        <QItemSection class="text-caption text-grey-5 text-weight-medium">
            Searches will be deleted after 7 days. <br/>You can keep searches by toggling the keep switch.
        </QItemSection>
    </QItem>

    <QItem dense>
        <QItemSection side >
            <QCheckbox v-model="select_all" color="blue-10" @click="markAll" />
        </QItemSection>
        <QItemSection>
            Name
        </QItemSection>
        <QItemSection side >
            <QBtn icon="delete" flat round :disabled="selected_searches.length == 0" @click="deleteMarked" >
                <QTooltip>Delete selected</QTooltip>
            </QBtn>
        </QItemSection>
    </QItem>

    <QSeparator inset />

    <QItem v-for="(search, index) in saved_search" :key="search.id" clickable dense>
        <QMenu cover class="no-shadow" separate-close-popup>
            <QItem>
                <QItemSection side class="items-center q-pl-xs" >
                    <div>Keep</div>
                    <QCheckbox v-model="search.keep" dense color="green" size="xs" />
                </QItemSection>
                <QItemSection>
                    <QInput v-model="search.name" dense />
                </QItemSection>
                <QItemSection side >
                    <QBtn icon="close" v-close-popup flat round />
                </QItemSection>
            </QItem>
        </QMenu>
        
        <QItemSection side >
            <QCheckbox v-model="saved_searches_checkbox_list[index]" color="blue-10" />
        </QItemSection>
        <QItemSection class="ellipsis" v-close-popup @click="$emit('openSearch', search)">
            {{ search.name }}
        </QItemSection>
        <QItemSection side >
            <QBtn icon="edit" flat round />
        </QItemSection>
    </QItem>
</QList>
</template>

<style scoped lang="scss">

</style>