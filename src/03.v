import os

// Store the bit count across a particular column of the input data.
struct BitFrequency {
mut:
	zeroes i64
	ones   i64
}

fn parse_input(lines []string) ?[]BitFrequency {
	// Populate the bit counts across the bins for every position.
	number_of_bins := lines[0].len
	mut bitfreqs := []BitFrequency{len: number_of_bins, cap: number_of_bins, init: BitFrequency{0, 0}}

	for w_idx, w in lines {
		for idx, elem in w {
			if elem == 48 {
				bitfreqs[idx].zeroes += 1
			} else if elem == 49 {
				bitfreqs[idx].ones += 1
			} else {
				panic('Non-binary character encountered: $elem, index: $idx, row: $w_idx, word: $w')
			}
		}
	}

	return bitfreqs
}

fn part_1(freqs []BitFrequency) i64 {
	mut gamma_rate := i64(0)
	mut epsilon := i64(0)

	mut pow := 1
	for item in freqs.reverse() {
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
	// Convert a binary string to i64.

	mut result := i64(0)
	mut pow := 1

	for item in b.reverse() {
		if item == 49 { // 0
			result += pow
		}
		pow *= 2
	}
	return result
}

fn (r Round) is_complete() bool {
	return r.words.len <= 1
}

fn round_of_ones(words []string) Round {
	return Round{words.map(fn (w string) string {
		return w.trim_prefix('1')
	}), '1'}
}

fn round_of_zeros(words []string) Round {
	return Round{words.map(fn (w string) string {
		return w.trim_prefix('0')
	}), '0'}
}

fn filter_words(words []string, max bool) Round {
	if words.len <= 1 {
		return Round{words, '-1'}
	}

	lead_zeros := words.filter(fn (w string) bool {
		return w[0] == 48 // 0
	})

	lead_ones := words.filter(fn (w string) bool {
		return w[0] == 49 // 1
	})

	// If:
	// #1s >= #0s and bit criteria is 'max'
	// OR
	// #0s > #1s and bit criteria is 'min'
	// then 1's get picked out from the start.
	// Else:
	// 0's get picked out from the start.

	if ((lead_ones.len >= lead_zeros.len) && max) || ((lead_ones.len < lead_zeros.len) && !max) {
		return round_of_ones(lead_ones)
	}
	return round_of_zeros(lead_zeros)
}

fn run_criterion(words []string, max bool) string {
	// Given some words and a bit criterion,
	// simulate the result of filtering the words.

	mut result := ''
	mut current_round := filter_words(words, max)

	result += current_round.winner

	for {
		if current_round.is_complete() {
			// Append any remaining chars.
			result += current_round.words[0]
			break
		}
		current_round = filter_words(current_round.words.clone(), max)
		result += current_round.winner
	}
	return result
}

fn part_2(lines []string) i64 {
	oxygen := run_criterion(lines.clone(), true)
	co2 := run_criterion(lines.clone(), false)

	oxygen_value := to_dec(oxygen)
	co2_value := to_dec(co2)

	return oxygen_value * co2_value
}

fn main() {
	lines := os.get_lines()
	bitfreqs := parse_input(lines) ?

	ans_1 := part_1(bitfreqs)
	ans_2 := part_2(lines)

	println('part_1: $ans_1 \npart_2: $ans_2')
}
