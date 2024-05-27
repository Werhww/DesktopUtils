declare global {
	type SkipFolderItem = {
		name: string
		active: boolean
	}
	
	type SkipFolderLists = {
		defaults: SkipFolderItem[]
		custom: SkipFolderItem[]
	}
}

export {};