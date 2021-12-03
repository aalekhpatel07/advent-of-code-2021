import os

struct BitFrequency {
mut:
	zeroes i64
	ones   i64
}

fn parse_input(lines []string) ?[]BitFrequency {
	number_of_bins := lines[0].len
	mut bitfreqs := []BitFrequency{len: number_of_bins, cap: number_of_bins, init: BitFrequency{0, 0}}

	for w in lines {
		for idx, elem in w {
			if elem == 48 {
				bitfreqs[idx].zeroes += 1
			} else if elem == 49 {
				bitfreqs[idx].ones += 1
			} else {
				panic('Whoops')
			}
		}
	}

	return bitfreqs
}

fn part_1(arr []BitFrequency) i64 {
	mut gamma_rate := i64(0)
	mut epsilon := i64(0)

	mut pow := 1
	for item in arr.reverse() {
		if item.ones > item.zeroes {
			gamma_rate += pow
		} else if item.zeroes > item.ones {
			epsilon += pow
		}
		pow *= 2
	}

	return gamma_rate * epsilon
}

struct Round {
	words []string
mut:
	winner string
}

fn to_dec(b string) i64 {
	mut result := i64(0)
	mut pow := 1

	for item in b.reverse() {
		if item == 49 {
			result += pow
		}
		pow *= 2
	}
	return result
}

fn (r Round) is_complete() bool {
	return r.words.len <= 1
}

fn filter_words(words []string, max bool) Round {
	if words.len <= 1 {
		return Round{words, '2'}
	}

	lead_zeros := words.filter(fn (w string) bool {
		return w[0] == 48
	})

	lead_ones := words.filter(fn (w string) bool {
		return w[0] == 49
	})

	if lead_ones.len == lead_zeros.len {
		if max {
			return Round{lead_ones.map(fn (w string) string {
				return w.trim_prefix('1')
			}), '1'}
		} else {
			return Round{lead_zeros.map(fn (w string) string {
				return w.trim_prefix('0')
			}), '0'}
		}
	} else if lead_ones.len > lead_zeros.len {
		if max {
			return Round{lead_ones.map(fn (w string) string {
				return w.trim_prefix('1')
			}), '1'}
		} else {
			return Round{lead_zeros.map(fn (w string) string {
				return w.trim_prefix('0')
			}), '0'}
		}
	} else {
		if max {
			return Round{lead_zeros.map(fn (w string) string {
				return w.trim_prefix('0')
			}), '0'}
		} else {
			return Round{lead_ones.map(fn (w string) string {
				return w.trim_prefix('1')
			}), '1'}
		}
	}
}

fn part_2(lines []string) i64 {
	mut result_oxygen := ''
	mut result_co2 := ''

	mut oxygen := filter_words(lines.clone(), true)
	mut co2 := filter_words(lines.clone(), false)
	println(co2)

	result_oxygen += oxygen.winner
	result_co2 += co2.winner

	for {
		if oxygen.is_complete() {
			result_oxygen += oxygen.words[0]
			break
		}
		oxygen = filter_words(oxygen.words.clone(), true)
		result_oxygen += oxygen.winner
	}

	for {
		if co2.is_complete() {
			result_co2 += co2.words[0]
			break
		}
		co2 = filter_words(co2.words.clone(), false)
		result_co2 += co2.winner
		println(co2)
	}

	oxygen_value := to_dec(result_oxygen)
	co2_value := to_dec(result_co2)

	println('$oxygen_value, $co2_value')
	return oxygen_value * co2_value
}

fn main() {
	lines := os.get_lines()
	bitfreqs := parse_input(lines) ?

	ans_1 := part_1(bitfreqs)
	ans_2 := part_2(lines)

	println('part_1: $ans_1 \npart_2: $ans_2')
}
