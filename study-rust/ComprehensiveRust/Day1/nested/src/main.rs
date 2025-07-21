/*
함수 선언 시 입력값으로 type 을 명시해줄 때
초기화 관련 문법을 이용해서 표현할 수 있다.
*/

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
	let mut result = [[0; 3]; 3];
  	for i in 0..3 {
        	for j in 0..3 {
            		result[j][i] = matrix[i][j];
        	}
    	}
    	result
}

fn main() {
    	let matrix = [
        	[101, 102, 103],
        	[201, 202, 203],
        	[301, 302, 303],
    	];

    	dbg!(matrix);
    	let transposed = transpose(matrix);
    	dbg!(transposed);
}
