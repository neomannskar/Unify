# UnifyAll
UnifyAll is a simple build script CLI-tool meant to simplify the compilation of large codebases

## Example 'build.u' for C++

```md
# Build script

COMP: "g++"

ARGS:
  "-o",
  "main",
  "main.cpp",
```
### Build

```u
UnifyAll 'build.u'
```
### Create a new 'build.u' file

```u
UnifyAll --new <compiler> <arg#1> <arg#2> ...
```
