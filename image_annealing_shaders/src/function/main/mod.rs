use crate::constant;
use std::io::Write;

mod header;

pub use header::SHADER_ENTRY_POINT;

pub fn create_permutation<W: Write>(mut writer: W) -> std::io::Result<()> {
    header::global_invocation_id_header(&mut writer)?;
    writeln!(
        writer,
        "  textureStore(output_permutation, vec2<i32>(global_id.xy), vec4<u32>(0u,0u,0u,0u));
}}"
    )
}

pub fn forward_permute<W: Write>(mut writer: W) -> std::io::Result<()> {
    header::global_invocation_id_header(&mut writer)?;
    writeln!(
        writer,
        "  let coords : vec2<i32> = vec2<i32>(global_id.xy);
  let permutation_vector : vec2<i32> = load_permutation_vector(coords);
  let image_texel : vec4<u32> = textureLoad(input_image, coords + permutation_vector, 0);
  textureStore(output_image, coords, image_texel);
}}"
    )
}

pub fn swap<W: Write>(mut writer: W) -> std::io::Result<()> {
    header::swap_header(&mut writer)?;
    writeln!(
    writer,
    "  var count : f32 = 0.0;
  let coords1 : vec2<i32> = vec2<i32>(i32(global_id.x * {}u), i32(global_id.y));
  let displacement : vec2<i32> = vec2<i32>(1, 0);
  let coords2 : vec2<i32> = coords1 + displacement;
  let dimensions : vec2<i32> = textureDimensions(input_permutation);
  if(coords1.x < dimensions.x && coords1.y < dimensions.y) {{
    let input_permutation_vector1 : vec2<i32> = load_permutation_vector(coords1);
    var output_permutation_vector1 : vec2<i32> = input_permutation_vector1;

    if(coords2.x < dimensions.x && coords2.y < dimensions.y) {{
      let input_permutation_vector2 : vec2<i32> = load_permutation_vector(coords2);
      var output_permutation_vector2 : vec2<i32> = input_permutation_vector2;

      if(swap_cost(coords1, displacement, input_permutation_vector1, input_permutation_vector2) < parameters.acceptance_threshold) {{
        output_permutation_vector1 = input_permutation_vector2 + displacement;
        output_permutation_vector2 = input_permutation_vector1 - displacement;
        count = 1.0;
      }}

      store_permutation_vector(coords2, output_permutation_vector2);
    }}

    store_permutation_vector(coords1, output_permutation_vector1);
  }}

  partial_sum[local_id] = count;
  workgroupBarrier();

  reduce_partial_sum(local_id);

  if (local_id == 0u) {{
    let workgroup_index : u32 = workgroup_id.x + (workgroup_id.y * num_workgroups.x) + (workgroup_id.z * num_workgroups.x * num_workgroups.y);
    count_output.arr[parameters.count_output_offset + workgroup_index] = partial_sum[local_id];
  }}
}}", constant::swap::STRIDE
  )
}

pub fn count_swap<W: Write>(mut writer: W) -> std::io::Result<()> {
    header::count_swap_header(&mut writer)?;
    writeln!(
    writer,
    "  // Parallel reduction code based on:
  //   Optimizing Parallel Reduction in CUDA, by Mark Harris, NVIDIA Developer Technology
  //   Downloaded as a PDF. The PDF was created on August 28, 2018 17:17:47.
  //   Original download URL unknown. A CUDA implementation can be found at
  //     https://github.com/zhyma/parallel_reduction,
  //     along with a link to a slightly older version of the PDF.
  let total_invocations : u32 = num_workgroups.x * num_workgroups.y * num_workgroups.z * workgroup_invocations;
  let global_id : u32 = local_id + (workgroup_id.x + (workgroup_id.y * num_workgroups.x) + (workgroup_id.z * num_workgroups.x * num_workgroups.y)) * workgroup_invocations;

  var local_sum : vec4<f32> = vec4<f32>(0.0);
  for(var channel: u32 = 0u; channel < n_channel; channel = channel + 1u) {{
    if (parameters.do_segment[channel] != 0u) {{
      var i : u32 = parameters.segment_start[channel] + global_id;
      let end : u32 = parameters.segment_end[channel];
      loop {{
        if (i >= end) {{
            break;
        }}

        local_sum[channel] = local_sum[channel] + input.arr[i];

        i = i + total_invocations;
      }}
    }}
  }}

  partial_sum[local_id] = local_sum;
  workgroupBarrier();

  reduce_partial_sum(local_id);

  if (local_id == 0u) {{
    output.arr[workgroup_id.x] = partial_sum[local_id];
  }}
}}"
  )
}
