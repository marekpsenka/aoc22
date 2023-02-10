use cgmath::Vector4;
use priority_queue::PriorityQueue;

struct Blueprint {
    ore_cost: Vector4<u32>,
    clay_cost: Vector4<u32>,
    obsidian_cost: Vector4<u32>,
    geode_cost: Vector4<u32>,
    max_cost: Vector4<u32>,
}

const TIME_LIMIT: u32 = 24;

#[derive(Hash, PartialEq, Eq)]
struct State {
    resources: Vector4<u32>,
    robots: Vector4<u32>,
    remaining: u32,
}

#[derive(PartialEq, Eq)]
struct CustomPriority {
    v: Vector4<u32>,
}

impl From<Vector4<u32>> for CustomPriority {
    fn from(v: Vector4<u32>) -> Self {
        CustomPriority { v }
    }
}

impl Ord for CustomPriority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.v.w.cmp(&other.v.w) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => match self.v.z.cmp(&other.v.z) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                std::cmp::Ordering::Equal => match self.v.y.cmp(&other.v.y) {
                    std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Equal => match self.v.x.cmp(&other.v.x) {
                        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
                    },
                },
            },
        }
    }
}

impl PartialOrd for CustomPriority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn can_afford_now(cost: &Vector4<u32>, resources: &Vector4<u32>) -> bool {
    cost.x <= resources.x && cost.y <= resources.y && cost.z <= resources.z && cost.w <= resources.w
}
fn can_afford_after(
    cost: &Vector4<u32>,
    resources: &Vector4<u32>,
    robots: &Vector4<u32>,
) -> Option<u32> {
    if can_afford_now(cost, resources) {
        return Some(0);
    }

    let x_wait;
    let y_wait;
    let z_wait;
    let w_wait;
    if cost.x > 0 && robots.x == 0 {
        return None;
    } else if cost.x == 0 {
        x_wait = 0.0;
    } else {
        x_wait = f32::ceil((cost.x as f32 - resources.x as f32) / robots.x as f32);
    }
    if cost.y > 0 && robots.y == 0 {
        return None;
    } else if cost.y == 0 {
        y_wait = 0.0;
    } else {
        y_wait = f32::ceil((cost.y as f32 - resources.y as f32) / robots.y as f32);
    }
    if cost.z > 0 && robots.z == 0 {
        return None;
    } else if cost.z == 0 {
        z_wait = 0.0;
    } else {
        z_wait = f32::ceil((cost.z as f32 - resources.z as f32) / robots.z as f32);
    }
    if cost.w > 0 && robots.w == 0 {
        return None;
    } else if cost.w == 0 {
        w_wait = 0.0;
    } else {
        w_wait = f32::ceil((cost.w as f32 - resources.w as f32) / robots.w as f32);
    }

    Some(0.0f32.max(x_wait.max(y_wait.max(z_wait.max(w_wait)))) as u32)
}

fn max_geodes(b: &Blueprint) -> u32 {
    let mut candidate = 0u32;
    let mut queue = PriorityQueue::<State, CustomPriority>::new();
    queue.push(
        State {
            resources: Vector4::new(0, 0, 0, 0),
            robots: Vector4::new(1, 0, 0, 0),
            remaining: TIME_LIMIT,
        },
        Vector4::new(TIME_LIMIT, 0, 0, 0).into(),
    );
    while let Some((s, p)) = queue.pop() {
        if s.remaining > 2
            && s.resources.w + s.robots.w * s.remaining + (s.remaining - 1) * (s.remaining - 2)
                < candidate
        {
            // If the current upper bound on geode making potential is smaller than current
            // candidate then continue
            continue;
        }

        if s.remaining == 0 {
            if s.resources.w > candidate {
                candidate = s.resources.w;
            }
            continue;
        }
        if let Some(delay) = can_afford_after(&b.geode_cost, &s.resources, &s.robots) {
            if s.remaining > delay {
                queue.push(
                    State {
                        resources: s.resources + (delay + 1) * s.robots - b.geode_cost,
                        robots: s.robots + Vector4::unit_w(),
                        remaining: s.remaining - delay - 1,
                    },
                    (p.v + Vector4::unit_w() * (s.remaining - delay - 1)).into(),
                );
            }
        }
        if s.robots.z < b.max_cost.z {
            if let Some(delay) = can_afford_after(&b.obsidian_cost, &s.resources, &s.robots) {
                if s.remaining > delay {
                    queue.push(
                        State {
                            resources: s.resources + (delay + 1) * s.robots - b.obsidian_cost,
                            robots: s.robots + Vector4::unit_z(),
                            remaining: s.remaining - delay - 1,
                        },
                        (p.v + Vector4::unit_z() * (s.remaining - delay - 1)).into(),
                    );
                }
            }
        }
        if s.robots.y < b.max_cost.y {
            if let Some(delay) = can_afford_after(&b.clay_cost, &s.resources, &s.robots){
                if s.remaining > delay {
                    queue.push(
                        State {
                            resources: s.resources + (delay + 1) * s.robots - b.clay_cost,
                            robots: s.robots + Vector4::unit_y(),
                            remaining: s.remaining - delay - 1,
                        },
                        (p.v + Vector4::unit_y() * (s.remaining - delay - 1)).into(),
                    );
                }
            }
        }
        if s.robots.x < b.max_cost.x {
            if let Some(delay) = can_afford_after(&b.ore_cost, &s.resources, &s.robots) {
                if s.remaining > delay {
                    queue.push(
                        State {
                            resources: s.resources + (delay + 1) * s.robots - b.ore_cost,
                            robots: s.robots + Vector4::unit_x(),
                            remaining: s.remaining - delay - 1,
                        },
                        (p.v + Vector4::unit_x() * (s.remaining - delay - 1)).into(),
                    );
                }
            }
        }
    }
    candidate
}

fn main() {
    let blueprints = std::io::stdin().lines().map(|maybe_line| {
        let line = maybe_line.expect("A line is read");
        let nums = line
            .split_ascii_whitespace()
            .filter_map(|part| part.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        Blueprint {
            ore_cost: Vector4::new(nums[0], 0, 0, 0),
            clay_cost: Vector4::new(nums[1], 0, 0, 0),
            obsidian_cost: Vector4::new(nums[2], nums[3], 0, 0),
            geode_cost: Vector4::new(nums[4], 0, nums[5], 0),
            max_cost: Vector4::new(
                [nums[0], nums[1], nums[2], nums[4]]
                    .into_iter()
                    .max()
                    .unwrap(),
                nums[3],
                nums[5],
                0,
            ),
        }
    });

    let quality = blueprints
        .zip(1..)
        .map(|(b, i)| {
            max_geodes(&b) * i
        })
        .sum::<u32>();

    println!("{quality}")
}
