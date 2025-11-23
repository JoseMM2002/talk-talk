import pluginVitest from '@vitest/eslint-plugin';
import skipFormatting from '@vue/eslint-config-prettier/skip-formatting';
import { defineConfigWithVueTs, vueTsConfigs } from '@vue/eslint-config-typescript';
import { globalIgnores } from 'eslint/config';
import simpleImportSort from 'eslint-plugin-simple-import-sort';
import pluginVue from 'eslint-plugin-vue';

// To allow more languages other than `ts` in `.vue` files, uncomment the following lines:
// import { configureVueProject } from '@vue/eslint-config-typescript'
// configureVueProject({ scriptLangs: ['ts', 'tsx'] })
// More info at https://github.com/vuejs/eslint-config-typescript/#advanced-setup

export default defineConfigWithVueTs(
  {
    name: 'app/files-to-lint',
    files: ['**/*.{ts,mts,tsx,vue}'],
  },

  globalIgnores(['**/dist/**', '**/dist-ssr/**', '**/coverage/**', '**/src-tauri/**']),

  pluginVue.configs['flat/essential'],
  vueTsConfigs.recommended,

  {
    ...pluginVitest.configs.recommended,
    files: ['src/**/__tests__/*'],
  },
  skipFormatting,
  {
    plugins: {
      'simple-import-sort': simpleImportSort,
    },
    rules: {
      'vue/multi-word-component-names': 'off',
      'vue/singleline-html-element-content-newline': 'off',
      'vue/no-unused-vars': 'error',
      'simple-import-sort/imports': [
        'error',
        {
          groups: [
            // 1) Side effects
            ['^\\u0000'],
            // 2) Node builtins
            [
              '^node:',
              '^(assert|buffer|child_process|crypto|fs|http|https|path|url|util|zlib)(/.*)?$',
            ],
            // 3) Packages (vue first)
            ['^vue$', '^@?\\w'],
            // 4) Aliases (e.g., @/)
            ['^@/'],
            // 5) Parent imports
            ['^\\.\\.(?!/?$)', '^\\.\\./?$'],
            // 6) Sibling and index
            ['^\\./(?=.*/)(?!/?$)', '^\\.(?!/?$)', '^\\./?$'],
            // 7) Styles
            ['^.+\\.s?css$'],
            // 8) Vue SFC virtual subpaths
            ['\\?vue'],
          ],
        },
      ],
      'simple-import-sort/exports': 'error',
    },
  }
);
