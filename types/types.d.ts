declare global {
	type SkipFolderItem = {
		name: string
		active: boolean
	}
	
	type SkipFolderLists = {
		defaults: SkipFolderItem[]
		custom: SkipFolderItem[]
	}

	interface FileExplorerItem {
		name: string
		path: string
		children: FileExplorerItem[]
	}
}

export {};