module.exports = {
  branches: ["main"],
  plugins: [
    [
      "@semantic-release/commit-analyzer",
      {
        preset: "conventionalcommits",
        releaseRules: [
          { type: "build", scope: "deps", release: "patch" },
          { type: "build", scope: "deps-dev", release: "patch" },
          { type: "refactor", release: "patch" },
          { type: "style", release: "patch" },
          { type: "ci", release: "patch" },
          { type: "chore", release: "patch" },
          { type: "docs", release: "patch" },
          { breaking: true, release: "major" },
        ],
      },
    ],
    [
      "@semantic-release/release-notes-generator",
      {
        preset: "conventionalcommits",
      },
    ],
    [
      "@semantic-release/changelog",
      {
        changelogFile: "CHANGELOG.md",
        changelogTitle: "# AXONE contracts",
      },
    ],
    [
      "semantic-release-replace-plugin",
      {
        replacements: [
          {
            files: ["Cargo.toml"],
            from: /^version\s+=\s+"\d+\.\d+\.\d+"$/gm,
            to: 'version = "${nextRelease.version}"',
            countMatches: true,
            results: [
              {
                file: "Cargo.toml",
                hasChanged: true,
                numMatches: 1,
                numReplacements: 1,
              },
            ],
          },
          {
            files: ["Cargo.toml"],
            from: /((axone-[\w-]+)\s*=\s*\{\s*path\s*=\s*"\.\/[^"]*",\s+version\s+=\s+)"\d+\.\d+\.\d+"/g,
            to: (
              _match,
              prefix,
              _dependencyName,
              _path,
              _extra,
              _version,
              context
            ) => `${prefix}"${context.nextRelease.version}"`,
            countMatches: true,
            results: [
              {
                file: "Cargo.toml",
                hasChanged: true,
                numMatches: 1,
                numReplacements: 1,
              },
            ],
          },
        ],
      },
    ],
    [
      "@semantic-release/exec",
      {
        prepareCmd:
          "cargo make schema && cargo make docs && cargo make release-wasm",
      },
    ],
    [
      "@semantic-release/github",
      {
        successComment: false,
        assets: [
          { path: "./artifacts/axone_dummy.wasm" },
          { path: "./artifacts/checksums.txt" },
        ],
      },
    ],
    [
      "@semantic-release/git",
      {
        assets: [
          "CHANGELOG.md",
          "docs/**",
          "Cargo.toml",
          "Cargo.lock",
        ],
        message: "chore(release): perform release ${nextRelease.version}",
      },
    ],
  ],
};
