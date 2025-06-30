import { describe, it, expect } from "vitest";
import AlphaDB from ".";
import fs from "fs";

const db = new AlphaDB();

describe("AlphaDB Tests", () => {

	it("should connect to the database", () => {
		expect(db.is_connected).toEqual(false);
		expect(db.db_name).toBeUndefined();
		db.connect({
			host: "localhost",
			user: "root",
			password: "test",
			database: "adb_test1",
			port: 333
		});
		expect(db.db_name).toEqual("adb_test1");
		expect(db.is_connected).toEqual(true);
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
			name: "adb_test1",
			template: null,
		});
	});

	it("should update the database structure to version 0.2.6", () => {
		const structure = fs.readFileSync("../../assets/test-db-structure.json", "utf-8");
		db.update(JSON.parse(structure));

		const status = db.status();
		expect(status).toEqual({
			init: true,
			version: "0.2.6",
			name: "adb_test1",
			template: "test",
		});
	});

	it("should vacate the database", () => {
		db.vacate();

		const status = db.status();
		expect(status).toEqual({
			init: false,
			version: null,
			name: "adb_test1",
			template: null,
		});
	});
});

