# math

All the math you ever wanted.

by Desmond Germans, Ph.D
Germans Media Technology & Services

## Design Philosophy

- Bundle together coordinates for various purposes as much as possible.
- Use the same notation for everything.
- Overload operators, so it looks readable.
- Direct access to coordinate members.
- Lowercase extension to basic types, like f32xy, f32m4x4, etc.

## Elements

### Vectors

There are 2-, 3- and 4-dimensional vectors, described like `Vec2<T>`, `Vec3<T>` and `Vec4<T>`. Because vectors like this are immeasurably common, they are also defined by their base type, followed by `xy`, `xyz` or `xyzw`, like in `u16xy`, `isizexyz`, `f32xyzw`, etc.

Most algebraic operations are defined for each vector type, including the modulus, dot product and cross product (for `Vec3<T>` only).

### Matrices

There are 2x2-, 3x3- and 4x4-dimensional matrices, going with the corresponding vectors. They are described like `Mat2x2<T>`, `Mat3x3<T>` and `Mat4x4`. Because matrices like this are very common, they are also defined by their base type, followed by `m2x2`, `m3x3` or `m4x4`.

Most algebraic operations are defined for each matrix type, including the determinant, matrix product, inverse, transpose, and matrix-vector multiplications.

### Multivectors

Multivectors are a more intuitive and formal way to describe most operations. There are 2-, 3- and 4-dimensional multivectors, described like `MultiVec2<T>`, `MultiVec3<T>` and `MultiVec4<T>`. Because multivectors should be common, they are also defined by their base type, followed by `mv2`, `mv3` or `mv4`.

### Rectangles

Rectangles (and boxes) are 2-dimensional (and 3-dimensional) rectangular (or cuboid) volumes. They are described like `Rectangle<T>` and `Volume<T>` (TBD). Because rectangles are unbelievably common, they are also defined by their base type, followed by `r` or `v`, like in `i16r`, `isizer` and `f64v`.

Most boolean operations are defined on rectangles (and volumes).

### Complex Numbers

Complex numbers are described like `Complex<T>`. Because they are fairly common, they are also defined by their base type, followed by `c`, like `f32c` or `f64c`.

### Quaternions

Quaternions are described like `Quaternion<T>`. Because they should be much more common than you think, they are also defined by their base type, followed by `q`, like `f32q` and `f64q`.

### Euler Angles

Since some people still use Euler angles, they are described like `YPB<T>` (chosen over `Euler<T>` because of the confusion about the components). And yes, `f32ybp` and `f64ybp` also exist.

### Color Specifications

Math on colors can be done by using `RGB<T>` and `RGBA<T>`. Shortcuts exist as `f32rgb`, `f64rgba`, etc. The `Pixel` trait in the `image` crate is also defined for `RGB<T>` and `RGBA<T>`.
