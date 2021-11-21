use std::io::Write;

pub fn potential_energy<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "fn potential_energy(distance: f32) -> f32 {{
  return 1.0 - (1.0 / (distance + 1.0));
}}"
    )
}

pub fn displacement_cost<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "fn displacement_cost(coords : vec2<i32>, displacement : vec2<i32>, permutation_vector : vec2<i32>) -> f32 {{
  let origin : vec2<i32> = coords + permutation_vector;
  let desired_position : vec2<i32> = origin + load_displacement_goal_vector(origin);
  let current_distance : f32 = length(vec2<f32>(desired_position - coords));
  let displaced_distance : f32 = length(vec2<f32>(desired_position - (coords + displacement)));
  return potential_energy(displaced_distance) - potential_energy(current_distance);
}}"
    )
}

pub fn swap_cost<W: Write>(mut writer: W) -> std::io::Result<()> {
    writeln!(
        writer,
        "fn swap_cost(
  coords1 : vec2<i32>,
  displacement : vec2<i32>,
  permutation_vector1 : vec2<i32>,
  permutation_vector2 : vec2<i32>
) -> f32 {{
  let coords2 : vec2<i32> = coords1 + displacement;
  return displacement_cost(coords1, displacement, permutation_vector1) + displacement_cost(coords2, -displacement, permutation_vector2);
}}"
    )
}
