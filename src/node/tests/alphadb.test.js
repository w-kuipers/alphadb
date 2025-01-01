import { describe, it, expect, beforeAll } from "vitest";
import AlphaDB from ".";
import fs from "fs";

const db = new AlphaDB();
db.connect({
	host: "localhost",
	user: "root",
	password: "test",
	database: "test",
});

describe("AlphaDB Tests", () => {
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
			name: "test",
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
			name: "test",
			template: "test",
		});
	});

	it("should vacate the database", () => {
		db.vacate();

		const status = db.status();
		expect(status).toEqual({
			init: false,
			version: null,
			name: "test",
			template: null,
		});
	});
});

