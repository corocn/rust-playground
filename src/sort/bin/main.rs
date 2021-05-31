fn main() {
    let mut A = [5, 2, 4, 6, 1, 3];

    let len = A.len();
    for j in 1..len {
        let key = A[j];
        let mut i = j;
        while i > 0 && A[i] < A[i - 1] {
            A.swap(i , i - 1);
            i = i - 1;
        }
    }
    dbg!(A);
}
