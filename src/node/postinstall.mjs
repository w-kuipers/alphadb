import { fileURLToPath } from "url";
import os from "os";
import fs from "fs";
import path from "path";
import { execSync } from "child_process";
import fetch from "node-fetch";

const BASE_URL = "https://github.com/w-kuipers/alphadb/releases/download/version-number";
const SUPPORTED_BINARIES = {
	"linux-x64": "linux-x64-gnu.node",
	"darwin-arm64": "darwin-arm64.node",
	"darwin-x64": "darwin-x64.node",
	"win32-x64": "win32-x64-msvc.node",
};

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const platform = os.platform();
const arch = os.arch();
const platformArch = `${platform}-${arch}`;
const packageJson = JSON.parse(fs.readFileSync(path.resolve(__dirname, "./package.json"), "utf8"));
const isPostgresPackage = packageJson.name.includes("postgres");

function getBinaryURL() {
	const fileName = SUPPORTED_BINARIES[platformArch];

	if (!fileName) {
		return null;
	}

	return `${BASE_URL}/${fileName}`;
}

async function downloadBinary(url) {
	console.log(`Downloading AlphaDB binary from: ${url}`);
	const response = await fetch(url);
	const binaryPath = path.resolve(__dirname, "./index.node");

	if (!response.ok) {
		console.warn(`Failed to download binary: ${response.statusText}`);
		return false;
	}

	const fileStream = fs.createWriteStream(binaryPath);

	try {
		await new Promise((resolve, reject) => {
			response.body?.pipe(fileStream);
			response.body?.on("error", reject);
			fileStream.on("finish", resolve);
		});

		console.log("AlphaDB binary successfully downloaded");

		return true;
	} catch (error) {
		console.warn("Error while downloading AlphaDB binary:", error);
		return false;
	}
}

function hasRustInstalled() {
	try {
		execSync("rustc --version", { stdio: "ignore" });

		return true;
	} catch {
		return false;
	}
}

function buildFromSource() {
	if (!hasRustInstalled()) {
		console.error("Rust is not installed. Install Rust to build AlphaDB from source.");
		process.exit(1);
	}

	try {
		execSync(isPostgresPackage ? "npm run build:postgres:notsc" : "npm run build:mysql:notsc", {
			cwd: __dirname,
			stdio: "inherit",
		});

		// Run neon dist to generate the native binary
		const cargoLogPath = path.resolve(__dirname, "./cargo.log");
		execSync(`neon dist -n alphadb-node < ${cargoLogPath}`, {
			cwd: __dirname,
			stdio: "inherit",
		});

	} catch (error) {
		console.error("Failed to build from source:", error.message);
		process.exit(1);
	}
}

async function main() {
	try {
		const binaryURL = getBinaryURL();

		if (!binaryURL) {
			console.warn(`There are no prebuilt binaries for platform ${platformArch}. Attempting to build from source.`);
			buildFromSource();
			return;
		}

		const downloaded = await downloadBinary(binaryURL);
		if (!downloaded) {
			buildFromSource();
		}
	} catch (error) {
		console.error(`AlphaDB postinstall script failed: ${error.message}`);
		process.exit(1);
	}
}

main();
