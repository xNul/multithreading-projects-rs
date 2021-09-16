cargo build --release
sum=0

./target/release/assignment-1
line=$(head -n 1 primes.txt)
time=$(echo "$line" | head -n1 | awk '{print $1;}')
primes=$(echo "$line" | head -n1 | awk '{print $2;}')
psum=$(echo "$line" | head -n1 | awk '{print $3;}')

if [[ "$primes" != "5761455" ]]; then
	echo "Failed prime count check."
	read -p "Press any key to resume ..."
fi
if [[ "$psum" != "279209790387276" ]]; then
	echo "Failed prime sum check."
	read -p "Press any key to resume ..."
fi

for i in {1..100}; do
	./target/release/assignment-1
	line=$(head -n 1 primes.txt)
	time=$(echo "$line" | head -n1 | awk '{print $1;}')
	sum=$(echo "$sum + $time" | bc)
	echo "[$((100*$i/100))%] Iterating..."
done

echo "Average Runtime: $(echo "scale=4; $sum/100" | bc) seconds"
