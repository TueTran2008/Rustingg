#[derive(Debug)]

struct Point(i32, i32);

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 {
        p2
    } else {
        p1
    }
}

fn caculate_distance(p1: &Point, p2: &Point) -> i32 {
    (p2.0 - p1.0).abs() + (p2.0 - p1.0).abs()
}

fn nearest<'a, 'q>(points: &'a [Point], query: &'q Point) -> Option<&'a Point> {
    let mut nearest = None;
    for p in points {
        if let Some((_, nearest_dist)) = nearest {
            let dist = caculate_distance(p, query);
            if dist < nearest_dist {
                nearest = Some((p, dist));
            }
        } else {
            nearest = Some((p, caculate_distance(p, query)));
        }
    }
    nearest.map(|(x, _)| x)
}
#[derive(Debug)]
struct Hightlight<'doc>(&'doc str);

fn main() {
    let p1: Point = Point(1, 2);
    let p2: Point = Point(3, 4);
    let _p3 = left_most(&p1, &p2);

    let array_point = [Point(6, 7), Point(8, 9), Point(10, 11)];
    let query_point = Point(0, 0);
    let _nearst_point = nearest(&array_point, &query_point);

    let text = String::from("This is a hello from Ferado");
    let _h1 = Hightlight(&text[..10]);
    let _h2 = Hightlight(&text[10..]);
    //println!("{h1} {h2}")
}
