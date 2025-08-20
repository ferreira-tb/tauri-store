// swift-tools-version:6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "tauri-plugin-__SNAKE_PLUGIN_TITLE__",
    platforms: [
        .macOS(.v10_13),
        .iOS(.v13),
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "tauri-plugin-__SNAKE_PLUGIN_TITLE__",
            type: .static,
            targets: ["tauri-plugin-__SNAKE_PLUGIN_TITLE__"]),
    ],
    dependencies: [
        .package(name: "Tauri", path: "../.tauri/tauri-api")
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .target(
            name: "tauri-plugin-__SNAKE_PLUGIN_TITLE__",
            dependencies: [
                .byName(name: "Tauri")
            ],
            path: "Sources")
    ]
)
