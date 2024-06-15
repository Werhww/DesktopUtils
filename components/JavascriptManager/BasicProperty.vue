<script setup lang="ts">
const props = defineProps<{
	icon: string
	label: string
	info?: string
	defaultValue?: string	
}>()

const emit = defineEmits<{
	editProperty: [value: Ref<string | undefined>, label: string, open: Ref<boolean>, info: string | undefined]
	close: []
}>()

const value = defineModel<string | undefined>({ required: true })
const open = ref(false)
const showClose = ref(false)

function editProperty() {
	if(!value.value) {
		value.value = props.defaultValue || " "
	}

	emit("editProperty", value, props.label, open, props.info)
}

watch(open, () => {
	if(open.value) {
		setTimeout(() => {
			showClose.value = true
		}, 200)
	} else {
		setTimeout(() => {
			showClose.value = false
		}, 200)
	}
})

</script>

<template>
	<div v-if="!showClose" class="duration-200" :class="{
		'opacity-0': open,
	}">
		<div @click="editProperty" :class="{
			'hover:opacity-100 cursor-pointer': value,
		}" class="flex flex-col items-center opacity-40 duration-200">
			<QIcon :name="icon" size="md" />
			<div>{{ label }}</div>
			<QTooltip v-if="!value" class="text-center">Missing propery <br/> Click to add</QTooltip>
		</div>
	</div>
	<div v-else class="flex items-center justify-center hover:bg-zinc-700 duration-200 cursor-pointer" :class="{
		'opacity-0': !open,
	}" @click="$emit('close')">
		<QIcon name="sym_r_close" size="sm" />
	</div>
</template>

<style scoped lang="scss"></style>
