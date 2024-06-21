<script setup lang="ts">
import type { Projects } from "~/utils/modules/JavascriptProjectManager"

const project = defineModel<Projects>({ required: true })
const folderPath = computed(() => {
	const pathArr = project.value.path.split(/[\\\/]/)

	return pathArr.slice(0, pathArr.length - 1).join("/")
})

const editOpen = ref(false)
const property = ref({
	value: ref<string | undefined>(),
	open: ref(false),
	label: "",
	hint: "",
	icon: "",
})

const labelItem = ref({
	show: false,
	text: "",
	icon: "",
})

function openEdit(
	icon: string,
	label: string,
	hint: string,
	value: Ref<string | undefined>,
	open: Ref<boolean>
) {
	if (!labelItem.value.show) {
		labelItem.value.text = label
		labelItem.value.icon = icon
		labelItem.value.show = true
	} else {
		labelItem.value.show = false
	}

	property.value.open = false

	editOpen.value = true

	property.value.label = label
	property.value.hint = hint
	property.value.icon = icon
	// @ts-ignore
	property.value.value = value
	// @ts-ignore
	property.value.open = open
}

function closeEdit() {
	editOpen.value = false
	// @ts-ignore
	property.value.open = ref(false)
}



/* Custom stuff */
const licenseDialog = ref(false)

const mainFileDialog = ref(false)
</script>

<template>
	<div class="grid grid-cols-5 mt-3">
		<JavascriptManagerMainsProperty
			v-model="project.data.description"
			icon="sym_r_description"
			label="Description"
			hint="A short description of the project"
			@edit="openEdit"
			@close="closeEdit"
		/>
		<JavascriptManagerMainsProperty
			v-model="project.data.author"
			icon="sym_r_signature"
			label="Author"
			hint="Your name <example@email.com> (https://yourwebsite.com)"
			@edit="openEdit"
			@close="closeEdit"
		/>
		<JavascriptManagerMainsProperty
			v-model="project.data.license"
			icon="sym_r_license"
			label="License"
			hint="The license type of the project"
			@edit="openEdit"
			@close="closeEdit"
		/>
		<JavascriptManagerMainsProperty
			v-model="project.data.homepage"
			icon="sym_r_house"
			label="Homepage"
			hint="The URL to the project homepage"
			@edit="openEdit"
			@close="closeEdit"
		/>
		<JavascriptManagerMainsProperty
			v-model="project.data.main"
			icon="sym_r_draft"
			label="Main"
			hint="The entry point of the project"
			@edit="openEdit"
			@close="closeEdit"
		/>
	</div>
	<QSlideTransition>
		<div v-if="editOpen" class="mx-6">
			<div class="relative" style="height: 32px">
				<TransitionGroup
					appear
					enter-active-class="animated fadeInDown"
					leave-active-class="animated fadeOutUp"
				>
					<div v-if="labelItem.show" class="text-h6 row absolute">
						<QIcon :name="labelItem.icon" size="md" />
						<span>{{ labelItem.text }}</span>
					</div>
					<div v-else class="text-h6 row absolute">
						<QIcon :name="property.icon" size="md" />
						<span>{{ property.label }}</span>
					</div>
				</TransitionGroup>
			</div>
			
			<QBtn v-if="property.label == 'Main'" dense :label="property.hint" @click="mainFileDialog = true" />
			<QInput v-else dense v-model="property.value" :hint="property.hint" />
			<TransitionGroup
				enter-active-class="animated fadeInDown"
				leave-active-class="animated fadeOutUp"
			>
				<div
					v-if="property.label == 'License'"
					class="text-underline text-grey"
				>
					Need help finding your
					<span
						@click="licenseDialog = true"
						class="underline cursor-pointer"
						>license?</span
					>
				</div>
			</TransitionGroup>

			<QDialog v-model="licenseDialog">
				<QCard class="w-full h-screen">
					<iframe
						src="https://choosealicense.com/"
						class="w-full h-full"
					></iframe>
				</QCard>
			</QDialog>
			<QDialog v-model="mainFileDialog">
				<FileExplorer
					folderSelect
					multiple
					:path="folderPath"
					undertitle="Select the main file"
					:pathsToSkip="['node_modules', '.git', '.vscode', '.nuxt', '.gitignore']"
				/>
			</QDialog>
		</div>
	</QSlideTransition>
</template>

<style scoped lang="scss"></style>
