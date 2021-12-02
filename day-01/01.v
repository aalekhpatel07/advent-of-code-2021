import os

fn get_input() ?[]int {
	lines := os.read_lines('../data/01.in') ?
	nums := lines.map(fn (w string) int {
		return w.int()
	})
	return nums
}

fn part_1(arr []int) int {
	// Calculate the pairwise consecutive difference.
	// If the latter > former, increment a counter.
	mut counter := int(0)

	for idx, item in arr {
		if idx == 0 {
			continue
		}
		if item - arr[idx - 1] > 0 {
			counter += 1
		}
	}

	return counter
}

fn sum(items ...int) int {
	mut result := 0
	for item in items {
		result += item
	}
	return result
}

fn part_2(arr []int) int {
	// Calculate the window-wise consecutive differences of size (3).
	// If the latter > former, increment a counter.

	mut counter := int(0)

	for idx, value in arr {
		if idx < 3 {
			continue
		}
		// If the value coming in is larger than
		// the one going out, then there'll be an increase.

		// Furthermore, all increases will be accounted for
		// this way.
		if value > arr[idx - 3] {
			counter += 1
		}
	}

	return counter
}

fn main() {
	nums := get_input() ?
	ans_1 := part_1(nums)
	ans_2 := part_2(nums)
	println('part_1: $ans_1 \npart_2: $ans_2')
}
