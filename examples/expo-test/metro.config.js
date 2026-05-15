const path = require("path");
const { getDefaultConfig } = require("expo/metro-config");

const projectRoot = __dirname;
const workspaceRoot = path.resolve(projectRoot, "../..");
const config = getDefaultConfig(projectRoot);

// Allow Metro to resolve files from the monorepo root (linked packages)
config.watchFolders = [workspaceRoot];

// Force all React imports (including from linked workspace packages)
// to resolve from this Expo app, preventing invalid hook call errors.
config.resolver.extraNodeModules = {
  ...(config.resolver.extraNodeModules || {}),
  react: path.resolve(projectRoot, "node_modules/react"),
  "react-native": path.resolve(projectRoot, "node_modules/react-native"),
  "react-native-svg": path.resolve(
    projectRoot,
    "node_modules/react-native-svg",
  ),
};

// Prevent Metro from resolving react/react-native from within linked packages
config.resolver.resolveRequest = (context, moduleName, platform) => {
  if (
    moduleName === "react" ||
    moduleName === "react-native" ||
    moduleName === "react-native-svg"
  ) {
    return {
      filePath: require.resolve(moduleName, {
        paths: [projectRoot],
      }),
      type: "sourceFile",
    };
  }
  return context.resolveRequest(context, moduleName, platform);
};

// Ensure Metro doesn't follow symlinks into packages with their own node_modules
config.resolver.nodeModulesPaths = [
  path.resolve(projectRoot, "node_modules"),
  path.resolve(workspaceRoot, "node_modules"),
];

module.exports = config;
