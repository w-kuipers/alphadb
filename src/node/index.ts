import { spawn, ChildProcessWithoutNullStreams } from "child_process";
import fs from "fs";
import path from "path";

// Dynamically get pywrapper path
let pywrapperPath = path.join(path.dirname(require.resolve("alphadb/package.json")), "pywrapper");

interface AlphaDBConnectProps {
	host: string;
	user: string;
	password: string;
	database: string;
	port?: number;
}

interface AlphaDBCheck {
	check: boolean;
	current_version: string | null;
}

interface AlphaDBStatus {
	init: boolean;
	version: string;
	name: string;
	template: string;
}

interface AlphaDBUpdateProps {
	updateToVersion?: string;
	noData?: boolean;
}

type AlphaDBUpdateQueries = Array<string | Array<string>>;

function random_string(length: number = 10): string {
	let result = "";
	const characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

	const charactersLength = characters.length;
	for (let i = 0; i < length; i++) {
		result += characters.charAt(Math.floor(Math.random() * charactersLength));
	}

	return result;
}

export default class AlphaDB {
	adbInstanceId: string;

	constructor() {
		this.adbInstanceId = random_string();
	}

	destructor() {
		fs.unlink("./adbcache/" + this.adbInstanceId, () => { });
	}

	async handleChildProcess(pyprocess: ChildProcessWithoutNullStreams) {
		let data = "";
		for await (const chunk of pyprocess.stdout) {
			data += chunk;
		}
		let error = "";
		for await (const chunk of pyprocess.stderr) {
			error += chunk;
		}
		const exitCode = await new Promise((resolve, _reject) => {
			pyprocess.on("close", resolve);
		});

		// if (exitCode) {
		// 	throw new Error(`subprocess error exit ${exitCode}, ${error}`);
		// }

		return data;
	}

	callPython(args: Array<string>): ChildProcessWithoutNullStreams {
		return spawn(pywrapperPath, args);
	}

	async check(): Promise<AlphaDBCheck> {
		const pyprocess = this.callPython(["check", this.adbInstanceId]);
		return JSON.parse(await this.handleChildProcess(pyprocess));
	}

	async connect({ host, user, password, database, port = 3306 }: AlphaDBConnectProps): Promise<boolean> {
		const pyprocess = this.callPython(["connect", this.adbInstanceId, host, user, password, database, port.toString()]);
		return (await this.handleChildProcess(pyprocess)) == "True";
	}

	async init(): Promise<true | "already-initialized"> {
		const pyprocess = this.callPython(["init", this.adbInstanceId]);
		const process = await this.handleChildProcess(pyprocess);
		if (process === "True") return true;
		else return "already-initialized";
	}

	async status(): Promise<AlphaDBStatus> {
		const pyprocess = this.callPython(["status", this.adbInstanceId]);
		return JSON.parse(await this.handleChildProcess(pyprocess));
	}

	async updateQueries(version_source: string, { updateToVersion = undefined, noData = false }: AlphaDBUpdateProps): Promise<AlphaDBUpdateQueries> {
		const pyprocess = this.callPython([
			"update_queries",
			this.adbInstanceId,
			JSON.stringify(version_source),
			`${updateToVersion}`,
			noData.toString()
		]);

		return JSON.parse(await this.handleChildProcess(pyprocess));
	}
}
