use geo::{ coord, Coord };

pub struct Digit {
    coords: Vec<Coord>,
}

impl Digit {
    pub fn new(value: char) -> Self {
        let coords = match value {
            '0' => vec![coord! { x: 9.0, y: 21.0 }, coord! { x: 6.0, y: 20.0 }, coord! { x: 4.0, y: 17.0 }, coord! { x: 3.0, y: 12.0 }, coord! { x: 3.0, y: 9.0 }, coord! { x: 4.0, y: 4.0 }, coord! { x: 6.0, y: 1.0 }, coord! { x: 9.0, y: 0.0 }, coord! { x: 11.0, y: 0.0 }, coord! { x: 14.0, y: 1.0 }, coord! { x: 16.0, y: 4.0 }, coord! { x: 17.0, y: 9.0 }, coord! { x: 17.0, y: 12.0 }, coord! { x: 16.0, y: 17.0 }, coord! { x: 14.0, y: 20.0 }, coord! { x: 11.0, y: 21.0 }, coord! { x: 9.0, y: 21.0 }],
            '1' => vec![coord! { x: 6.0, y: 17.0 }, coord! { x: 8.0, y: 18.0 }, coord! { x: 11.0, y: 21.0 }, coord! { x: 11.0, y: 0.0 }],
            '2' => vec![coord! { x: 4.0, y: 16.0 }, coord! { x: 4.0, y: 17.0 }, coord! { x: 5.0, y: 19.0 }, coord! { x: 6.0, y: 20.0 }, coord! { x: 8.0, y: 21.0 }, coord! { x: 12.0, y: 21.0 }, coord! { x: 14.0, y: 20.0 }, coord! { x: 15.0, y: 19.0 }, coord! { x: 16.0, y: 17.0 }, coord! { x: 16.0, y: 15.0 }, coord! { x: 15.0, y: 13.0 }, coord! { x: 13.0, y: 10.0 }, coord! { x: 3.0, y: 0.0 }, coord! { x: 17.0, y: 0.0 }],
            '3' => vec![coord! { x: 5.0, y: 21.0 }, coord! { x: 16.0, y: 21.0 }, coord! { x: 10.0, y: 13.0 }, coord! { x: 13.0, y: 13.0 }, coord! { x: 15.0, y: 12.0 }, coord! { x: 16.0, y: 11.0 }, coord! { x: 17.0, y: 8.0 }, coord! { x: 17.0, y: 6.0 }, coord! { x: 16.0, y: 3.0 }, coord! { x: 14.0, y: 1.0 }, coord! { x: 11.0, y: 0.0 }, coord! { x: 8.0, y: 0.0 }, coord! { x: 5.0, y: 1.0 }, coord! { x: 4.0, y: 2.0 }, coord! { x: 3.0, y: 4.0 }],
            '4' => vec![coord! { x: 13.0, y: 21.0 }, coord! { x: 3.0, y: 7.0 }, coord! { x: 18.0, y: 7.0 }, coord! { x: 3.0, y: 7.0 }, coord! { x: 13.0, y: 21.0 }, coord! { x: 13.0, y: 0.0 }],
            '5' => vec![coord! { x: 15.0, y: 21.0 }, coord! { x: 5.0, y: 21.0 }, coord! { x: 4.0, y: 12.0 }, coord! { x: 5.0, y: 13.0 }, coord! { x: 8.0, y: 14.0 }, coord! { x: 11.0, y: 14.0 }, coord! { x: 14.0, y: 13.0 }, coord! { x: 16.0, y: 11.0 }, coord! { x: 17.0, y: 8.0 }, coord! { x: 17.0, y: 6.0 }, coord! { x: 16.0, y: 3.0 }, coord! { x: 14.0, y: 1.0 }, coord! { x: 11.0, y: 0.0 }, coord! { x: 8.0, y: 0.0 }, coord! { x: 5.0, y: 1.0 }, coord! { x: 4.0, y: 2.0 }, coord! { x: 3.0, y: 4.0 }],
            '6' => vec![coord! { x: 16.0, y: 18.0 }, coord! { x: 15.0, y: 20.0 }, coord! { x: 12.0, y: 21.0 }, coord! { x: 10.0, y: 21.0 }, coord! { x: 7.0, y: 20.0 }, coord! { x: 5.0, y: 17.0 }, coord! { x: 4.0, y: 12.0 }, coord! { x: 4.0, y: 7.0 }, coord! { x: 5.0, y: 3.0 }, coord! { x: 7.0, y: 1.0 }, coord! { x: 10.0, y: 0.0 }, coord! { x: 11.0, y: 0.0 }, coord! { x: 14.0, y: 1.0 }, coord! { x: 16.0, y: 3.0 }, coord! { x: 17.0, y: 6.0 }, coord! { x: 17.0, y: 7.0 }, coord! { x: 16.0, y: 10.0 }, coord! { x: 14.0, y: 12.0 }, coord! { x: 11.0, y: 13.0 }, coord! { x: 10.0, y: 13.0 }, coord! { x: 7.0, y: 12.0 }, coord! { x: 5.0, y: 10.0 }, coord! { x: 4.0, y: 7.0 }],
            '7' => vec![coord! { x: 17.0, y: 21.0 }, coord! { x: 7.0, y: 0.0 }, coord! { x: 17.0, y: 21.0 }, coord! { x: 3.0, y: 21.0 }, coord! { x: 17.0, y: 21.0 }],
            '8' => vec![coord! { x: 8.0, y: 21.0 }, coord! { x: 5.0, y: 20.0 }, coord! { x: 4.0, y: 18.0 }, coord! { x: 4.0, y: 16.0 }, coord! { x: 5.0, y: 14.0 }, coord! { x: 7.0, y: 13.0 }, coord! { x: 11.0, y: 12.0 }, coord! { x: 14.0, y: 11.0 }, coord! { x: 16.0, y: 9.0 }, coord! { x: 17.0, y: 7.0 }, coord! { x: 17.0, y: 4.0 }, coord! { x: 16.0, y: 2.0 }, coord! { x: 15.0, y: 1.0 }, coord! { x: 12.0, y: 0.0 }, coord! { x: 8.0, y: 0.0 }, coord! { x: 5.0, y: 1.0 }, coord! { x: 4.0, y: 2.0 }, coord! { x: 3.0, y: 4.0 }, coord! { x: 3.0, y: 7.0 }, coord! { x: 4.0, y: 9.0 }, coord! { x: 6.0, y: 11.0 }, coord! { x: 9.0, y: 12.0 }, coord! { x: 13.0, y: 13.0 }, coord! { x: 15.0, y: 14.0 }, coord! { x: 16.0, y: 16.0 }, coord! { x: 16.0, y: 18.0 }, coord! { x: 15.0, y: 20.0 }, coord! { x: 12.0, y: 21.0 }, coord! { x: 8.0, y: 21.0 }],
            '9' => vec![coord! { x: 16.0, y: 14.0 }, coord! { x: 15.0, y: 11.0 }, coord! { x: 13.0, y: 9.0 }, coord! { x: 10.0, y: 8.0 }, coord! { x: 9.0, y: 8.0 }, coord! { x: 6.0, y: 9.0 }, coord! { x: 4.0, y: 11.0 }, coord! { x: 3.0, y: 14.0 }, coord! { x: 3.0, y: 15.0 }, coord! { x: 4.0, y: 18.0 }, coord! { x: 6.0, y: 20.0 }, coord! { x: 9.0, y: 21.0 }, coord! { x: 10.0, y: 21.0 }, coord! { x: 13.0, y: 20.0 }, coord! { x: 15.0, y: 18.0 }, coord! { x: 16.0, y: 14.0 }, coord! { x: 16.0, y: 9.0 }, coord! { x: 15.0, y: 4.0 }, coord! { x: 13.0, y: 1.0 }, coord! { x: 10.0, y: 0.0 }, coord! { x: 8.0, y: 0.0 }, coord! { x: 5.0, y: 1.0 }, coord! { x: 4.0, y: 3.0 }],
            _ => vec![], // empty vector
        };
        
        Self { coords }
    }

    pub fn coords(&self) -> &Vec<Coord> {
        &self.coords
    }
}