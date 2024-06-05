# Description

Generates a high level summary of the demand for machines in a gitlab project's CI.

# Dependencies
Install rust - https://www.rust-lang.org/tools/install

# Build

```
$ cargo build
```

# Run

```
$ cargo run -- --token "your-gitlab-token" --project-path vtk/vtk
```

Example:

```
$ cargo run -- --token "your-gitlab-token" --project-path vtk/vtk

platform: Linux | demand: 99.091 | supply: 0.909 | pending: 218 | running: 2 | total: 220
platform: MacOSArm64 | demand: 98.387 | supply: 1.613 | pending: 61 | running: 1 | total: 62
platform: MacOSx86_64 | demand: 95.833 | supply: 4.167 | pending: 23 | running: 1 | total: 24
platform: Windows | demand: 98.315 | supply: 1.685 | pending: 175 | running: 3 | total: 178
```

```
$ cargo run -- --token "your-gitlab-token" --project-path paraview/paraview

platform: Linux | demand: 87.500 | supply: 12.500 | pending: 7 | running: 1 | total: 8
platform: MacOSArm64 | demand: 0.000 | supply: 100.000 | pending: 0 | running: 1 | total: 1
platform: MacOSx86_64 | demand: 100.000 | supply: 0.000 | pending: 1 | running: 0 | total: 1
platform: Windows | demand: 50.000 | supply: 50.000 | pending: 1 | running: 1 | total: 2
```
