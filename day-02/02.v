import os

struct Ins {
	direction string
	quantity  i64
}

fn get_input() ?[]Ins {
	lines := os.read_lines('../data/02.in') ?
	nums := lines.map(fn (w string) Ins {
		instruction := w.split(' ')
		// No array destructuring, V?
		// Arghh.
		direction := instruction[0]
		quantity := instruction[1]

		return Ins{direction, quantity.i64()}
	})
	return nums
}

fn part_1(arr []Ins) i64 {
	// Interpret a given list of instructions
	// with rules:

	// forward(x) := { horizontal += x }
	// down(x) := { depth += x }
	// up(x) := { depth -= x }

	mut horizontal := i64(0)
	mut depth := i64(0)

	for item in arr {
		match item.direction {
			'forward' { horizontal += item.quantity }
			'down' { depth += item.quantity }
			'up' { depth -= item.quantity }
			else {}
		}
	}

	return horizontal * depth
}

fn part_2(arr []Ins) i64 {
	// Interpret a given list of instructions
	// with rules:

	// forward(x) := {horizontal += x; depth += aim * x;}
	// down(x) := {aim += x}
	// up(x) := {aim -= x}

	mut horizontal := i64(0)
	mut depth := i64(0)
	mut aim := i64(0)

	for item in arr {
		match item.direction {
			'forward' {
				horizontal += item.quantity
				depth += aim * item.quantity
			}
			'down' {
				aim += item.quantity
			}
			'up' {
				aim -= item.quantity
			}
			else {}
		}
	}

	return horizontal * depth
}

fn main() {
	instructions := get_input() ?

	ans_1 := part_1(instructions)
	ans_2 := part_2(instructions)

	println('part_1: $ans_1 \npart_2: $ans_2')
}
