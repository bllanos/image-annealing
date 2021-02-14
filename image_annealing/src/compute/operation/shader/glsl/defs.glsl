#ifndef DEFS
#define DEFS

// Matches `WORKGROUP_SIZE` in src/compute/operation/shader/workgroup/mod.rs
#define WORKGROUP_SIZE_X 32
#define WORKGROUP_SIZE_Y 32
#define WORKGROUP_SIZE_Z 1

// Matches `<PermutationTexture as TextureDatatype>::format()` in src/compute/resource/texture/mod.rs
#define PERMUTATION_FORMAT rg16i

#endif // DEFS