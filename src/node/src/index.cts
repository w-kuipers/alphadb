import * as addon from './load.cjs';

interface AlphaDB {
	conn: any;
	internaldbname: any;
	connect(host: string, user: string, password: string, database: string, port: number): void;
	init(): void;
}

// Use this declaration to assign types to the addon's exports,
// which otherwise by default are `any`.
declare module "./load.cjs" {
	const conn: any;
	const internaldbname: any;
	function connect(conn: any, internaldbname: any, host: string, user: string, password: string, database: string, port: number): void;
	function init(conn: any, internaldbname: any): void;
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
}

export {
	AlphaDB
}
