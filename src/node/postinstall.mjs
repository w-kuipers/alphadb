import { fileURLToPath } from 'url';
import os from "os";
import fs from "fs";
import path from "path";
import { execSync } from "child_process";
import fetch from "node-fetch";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const platform = os.platform();
const arch = os.arch();

function getBinaryURL() {
	const BASE_URL = "https://github.com/w-kuipers/alphadb/releases/download/v1.0.0-beta.20";

	let fileName;

	if (platform === 'linux' && arch === 'x64') {
		fileName = "linux-x64.node";
	} else if (platform === 'darwin' && arch === 'arm64') {
		fileName = "darwin-x64.node";
	} else if (platform === 'darwin' && arch === 'x64') {
		fileName = "darwin-arm64.node";
	} else {
		return null;
	}

	return `${BASE_URL}/${fileName}`;
}

async function getBinary(url) {
	console.log(`Downloading AlphaDB binary from: ${url}`);
	const response = await fetch(url);

	const binaryPath = path.resolve(__dirname, "./index.node");

	if (!response.ok) {
		throw new Error(`Failed to download binary: ${response.statusText}`);
	}

	const fileStream = fs.createWriteStream(binaryPath);

	try {
		await new Promise((resolve, reject) => {
			response.body?.pipe(fileStream);
			response.body?.on('error', reject);
			fileStream.on('finish', resolve);
		});
		console.log("AlphaDB binary successfully downloaded");
	} catch (error) {
		console.error('Error while download AlphaDB binary:', error);
	}
}

function hasRustInstalled() {
	try {
		execSync('rustc --version', { stdio: 'ignore' });
		return true;
	} catch {
		return false;
	}
}

async function main() {
	try {
		const binaryURL = getBinaryURL();

		if (binaryURL) {
			await getBinary(binaryURL);
		} else {
			console.warn(`There are no prebuilt binaries for platform ${platform}-${arch}. Attempting to build from source.`);
			if (hasRustInstalled()) {
				buildFromSource();
			} else {
				console.error('Rust is not installed. Install Rust to build AlphaDB from source.');
				process.exit(1);
			}
		}
	} catch (error) {
		console.error(`AlphaDB postinstall script failed: ${error.message}`);
		process.exit(1);
	}
}


function buildFromSource() {
	try {
		execSync("yarn build", { stdio: "inherit" });
	} catch (error) {
		console.error('Failed to build from source:', error.message);
		process.exit(1);
	}
}

main();
