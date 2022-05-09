cargo build
sshpass -p "+Masterclass1879" scp target/armv7-unknown-linux-gnueabihf/debug/hello pi@192.168.1.21:/home/pi/
sshpass -p "+Masterclass1879" ssh pi@192.168.1.21

echo "Build, push and run complete!"
