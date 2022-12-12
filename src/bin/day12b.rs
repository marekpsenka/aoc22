use priority_queue::PriorityQueue;

fn main() {
    let mut map = std::io::stdin()
        .lines()
        .map(|maybe_line| {
            let line = maybe_line.expect("Successful read");
            line.trim()
                .chars()
                .into_iter()
                .map(|c| c as u32)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let height = map.len();
    let width = map[0].len();
    let mut dist = vec![vec![u32::MAX; width]; height];
    let mut visited = vec![vec![false; width]; height];

    let mut s_x = 0usize;
    let mut s_y = 0usize;
    let mut e_x = 0usize;
    let mut e_y = 0usize;

    let a_val = 'a' as u32;
    let s_val = 'S' as u32;
    let e_val = 'E' as u32;

    for y in 0..height {
        for x in 0..width {
            let val = map[y][x];
            if val == s_val {
                s_x = x;
                s_y = y;
                map[y][x] = 'a' as u32;
            }
            else if val == e_val {
                e_x = x;
                e_y = y;
                map[y][x] = 'z' as u32;
            }
        }
    }

    dist[e_y][e_x] = 0;
    let mut q = PriorityQueue::<(usize, usize), u32>::new();
    q.push((e_x, e_y), u32::MAX);

    while let Some(((x, y), _)) = q.pop() {
        if x < width - 1
            && !visited[y][x + 1]
            && map[y][x + 1] >= map[y][x] - 1
            && dist[y][x + 1] > dist[y][x]
        {
            dist[y][x + 1] = dist[y][x] + 1;
            q.push_increase((x + 1, y), u32::MAX - dist[y][x + 1]);
        }
        if x > 0
            && !visited[y][x - 1]
            && map[y][x - 1] >= map[y][x] - 1
            && dist[y][x - 1] > dist[y][x]
        {
            dist[y][x - 1] = dist[y][x] + 1;
            q.push_increase((x - 1, y), u32::MAX - dist[y][x - 1]);
        }
        if y > 0
            && !visited[y - 1][x]
            && map[y - 1][x] >= map[y][x] - 1
            && dist[y - 1][x] > dist[y][x]
        {
            dist[y - 1][x] = dist[y][x] + 1;
            q.push_increase((x, y - 1), u32::MAX - dist[y - 1][x]);
        }
        if y < height - 1
            && !visited[y + 1][x]
            && map[y + 1][x] >= map[y][x] - 1
            && dist[y + 1][x] > dist[y][x]
        {
            dist[y + 1][x] = dist[y][x] + 1;
            q.push_increase((x, y + 1), u32::MAX - dist[y + 1][x]);
        }

        visited[y][x] = true;

        if map[y][x] == a_val {
            s_x = x;
            s_y = y;
            break;
        }
    }

    println!("{}", dist[s_y][s_x])
}