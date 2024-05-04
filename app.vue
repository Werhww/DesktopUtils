<script setup lang="ts">
const leftDrawerOpen = ref(false)
const miniState = ref(true)
</script>

<template>
	<QLayout view="lHh lpR lFf">
		<QDrawer
			show-if-above
			v-model="leftDrawerOpen"
			side="left"
			behavior="desktop"
			:elevated="!miniState"
			bordered
      		mini-to-overlay
      
			:width="190"
			:mini-width="49"
			:mini="miniState"
		>
			<QBtn icon="sym_r_left_panel_open" dense flat @click="miniState = !miniState"/>
			<QList padding>
				<UtilDrawerButton
					icon="sym_r_folder"
					openIcon="sym_r_folder_open" 
					to="/"

					labelFont="nico-moji"
					
				>
					<template #label>
						<QItemSection class="text-subtitle2 text-white nico-moji text-center">
							<QItemLabel class="text-no-wrap">Node Module</QItemLabel>
							<QItemLabel>Cleanser</QItemLabel>
						</QItemSection>
					</template>
				</UtilDrawerButton>
				<UtilDrawerButton
					icon="sym_r_folder"
					openIcon="sym_r_folder_open" 
					to="/rtest"

					labelFont="nico-moji"
					label="R Test"
					
				>
					
				</UtilDrawerButton>

			</QList>
	</QDrawer>

		<QPageContainer>
			<NuxtPage />
		</QPageContainer>
	</QLayout>
</template>
