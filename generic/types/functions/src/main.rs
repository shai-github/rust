struct Point<T> {
    x: T,
    y: T,
}

// we want implementation to use generics
// do not need to use T, can instead use U to tie to implementation
impl<U> Point<U> {
    // take a reference to self an return a reference to the x field (a generic type)
    fn x(&self) -> &U {
        &self.x
    }
}

// can make an implementation block for Points of a specific type
// function y returns y value of the point
// whereas x is available to points of any type, y is only available to points of type f64
impl Point<f64> {
    fn y(&self) -> &f64 {
        &self.y
    }
}

// let's use a more complicated example
// x will be one type T and y will be another type U
struct PointComplicated<T, U> {
    x: T,
    y: U,
}

// we define an implementation block with generics T and U for our PointComplicated struct
// mixup has its own set of generics that are scoped to the mixup function
// the function takes the point as well as another point using V and W generics
// we use V and W here because we want the point being passed in to potentially have
// different types compared to the points being passed in with the self object
// return type says that x will come from return type being called on while y will come
// from the other point being passed in
impl<T, U> PointComplicated<T, U> {
    fn mixup<V, W>(self, other: PointComplicated<V, W>) -> PointComplicated<T, W> {
        PointComplicated {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // only x is available
    let p1 = Point { x: 5, y: 10 };
    p1.x();

    // x and y are available
    let p2= Point { x: 5.0, y: 10.0 };
    p2.x();
    p2.y();

    // what about a more complicated example?
    // first we declare a point with x of integer type and y of floating number type
    let p3 = PointComplicated { x: 5, y: 10.4 };

    // now we declare a point with x of string type and y of character type
    let p4 = PointComplicated { x: "Hello", y: 'c' };

    // output types will be an integer from x of p3 and a character from y of p4
    let p5 = p3.mixup(p4);

    println!("p5.x = {}, p5.y = {}", p5.x, p5.y);
}