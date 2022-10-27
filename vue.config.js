const { defineConfig } = require("@vue/cli-service");
module.exports = defineConfig({
  devServer: {
    port: 8099,
  },
  transpileDependencies: true,
});
