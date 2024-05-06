<script setup lang="ts">
const props = withDefaults(defineProps<{
    icon?: string
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

const route = useRoute()

const openState = computed(() => {

    if(props.customOpenIcon) return route.path === props.to

    if(!props.openIcon) return false 
    return route.path === props.to
})
</script>

<template>
<QItem clickable :to="to" v-ripple class="q-px-sm overflow-hidden" :class="{
    'bg-accent rounded-borders': openState,
}" style="max-height:48px">
    <QItemSection avatar style="min-width: unset; max-height: 32px;" class="q-pr-none">
        <QIcon size="md" v-if="!openState && !customIcon"  :name="icon" :color="iconColor" />    
        <QIcon size="md" v-if="openState && !customOpenIcon" class="" :name="openIcon" :color="iconColor" />
        <slot name="icon" v-if="!openState && customIcon"></slot>
        <slot name="openIcon" v-if="openState && customOpenIcon"></slot>
    </QItemSection>

    <slot name="label"></slot>
    <QItemSection v-if="label" style="max-height: 32px;" class="text-subtitle2 text-center text-white" :class="labelFont" >
        <QItemLabel :lines="1" class="text-no-wrap" :class="labelFont">
            {{ label }}
        </QItemLabel>
        
    </QItemSection>
</QItem>
</template>

<style scoped lang="scss">

</style>