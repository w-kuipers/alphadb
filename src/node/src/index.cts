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

interface ConnectProps {
	host: string;
	user: string;
	password: string;
	database: string;
	port?: number;
}

type ToleratedVerificationIssueLevel = "LOW" | "HIGH" | "CRITICAL" | "ALL";

interface AlphaDB {
	conn: any;
	internaldbname: any;
	internalisconnected: any;
	db_name: string | undefined;
	is_connected: boolean;
	connect(props: ConnectProps): void;
	init(): void;
	status(): Status;
	updateQueries(version_source: VersionSource, target_version?: string, no_data?: boolean): Array<Query>;
	update(version_source: VersionSource, target_version?: string, no_data?: boolean, verify?: boolean, toleratedVerificationIssueLevel?: ToleratedVerificationIssueLevel): void;
	vacate(): void;
}


// Use this declaration to assign types to the addon's exports,
// which otherwise by default are `any`.
declare module "./load.cjs" {
	const conn: any;
	const internaldbname: any;
	const internalisconnected: any;
	function get_is_connected(internalisconnected: any): boolean;
	function get_db_name(internaldbname: any): string | undefined;
	function connect(conn: any, internaldbname: any, internalisconnected: any, host: string, user: string, password: string, database: string, port: number): void;
	function init(conn: any, internaldbname: any): void;
	function status(conn: any, internaldbname: any): Status;
	function update_queries(conn: any, internaldbname: any, version_source: string, target_version: string, no_data: boolean): Array<Query>;
	function update(conn: any, internaldbname: any, version_source: string, target_version: string, no_data: boolean, verify: boolean, tolerated_verification_issue_level: string): Array<Query>;
	function vacate(conn: any): void;
}

class AlphaDB {
	private constructor() {
		this.conn = addon.conn;
		this.internaldbname = addon.internaldbname;
		this.internalisconnected = addon.internalisconnected;
		this.db_name = undefined;
		this.is_connected = false;
	}

	public connect(props: ConnectProps) {
		if (typeof props.port === "undefined") props.port = 3306;
		addon.connect(this.conn, this.internaldbname, this.internalisconnected, props.host, props.user, props.password, props.database, props.port);

		this.db_name = addon.get_db_name(this.internaldbname);
		this.is_connected = addon.get_is_connected(this.internalisconnected);
	}

	public init() {
		addon.init(this.conn, this.internaldbname);
	}

	public status() {
		return addon.status(this.conn, this.internaldbname);
	}

	public updateQueries(version_source: VersionSource, target_version?: string, no_data?: boolean) {
		if (typeof target_version === "undefined") target_version = "NOVERSION";
		if (typeof no_data === "undefined") no_data = false;
		return addon.update_queries(this.conn, this.internaldbname, JSON.stringify(version_source), target_version, no_data);
	}

	public async update(version_source: VersionSource, target_version?: string, no_data?: boolean, verify?: boolean, toleratedVerificationIssueLevel?: ToleratedVerificationIssueLevel) {
		if (typeof target_version === "undefined") target_version = "NOVERSION";
		if (typeof no_data === "undefined") no_data = false;
		if (typeof verify === "undefined") verify = true;
		if (typeof toleratedVerificationIssueLevel === "undefined") toleratedVerificationIssueLevel = "LOW";

		return addon.update(this.conn, this.internaldbname, JSON.stringify(version_source), target_version, no_data, verify, toleratedVerificationIssueLevel);
	}

	public vacate() {
		addon.vacate(this.conn);
	}
}

export {
	AlphaDB
}
