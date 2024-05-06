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
			
      		mini-to-overlay
      
			:width="190"
			:mini-width="49"
			:mini="miniState"
		>
			<div class="row items-center no-wrap">
				<QBtn icon="sym_r_left_panel_open" v-if="miniState" size="lg" dense flat @click="miniState = !miniState"/>
				<QBtn icon="sym_r_left_panel_close" v-else size="lg" dense flat @click="miniState = !miniState"/>
				<div class="text-h5" v-if="!miniState">Utils</div>
			</div>
			<QList padding>
				<UtilDrawerButton
					to="/NMCleanser"
				
					iconColor="light-green-13"
					labelFont="nico-moji"

					customIcon
					:customOpenIcon="true"
				>
					<template #icon>
						<QIcon name="sym_r_folder" class="q-pt-xs" size="md" color="light-green-13">
							<QBadge class="q-pa-none" style="background: none;"  floating><QIcon color="light-green-13" name="javascript" size="sm"/></QBadge>
						</QIcon>
					</template>

					<template #openIcon>
						<QIcon name="sym_r_folder_open" class="q-pt-xs" size="md" color="light-green-13">
							<QBadge class="q-pa-none" style="background: none;"  floating><QIcon color="light-green-13" name="javascript" size="sm"/></QBadge>
						</QIcon>
					</template>

					<template #label>
						<QItemSection class="column no-wrap text-subtitle2 text-white nico-moji text-center">
							<QItemLabel class="text-no-wrap">Node Module</QItemLabel>
							<QItemLabel>Cleanser</QItemLabel>
						</QItemSection>
					</template>
				</UtilDrawerButton>

			</QList>
	</QDrawer>

		<QPageContainer>
			<NuxtPage />
		</QPageContainer>
	</QLayout>
</template>
