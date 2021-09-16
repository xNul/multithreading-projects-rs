cargo build --release
sum=0
total_guests=0

for i in {1..10}; do
	guest_size=$(echo "$i * 10" | bc)
	total_guests=$(echo "$guest_size + $total_guests" | bc)
	SECONDS=0
	./target/release/assignment-2-problem-1 $guest_size
	sum=$(echo "$sum + $SECONDS" | bc)
	echo "[$((100*$i/10))%] Iterating..."
done

sum2=0

for i in {1..100}; do
	SECONDS=0
	./target/release/assignment-2-problem-1 50
	sum2=$(echo "$sum2 + $SECONDS" | bc)
	echo "[$((100*$i/100))%] Iterating..."
done

echo "Average Runtime Per Guest: $(echo "scale=4; $sum/$total_guests" | bc) seconds"
echo "Average Runtime for 50: $(echo "scale=4; $sum2/100" | bc) seconds"
