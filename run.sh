start_=${AOC_START:-1}
end_=${AOC_END:-2}


for i in $(seq $start_ $end_);
	do
		echo "Day: $i"
		day=$(printf "%02d" $i)
		cd ./day-$day
		./day-$day
		cd ..
	done;

