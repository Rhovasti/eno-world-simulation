import { defineConfig } from 'vite';

export default defineConfig({
  build: {
    lib: {
      entry: 'src/index.ts',
      name: 'WorldSimulationClient',
      fileName: (format) => `world-simulation-client.${format}.js`,
    },
    rollupOptions: {
      external: ['@clockworklabs/spacetimedb-sdk'],
      output: {
        globals: {
          '@clockworklabs/spacetimedb-sdk': 'SpacetimeDBSDK',
        },
      },
    },
    sourcemap: true,
  },
  server: {
    port: 3002,
    host: true,
  },
});