import { describe, it, expect } from "vitest";
import AlphaDB from ".";
import fs from "fs";
import path from "path";

const engine = process.env.ALPHADB_ENGINE ?? "mysql";

const engines = {
	mysql: {
		connect: {
			host: "localhost",
			user: "root",
			password: "test",
			database: "adb_test1",
			port: 333,
		},
		structure: "test-mysql-db-structure.json",
	},
	postgres: {
		connect: {
			host: "localhost",
			user: "postgres",
			password: "test",
			database: "adb_test1",
			port: 544,
		},
		structure: "test-postgres-db-structure.json",
	},
};

if (!engines[engine]) {
	throw new Error(`Unsupported ALPHADB_ENGINE '${engine}'. Expected one of: ${Object.keys(engines).join(", ")}`);
}

const config = engines[engine];
const db = new AlphaDB();

function loadStructure() {
	const structurePath = path.resolve("../../assets", config.structure);
	const structure = JSON.parse(fs.readFileSync(structurePath, "utf-8"));

	if (engine === "postgres") {
		delete structure.version[0].createtable.table1.index;
	}

	return structure;
}

describe(`AlphaDB ${engine} Tests`, () => {

	it("should connect to the database", () => {
		expect(db.is_connected).toEqual(false);
		expect(db.db_name).toBeUndefined();
		db.connect(config.connect);
		expect(db.db_name).toEqual(config.connect.database);
		expect(db.is_connected).toEqual(true);
		db.vacate();
	});

	it("should initialize the database and throw if already initialized", () => {
		db.init();
		expect(() => db.init()).toThrowError(
			"The database is already initialized"
		);
	});

	it("should be initialized", () => {
		const status = db.status();
		expect(status).toEqual({
			init: true,
			version: "0.0.0",
			name: config.connect.database,
			template: null,
		});
	});

	it("should update the database structure to version 0.2.6", async () => {
		await db.update(loadStructure());

		const status = db.status();
		expect(status).toEqual({
			init: true,
			version: "0.2.6",
			name: config.connect.database,
			template: "test",
		});
	});

	it("should vacate the database", () => {
		db.vacate();

		const status = db.status();
		expect(status).toEqual({
			init: false,
			version: null,
			name: config.connect.database,
			template: null,
		});
	});
});
