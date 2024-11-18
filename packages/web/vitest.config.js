// vitest.config.ts
import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    environment: 'jsdom', // Use jsdom for DOM API support
    globals: true, // Enable global test APIs like `describe`, `it`
    setupFiles: './src/testing/setupTests.ts', // Setup file path
    css: true, // Enable CSS if you're importing CSS files in components
    coverage: {
      reporter: ['text', 'lcov'], // Coverage reporters
      exclude: ['node_modules/', 'src/testing/setupTests.ts'],
    },
  },
});
