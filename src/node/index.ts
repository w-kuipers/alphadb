import { spawn, ChildProcessWithoutNullStreams } from "child_process";
import fs from "fs";

interface ConnectProps {
	host: string;
	user: string;
	password: string;
	database: string;
	port?: number;
}

function random_string(length: number = 10): string {
	let result = "";
	const characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

	const charactersLength = characters.length;
	for (let i = 0; i < length; i++) {
		result += characters.charAt(Math.floor(Math.random() * charactersLength));
	}

	return result;
}

class AlphaDB {
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

		if (exitCode) {
			throw new Error(`subprocess error exit ${exitCode}, ${error}`);
		}

		return data;
	}

	async connect({ host, user, password, database, port = 3306 }: ConnectProps) {
		const pyprocess = spawn("python", [
			"wrapper.py",
			"connect",
			this.adbInstanceId,
			host,
			user,
			password,
			database,
			port.toString()
		]);

		return this.handleChildProcess(pyprocess);
	}

	async init() {
		const pyprocess = spawn("python", ["wrapper.py", "init", this.adbInstanceId]);

		return this.handleChildProcess(pyprocess);
	}
}

const db = new AlphaDB();

const test = async () => {
	await db.connect({
		host: "localhost",
		user: "root",
		password: "test",
		database: "test"
	});

	db.init();
};

test();
