import js from '@eslint/js';
import tsPlugin from '@typescript-eslint/eslint-plugin';
import tsParser from '@typescript-eslint/parser';
import vuePlugin from 'eslint-plugin-vue';
import simpleImportSort from 'eslint-plugin-simple-import-sort';
import vueParser from 'vue-eslint-parser';
import globals from 'globals';

export default [
  js.configs.recommended,

  // TypeScript files
  {
    files: ['**/*.ts', '**/*.tsx'],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
      },
      globals: {
        ...globals.browser,
      },
    },
    plugins: {
      '@typescript-eslint': tsPlugin,
      'simple-import-sort': simpleImportSort,
    },
    rules: {
      ...tsPlugin.configs['flat/recommended'].rules,
      'simple-import-sort/imports': 'error',
      'simple-import-sort/exports': 'error',
    },
  },

  // Vue files
  ...vuePlugin.configs['flat/recommended'],
  {
    files: ['**/*.vue'],
    languageOptions: {
      parser: vueParser,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
        parser: tsParser,
      },
      globals: {
        ...globals.browser,
      },
    },
    plugins: {
      '@typescript-eslint': tsPlugin,
      'simple-import-sort': simpleImportSort,
    },
    rules: {
      ...tsPlugin.configs['flat/recommended'].rules,
      'simple-import-sort/imports': 'error',
      'simple-import-sort/exports': 'error',
      'vue/multi-word-component-names': ['error', {
        ignores: ['App']
      }],
    },
  },

  // Global ignores
  {
    ignores: [
      'dist/**',
      'node_modules/**',
      'src-tauri/**',
      '*.config.js',
      '*.config.ts',
      '*.cjs',
    ],
  },
];
