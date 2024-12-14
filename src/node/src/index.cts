// The Rust addon.
import * as addon from './load.cjs';

interface AlphaDB {
	conn: any;
	connect(host: string, user: string, password: string, database: string, port: number): void;
	init(): void;
}

// Use this declaration to assign types to the addon's exports,
// which otherwise by default are `any`.
declare module "./load.cjs" {
	const conn: any;
	function connect(conn: any, host: string, user: string, password: string, database: string, port: number): void;
}

// export const AlphaDB = addon.AlphaDB;

class AlphaDB {
	private constructor() {
		this.conn = addon.conn;
	}

	public connect(host: string, user: string, password: string, database: string, port: number) {
		addon.connect(this.conn, host, user, password, database, port);
	}

	// init(): void;
}

export {
	AlphaDB
}
