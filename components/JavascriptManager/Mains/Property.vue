<script setup lang="ts">
const props = defineProps<{
	icon: string
	label: string
	hint: string
}>()

const value = defineModel<string | undefined>({ required: true })

const open = ref(false)
const secondOpen = ref(false)

const emit = defineEmits<{
	edit: [
		icon: string,
		label: string,
		hint: string,
		value: Ref<string | undefined>,
		open: Ref<boolean>
	]
	close: []
}>()

function edit() {
	open.value = true

	emit("edit", props.icon, props.label, props.hint, value, open)
}

function close() {
	open.value = false
	emit("close")
}

watch(open, () => {
	if (open.value) {
		setTimeout(() => {
			secondOpen.value = true
		}, 200)
	} else {
		setTimeout(() => {
			secondOpen.value = false
		}, 200)
	}
})
</script>

<template>
	<div
		v-if="!secondOpen"
		class="duration-200"
		:class="{
			'opacity-0': open,
		}"
	>
		<div
			:class="{
				'hover:opacity-100': value,
			}"
			class="flex flex-col items-center opacity-40 duration-200 relative cursor-pointer"
			@click="edit"
		>
			<div v-if="!value" class="absolute text-xs bg-zinc-700">
				Missing propery
			</div>
			<QIcon :name="icon" size="md" />
			<div>{{ label }}</div>
		</div>
	</div>

	<div
		v-else
		@click="close"
		class="flex items-center justify-center hover:bg-zinc-700 duration-200 cursor-pointer"
		:class="{
			'opacity-0': !open,
		}"
	>
		<QIcon name="sym_r_close" size="sm" />
	</div>
</template>

<style scoped lang="scss"></style>
