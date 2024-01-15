module.exports = {
	extends: [
		"eslint:recommended",
	],
	rules: {
		"@typescript-eslint/explicit-function-return-type": ["error", { allowExpressions: true }]
	}
};

