
#[allow(dead_code)]
pub fn find_area_and_perim(shape_path: &Vec<[i32; 2]>) -> (f64, f64) {
    let mut area = 0;
    let mut perim: f64 = 0.0;

    let mut ox = shape_path[0][0];
    let mut oy = shape_path[0][1];

    for coord in &shape_path[1..] {
        // println!("COORD {:?}", coord);
        let x = coord[0];
        let y = coord[1];

        area += (x*oy)-(y*ox);
        // perim += f64::sqrt(((x-ox).pow(2)+(y-oy).pow(2)) as f64);
        perim += ((x-ox).pow(2)+(y-oy).pow(2)) as f64;

        ox = x;
        oy = y;
    }

    return (area as f64/2.0, perim);


    // 129 are not part of the circuit
    // 140 are
    // 60 are not points in the circuit
}
