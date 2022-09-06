// eslint-disable-next-line no-undef
module.exports = {
  experimental: {
    esmExternals: false,
  },
  webpack(config, options) {
    config.experiments = { ...config.experiments, topLevelAwait: true };
    config.module.rules.push({
      test: /flagger-client/,
      use: [options.defaultLoaders.babel],
    });

    config.module.rules = [
      ...config.module.rules,
      // ensure our libs barrel files don't constitute imports
      {
        test: /www\/.*index.ts/i,
        sideEffects: false,
      },
    ];

    return config;
  },
};
