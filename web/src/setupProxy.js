const { createProxyMiddleware } = require("http-proxy-middleware");
module.exports = function (app) {
  app.use(
    "/api",
    createProxyMiddleware({
      target: "http://localhost:4635",
      pathRewrite: (path, req) => path.replace("/api/", "/"),
      changeOrigin: true,
    })
  );
};
