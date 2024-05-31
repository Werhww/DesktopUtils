import { invoke } from "@tauri-apps/api/tauri"

export interface SavedSearch {
	id: string
	name: string
	datetime: string
	keep: boolean
	projectPaths: string[] 
}

export interface Projects {
	path: string

    data: Partial<PackageJson>
}

export interface PackageJson {
    name: string;
    version: string;
    description: string;
    main: string;
    scripts: { [key: string]: string };
    repository: { type: string; url: string };
    keywords: string[];
    author: string;
    license: string;
    bugs: { url: string };
    homepage: string;
    dependencies: { [key: string]: string };
    devDependencies: { [key: string]: string };
    peerDependencies: { [key: string]: string };
    optionalDependencies: { [key: string]: string };
    engines: { [key: string]: string };
    private: boolean;
}

export async function readPackageJson(path: string): Promise<Projects | "error"> {
	const fileContent = await invoke("read_file", { filePath: path }) as string

    if(fileContent === "") {
        Notify.create({
			message: "Error reading file, file may be empty or not found.",
			color: "negative",
			position: "top",
			icon: "error"
		})
        
        return "error"
    }

	const packageJson = JSON.parse(fileContent)

	return {
        path: path,
        data: packageJson,
    }
}
  