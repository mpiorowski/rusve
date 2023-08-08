module.exports = {
    root: true,
    extends: [
        "eslint:recommended",
        "plugin:@typescript-eslint/recommended",
        "plugin:svelte/recommended",
        // additional typescript rules
        "plugin:@typescript-eslint/eslint-recommended",
        "plugin:@typescript-eslint/recommended-requiring-type-checking",
        "plugin:@typescript-eslint/strict",
    ],
    parser: "@typescript-eslint/parser",
    plugins: ["@typescript-eslint"],
    parserOptions: {
        project: "./tsconfig.json",
        sourceType: "module",
        ecmaVersion: 2020,
        extraFileExtensions: [".svelte"],
    },
    env: {
        browser: true,
        es2017: true,
        node: true,
    },
    overrides: [
        {
            files: ["*.svelte"],
            parser: "svelte-eslint-parser",
            parserOptions: {
                parser: "@typescript-eslint/parser",
            },
        },
    ],
    rules: {
        "@typescript-eslint/no-shadow": ["error"],
        "@typescript-eslint/consistent-type-definitions": ["off"],
        "@typescript-eslint/no-throw-literal": ["off"],
        "@typescript-eslint/explicit-function-return-type": [
            "error",
            { allowExpressions: true },
        ],
        "@typescript-eslint/no-unused-vars": [
            "error",
            { argsIgnorePattern: "^_" },
        ],
        "@typescript-eslint/no-misused-promises": [
            "error",
            { checksVoidReturn: false },
        ],
    },
};
