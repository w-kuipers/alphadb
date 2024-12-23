
import os from "os";
import fs from "fs";
import path from "path";
import { execSync } from "child_process";
import fetch from "node-fetch";

const BASE_URL = "https://github.com/w-kuipers/alphadb/releases/download/v1.0.0-beta.20";

function getBinaryURL() {
	const platform = os.platform();
	const arch = os.arch();

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
	const binaryPath = "./index.node";

	if (!response.ok) {
		throw new Error(`Failed to download binary: ${response.statusText}`);
	}

	const tempFile = path.resolve(binaryPath);
	const fileStream = fs.createWriteStream(tempFile);

	await new Promise((resolve, reject) => {
		response.body?.pipe(fileStream);
		response.body?.on('error', reject);
		fileStream.on('finish', resolve);
	});

	fs.unlinkSync(tempFile);
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
			console.warn('Unsupported platform/architecture.');
			if (hasRustInstalled()) {
				// buildFromSource();
			} else {
				console.error('Rust is not installed. Install Rust to build the binary from source.');
				process.exit(1);
			}
		}
	} catch (error) {
		console.error(`Postinstall script failed: ${error.message}`);
		process.exit(1);
	}
}


// function buildFromSource() {
// 	console.log('Building from source...');
// 	try {
// 		execSync('cargo build --release', { stdio: 'inherit' });
// 		console.log('Build completed. Copying binary...');
// 		const sourcePath = path.resolve('target', 'release', BINARY_NAME);
// 		const destPath = path.resolve(__dirname, 'bin', BINARY_NAME);
// 		fs.mkdirSync(path.dirname(destPath), { recursive: true });
// 		fs.copyFileSync(sourcePath, destPath);
// 		if (os.platform() !== 'win32') {
// 			fs.chmodSync(destPath, '755');
// 		}
// 		console.log(`Binary built and placed at: ${destPath}`);
// 	} catch (error) {
// 		console.error('Failed to build from source:', error.message);
// 		process.exit(1);
// 	}
// }

main();
