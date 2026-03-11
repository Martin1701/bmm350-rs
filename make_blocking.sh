#!/bin/bash
sed \
  -e 's/async fn/fn/g' \
  -e 's/\.await\?$//' \
  -e 's/\.await//g' \
  -e 's/embedded_hal_async::delay::DelayNs/embedded_hal::delay::DelayNs/g' \
  -e 's/interface_async/interface/g' \
  src/device_async.rs > src/device.rs

echo "Done"
