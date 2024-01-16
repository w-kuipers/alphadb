import { spawn, ChildProcessWithoutNullStreams } from "child_process";
import fs from "fs";
import path from "path";


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
}

interface AlphaDBUpdateProps {
	updateToVersion?: string;
	noData?: boolean;
}

type AlphaDBUpdateQueries = Array<string | Array<string>>;
type VersionSource = object;

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
	pywrapperPath: string = "";

	constructor() {

		// Dynamically get pywrapper path
		if (process.platform === "linux") {
			this.pywrapperPath = path.join(path.dirname(__dirname), "pywrapper_linux_x86_64/pywrapper_linux_x86_64");
		}
		else if (process.platform === "win32") {
			this.pywrapperPath = path.join(path.dirname(__dirname), "pywrapper_win32_x86_64/pywrapper_win32_x86_64.exe");
		}
		else {
			throw Error("Unsupported platform");
		}

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

		if (exitCode) {
			throw new Error(`subprocess error exit ${exitCode}, ${error}`);
		}

		return data;
	}

	callPython(args: Array<string>): ChildProcessWithoutNullStreams {
		return spawn(this.pywrapperPath, args);
	}

	removeNewlines(val: string): string {
		return val.replace(/\r?\n|\r/g, "");
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
		if (this.removeNewlines(process) === "True") return true;
		else return "already-initialized";
	}

	async status(): Promise<AlphaDBStatus> {
		const pyprocess = this.callPython(["status", this.adbInstanceId]);
		return JSON.parse(await this.handleChildProcess(pyprocess));
	}

	async updateQueries(
		version_source: VersionSource,
		{ updateToVersion = undefined, noData = false }: AlphaDBUpdateProps
	): Promise<"up-to-date" | AlphaDBUpdateQueries> {
		const pyprocess = this.callPython([
			"update_queries",
			this.adbInstanceId,
			`${JSON.stringify(version_source)}`,
			`${updateToVersion}`,
			noData.toString()
		]);

		const processed = await this.handleChildProcess(pyprocess);

		if (this.removeNewlines(processed) === "up-to-date") return "up-to-date";
		else return JSON.parse(processed);
	}

	async update(version_source: VersionSource, { updateToVersion = undefined, noData = false }: AlphaDBUpdateProps): Promise<true | "up-to-date"> {
		const pyprocess = this.callPython(["update", this.adbInstanceId, `${JSON.stringify(version_source)}`, `${updateToVersion}`, noData.toString()]);
		const processed = await this.handleChildProcess(pyprocess);

		if (this.removeNewlines(processed) === "True") return true;
		else return "up-to-date";
	}

	async vacate(confirm: boolean): Promise<boolean> {
		const pyprocess = this.callPython(["vacate", this.adbInstanceId, `${confirm}`]);

		return this.removeNewlines(await this.handleChildProcess(pyprocess)) == "True";
	}
}
