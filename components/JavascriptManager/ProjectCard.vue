<script setup lang="ts">
const props = defineProps<{
    path: string
}>()


</script>

<template>
<QCard flat class="cursor-pointer">
    <QCardSection>
        {{ path }}
    </QCardSection>
</QCard>
</template>

<style scoped lang="scss">

</style>