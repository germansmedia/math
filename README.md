# math

All the math you ever wanted.

## Design Philosophy

- Bundle together coordinates for various purposes as much as possible.
- Use the same notation for everything.
- Overload operators, so it looks readable.
- Direct access to coordinate members.
- Lowercase extension to basic types, like f32xy, f32m4x4, etc.

## Elements

### Vectors

`u16xy`, `isizexyz`, `f32xyzw`, etc.

Includes basic vector math, dot products, cross products, etc.

### Matrices

`f32m2x2`, `f64m4x4`, etc.

Includes inverse, determinant, etc. as well as traditional 3D geometry stuff, like translate, rotate, frustum projection, etc.

### Multivectors

`f32mv2`, `f64mv4`, etc.

Geometric algebra components.

### Rectangles

`u32r`, `f64b`, etc.

Includes containment test and boolean operators.

### Complex Numbers

`f32c`, `f64c`, etc.

Includes conversions and most basic math.

### Quaternions

`f32q`, `f64q`, etc.

Includes conversions to/from Euler or axis/angle pairs.

### Euler Angles

`f32ypb`, `f64ypb`, etc.

### Color Specifications

`f32rgb`, `f64rgba`, etc.
