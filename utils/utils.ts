export function getSkipFoldersList(list: SkipFolderLists): string[] {
	let skipped_folder_names = []

	for(const folder of list.defaults) {
		if(folder.active) skipped_folder_names.push(folder.name)
	}

	for(const folder of list.custom) {
		if(folder.active) skipped_folder_names.push(folder.name)
	}

	return skipped_folder_names
}