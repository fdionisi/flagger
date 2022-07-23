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

    return config;
  },
};
