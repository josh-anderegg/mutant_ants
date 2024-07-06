mkdir -p target/benchlogs
timestamp=$(date +"%Y%m%d_%H%M%S")
outputfile="bench_$timestamp.txt"
cargo bench > "target/benchlogs/$outputfile"