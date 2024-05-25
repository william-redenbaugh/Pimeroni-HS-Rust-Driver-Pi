cargo build --target aarch64-unknown-linux-gnu
sshpass -p "mamaral2000" scp target/aarch64-unknown-linux-gnu/debug/hello wredenba@192.168.3.249:/home/wredenba/
sshpass -p "mamaral2000" ssh wredenba@192.168.3.249

echo "Build, push and run complete!"
