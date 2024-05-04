<script setup lang="ts">
withDefaults(defineProps<{
    icon: string
    customIcon?: boolean

    openIcon?: string
    customOpenIcon?: boolean

    label?: string
    to: string

    labelFont?: string
    iconColor?: string


}>(), {
    customIcon: false,
    customOpenIcon: false,

    iconColor: 'white',
    labelFont: 'Inter, sans-serif'
})
</script>

<template>
<QItem clickable :to="to"  v-ripple class="q-px-sm">
    <QItemSection avatar style="min-width: unset; max-height: 32px;" class="q-pr-none">
        <QIcon size="md"  :name="icon" :color="iconColor" />
<!--         <QIcon size="md" class="" :name="openIcon" :color="iconColor" />
 -->    
    </QItemSection>


    <slot name="label"></slot>
    <QItemSection v-if="label" style="max-height: 32px;" class="text-subtitle2 text-center text-white" :class="labelFont" >
        <QItemLabel :lines="1"  :class="labelFont">
            {{ label }}
        </QItemLabel>
        
    </QItemSection>
</QItem>
</template>

<style scoped lang="scss">

</style>