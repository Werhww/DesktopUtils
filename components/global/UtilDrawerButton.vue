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

type shownIcon = 'main' | 'mainOpen' | 'custom' | 'customOpen'

const openState = computed<shownIcon>(() => {
    let open = route.path === props.to


    if(open == false) {
        if(props.customIcon) {
            return 'custom'
        } else {
            return 'main'
        }
    } else {
        if(props.customOpenIcon && props.customIcon) {
            return 'customOpen'
        } else if(props.customOpenIcon) {
            return 'mainOpen'
        } else if(props.customIcon) {
            return 'custom'
        } else {
            return 'main'
        }
    }
})

watch(() => openState.value, () => {
    console.log(props.to)
    console.log(openState.value)
})
</script>

<template>
<QItem clickable :to="to" v-ripple class="q-px-sm overflow-hidden rounded-borders" :class="{
    'bg-accent rounded-borders': route.path === props.to,
}" style="max-height:48px">
    <QItemSection avatar style="min-width: unset; max-height: 32px;" class="q-pr-none">
        <QIcon size="md" v-if="openState == 'main'"  :name="icon" :color="iconColor" />    
        <QIcon size="md" v-if="openState == 'mainOpen'" class="" :name="openIcon" :color="iconColor" />
        <slot name="icon" v-if="openState == 'custom'"></slot>
        <slot name="openIcon" v-if="openState == 'customOpen'"></slot>
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