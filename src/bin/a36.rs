// Topic: Arrays & Slices
//
// Requirements:
// * Print pairs of numbers and their sums as they are streamed from a data source
// * If only one number is received, then print "Unpaired value: V",
//   where V is the value
// * If no numbers are received, print "Data stream complete"
//
// Notes:
// * A simulated data stream is already configured in the code
// * See the stdlib docs for the "chunks" method on "slice" for more info

fn data() -> &'static [u64] {
    &[5, 5, 4, 4, 3, 3, 1]
}

fn calculate_chunk(chunk: &[u64]) {
    match chunk {
        [first, last] => println!("Chunk: {chunk:?} and the sum is: {:?}", first + last),
        [first] => println!("Unpaired value: {first}"),
        [] => println!("Data stream complete"),
        [..] => unreachable!("Chunk size must be equal to 2"),
    }
}

fn main() {
    // `stream` is an iterator of Option<&[u64]>
    let mut stream = data().chunks(2);

    for chunk in stream {
        calculate_chunk(chunk)
    }
}
