<script setup lang="ts">
import type { QInput } from 'quasar';

defineProps<{
	icon: string
	overline: string
	info?: string
    mask?: string
	property: string
}>()

const value = defineModel<string | undefined>({ required: true })
const edit = ref(true)

const input = ref<QInput | null>(null)

function editProperty() {
	edit.value = !edit.value

	/* if(edit.value) {
		setTimeout(() => {
			input.value?.focus()
		}, 100)
	} */

	setTimeout(() => {
			input.value?.focus()
		}, 10)

}
</script>

<template>
	<QItem clickable class="group" @click="editProperty">
		<QItemSection side>
			<QIcon :name="icon" class="group-hover:text-blue-300" />
		</QItemSection>
		<QItemSection>
			<QItemLabel overline class="group-hover:text-blue-300 row items-end">
				{{ overline }}
				<QIcon
                    v-if="info"
					class="duration-200 opacity-0 group-hover:opacity-70"
					name="sym_r_info"
					dense
					size="xs"
				>
                <QTooltip> {{ info }} </QTooltip>
                </QIcon>
			</QItemLabel>
			<QItemLabel>
				<QInput ref="input" color="blue-10" :disable="edit" :borderless="edit" placeholder="Missing property" dense v-model="value" :mask="mask" @blur="() => {
					edit = true
				}"  />
			</QItemLabel>
		</QItemSection>
		<QItemSection
			side
			class="duration-300 opacity-0 group-hover:opacity-100"
		>
			<QIcon name="edit">
				<QTooltip>Edit</QTooltip>
			</QIcon>
		</QItemSection>
	</QItem>
</template>

<style scoped lang="scss"></style>
