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

export interface PackageJson extends SimpleField {
    directories: { [key: string]: string };
    scripts: { [key: string]: string };
    dependencies: { [key: string]: string };
    devDependencies: { [key: string]: string };
    peerDependencies: { [key: string]: string };
    optionalDependencies: { [key: string]: string };
    engines: { [key: string]: string };
    repository: string | { type: string, url: string };
    
    keywords: string[];
    bugs: { url: string, email: string };

    contributors: {
        name: string;
        email?: string;
        url?: string;
    }[]

    funding: funding

    files: string[];
    bin: { [key: string]: string } | string;
}

// Types for the funding field in package.json
export type funding = Array<{ type: string, url: string } | string> | string | { type: string, url: string };


interface SimpleField {
    name: string;
    description: string;
    version: string;
    
    author: string;
    license: string;
    homepage: string;
    
    main: string;
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
  