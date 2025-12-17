podman run --name zed-pm -d -p 2225:22 --security-opt label=disable --userns=keep-id --group-add keep-groups -v="/dev/:/dev:rslave,dev" --mount "type=devpts,destination=/dev/pts" --mount "type=bind,source=/run/udev,target=/run/udev,readonly" \
--mount "type=bind,source=/home/ferdy/.cargo/registry,target=/home/esp/.cargo/registry,consistency=cached" \
--mount "type=bind,source=/home/ferdy/proj/esp-idf-svc,target=/home/esp/esp-idf-svc,readonly" \
--mount "type=bind,source=/home/ferdy/proj/pm-guage,target=/home/esp/pm-guage,consistency=cached" zed-espup
