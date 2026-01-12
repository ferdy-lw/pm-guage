#podman exec -ti zed-pm espflash flash --monitor --partition-table pm-gauge/partitions.csv --erase-parts otadata --bootloader pm-gauge/target/xtensa-esp32s3-espidf/debug/bootloader.bin pm-gauge/target/xtensa-esp32s3-espidf/debug/pm-gauge
podman exec -ti zed-pm espflash flash --monitor pm-gauge/target/xtensa-esp32s3-espidf/debug/pm-gauge
