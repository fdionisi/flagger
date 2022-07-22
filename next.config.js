// eslint-disable-next-line no-undef
module.exports = {
  experimental: {
    esmExternals: false,
  },
  webpack(config, options) {
    config.experiments = { ...config.experiments, topLevelAwait: true };
    config.module.rules.push({
      test: /lib\/flagger-client/,
      use: [options.defaultLoaders.babel],
    });

    return config;
  },
};
