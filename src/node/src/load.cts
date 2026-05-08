// This module loads the platform-specific build of the addon on
// the current system. The supported platforms are registered in
// the `platforms` object below, whose entries can be managed by
// by the Neon CLI:
//
//   https://www.npmjs.com/package/@neon-rs/cli


const packageJson = require('../package.json') as { alphadb?: { engine?: string } };
const binaryScope = packageJson.alphadb?.engine === 'postgres' ? '@alphadb-postgres' : '@alphadb-mysql';

module.exports = require('@neon-rs/load').proxy({
	platforms: {
		'win32-x64-msvc': () => require(`${binaryScope}/win32-x64-msvc`),
		'darwin-x64': () => require(`${binaryScope}/darwin-x64`),
		'darwin-arm64': () => require(`${binaryScope}/darwin-arm64`),
		'linux-x64-gnu': () => require(`${binaryScope}/linux-x64-gnu`),
		'linux-arm64-gnu': () => require(`${binaryScope}/linux-arm64-gnu`)
	},
	debug: () => require('../index.node')
});
