// https://atcoder.jp/contests/ahc071/tasks/ahc071_a
use std::io::{self, BufWriter, Read, Write};
// https://atcoder.jp/contests/ahc071/submissions/79423781

fn spine_place(
    holes: Vec<(usize, usize)>, /* 穴のxy座標の集合 */
    width: usize /* ブリックの幅 */
    ) -> Vec<Vec<(usize, usize)>> /* 列番号をそのまま添字とした、各列が担当する穴のリスト */ {
    let mut hole_sorted: Vec<((usize, usize), bool)> = holes.into_iter().map(|(x, y)| ((x, y), false)).collect(); // boolの方は処理済みかどうかの値
    hole_sorted.sort();

    let mut spines: Vec<Vec<(usize, usize)>> = vec![Vec::new(); width];
    let mut prev_zone_end: Option<usize> = None; // 前のスパインのreach右端
    
    loop {
        // hole_sortedの未処理の中で最小のものをpとする
        let idx = hole_sorted.iter().position(|&(_, done)| !done).unwrap();
        let p = hole_sorted[idx].0;

        // スパイン列をs=p+4におく。はしに近ければs-4>=0, s+4<=w-1に収まるように調整
        let mut spine_column = (p.0 + 4).min(width - 5);

        // 前のスパインのreachに食い込むかどうか
        let squeezed = prev_zone_end
            .map_or(false, |end| spine_column.saturating_sub(4) <= end);

        let (lo, hi) = if squeezed {
            // 感覚が取れないので、このマス単独のスパインにする
            spine_column = p.0;
            (p.0, p.0)
        } else {
            let lo = spine_column.saturating_sub(4).max(prev_zone_end.map_or(0, |end| end + 1));
            let hi = (spine_column + 4).min(width - 1);
            (lo, hi)
        };

        // [s-4, s+4]に入っているhole_sortedの要素の...
        for i in &mut hole_sorted {
            if !i.1
            && i.0 .0 >= lo
            && i.0 .0 <= hi
            {
                i.1 = true;        // ブール値をtrueに変える -> 処理済の印
                spines[spine_column].push(i.0);  // 逆にスパイン列がどこにできるかのindexを配列に保存
            }
        }

        prev_zone_end = Some(hi);

        // すべての要素が処理済みかどうかをチェック
        if hole_sorted.iter().all(|&(_, done)| done) {
            break;
        }
    }

    spines
}

fn cheapest_length(reach: usize, costs: [usize;5]) -> (usize, usize) {
    // reachを満たせる幅の中で一番安いもの
    // costs[0]=c1, costs[1]=c3, costs[2]=c5, costs[3]=c7, costs[4]=c9 とする
    // 戻り値は (選んだ幅, その値段)
    let mut min_cost = usize::MAX;
    let mut min_width = 0;
    for (width, cost) in [(1, costs[0]), (3, costs[1]), (5, costs[2]), (7, costs[3]), (9, costs[4])] {
        if (width - 1) / 2 >= reach && cost < min_cost {
            min_cost = cost;
            min_width = width;
        }
    }
    (min_width, min_cost)
}

fn solve(spines: Vec<Vec<(usize, usize)>>, costs: [usize; 5]) -> Vec<(usize, usize, usize)> {
    let mut saved_coords: Vec<(usize, usize, usize)> = Vec::new();
    // 担当穴リストから、最大のy（そのスパインが到達すべき最大の高さ）を求める
    // filterで要素が一つもない場合を除く
    for (spine_x, spine) in spines.iter().enumerate().filter(|spine| !spine.1.is_empty()) {
        let max_y = spine
            .iter()
            .map(|&(_, y)| y)
            .max()
            .unwrap();
        
        // y=0からその最大値まで、1段ずつ処理する
        for y in 0..=max_y {
            // その段に属する穴を担当穴リストから絞り込む
            // 複数ある可能性を考慮
            let row_holes: Vec<(usize, usize)> = spine
                .iter()
                .filter(|(_, y2)| *y2 == y)
                .cloned()
                .collect();
            
            // スパイン列との最大距離を取得する(穴がなければ、最安レンガの長さを採用)
            let max_distance = row_holes
                .iter()
                .map(|&(x, _)| x.abs_diff(spine_x))
                .max().unwrap_or(0);
            let (adopted_length, _) = cheapest_length(max_distance, costs);
            
            // 選んだ幅から一番左の座標をsaved_coordsに保存
            let leftmost_x = spine_x - (adopted_length - 1)/2;
            saved_coords.push((leftmost_x, y, adopted_length));
        }
    }
    saved_coords
}

fn main() {
    let mut inp = String::new();
    io::stdin().read_to_string(&mut inp).unwrap();
    let mut input = inp.split_whitespace();

    let w: usize = input.next().unwrap().parse().unwrap();
    let _h: usize = input.next().unwrap().parse().unwrap();

    // Kは穴の空いた総数
    let k: usize = input.next().unwrap().parse().unwrap();

    // 値段表(c1は1x1の値段、c3は1x3の値段、c5は1x5の値段、c7は1x7の値段、c9は1x9の値段)
    let pricing: [usize;5] = [
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap().parse().unwrap(),
    ];

    // 穴の座標
    let mut holes = Vec::new();
    for _ in 0..k {
        holes.push((
            input.next().unwrap().parse().unwrap(),
            input.next().unwrap().parse().unwrap(),
        ));
    }

    let a: Vec<Vec<(usize, usize)>> = spine_place(holes.clone(), w);
    let answer: Vec<(usize, usize, usize)> = solve(a, pricing);
    

    let mut out = BufWriter::new(io::stdout());
    writeln!(out, "{}", answer.len()).unwrap();
    for (x, y, l) in answer {
        writeln!(out, "{} {} {}", x, y, l).unwrap();
    }
}

