import { is } from "quasar";

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

export function highlightSearchTerm(text: string, searchTerm: string) {
	const normalizedTitle = String(text).toLowerCase();
    const normalizedSearchTerm = String(searchTerm).toLowerCase();

    const startIndex = normalizedTitle.indexOf(normalizedSearchTerm);

    if (startIndex === -1) {
        return {
			html: text,
			isMatch: false
		};
    }

    const before = text.substr(0, startIndex);
    const match = text.substr(startIndex, searchTerm.length);
    const after = text.substr(startIndex + searchTerm.length);

    return {
		html: before + '<span class="bg-yellow-7 text-black">' + match + '</span>' + after,
		isMatch: true
	}
}