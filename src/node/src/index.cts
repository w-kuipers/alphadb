import * as addon from './load.cjs';

interface Status {
	init: boolean;
	version: string | null;
	name: string;
	template: string | null;
}

type Query = [string, Array<string>];

interface Version {
	_id: string;
	createtable?: string;
	altertable?: string;
}

interface VersionSource {
	name: string;
	version: Array<Version>;
}

interface AlphaDB {
	conn: any;
	internaldbname: any;
	connect(host: string, user: string, password: string, database: string, port: number): void;
	init(): void;
	status(): Status;
	updateQueries(version_source: VersionSource, update_to_version?: string): Array<Query>;
}


// Use this declaration to assign types to the addon's exports,
// which otherwise by default are `any`.
declare module "./load.cjs" {
	const conn: any;
	const internaldbname: any;
	function connect(conn: any, internaldbname: any, host: string, user: string, password: string, database: string, port: number): void;
	function init(conn: any, internaldbname: any): void;
	function status(conn: any, internaldbname: any): Status;
	function update_queries(conn: any, internaldbname: any, version_source: string, update_to_version?: string): Array<Query>;
}

class AlphaDB {
	private constructor() {
		this.conn = addon.conn;
		this.internaldbname = addon.internaldbname;
	}

	public connect(host: string, user: string, password: string, database: string, port: number) {
		addon.connect(this.conn, this.internaldbname, host, user, password, database, port);
	}

	public init() {
		addon.init(this.conn, this.internaldbname);
	}

	public status() {
		return addon.status(this.conn, this.internaldbname);
	}

	public updateQueries(version_source: VersionSource, update_to_version?: string) {
		if (typeof update_to_version === "undefined") update_to_version = "NOVERSION";
		return addon.update_queries(this.conn, this.internaldbname, JSON.stringify(version_source), update_to_version);
	}
}

export {
	AlphaDB
}
