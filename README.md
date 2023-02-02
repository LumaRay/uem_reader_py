# Python binding to Rust uem_reader crate

This is a Python binding for [MicroEM](https://microem.ru/) reader support [crate in Rust](https://crates.io/crates/uem-reader).

## Note on using libusb

On Linux you need to add write permissions to the device you want to use, e.g.:

```console
sudo chmod o+w /dev/bus/usb/002/008
```

To work with Windows you need to [install libusb driver](https://github.com/libusb/libusb/wiki/Windows#how-to-use-libusb-on-windows) for a reader.

## Usage

You can find module sources on [GitHub](https://github.com/LumaRay/uem_reader_py).

```python
import uem_reader_py
rdrs = uem_reader_py.find_usb_readers()
assert len(rdrs) > 0
rdrs[0].open()
print(rdrs[0].send([0x64]))  # Get Version
print(rdrs[0].send([0x22]))  # Get Serial
rdrs[0].send([0x05, 0x03])  # Beep 3 times
rdrs[0].close()
```

## Module development process

```console
sudo apt-get install curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
python3 -m venv .env
source ./.env/bin/activate
pip3 install maturin
maturin develop
pip3 install hypothesis
python3 ./python/tests/test.py
sudo apt install docker.io
sudo docker run --rm -v $(pwd):/io ghcr.io/pyo3/maturin build --release
# maturin build --release --bindings pyo3
sudo docker run --rm -v $(pwd):/io ghcr.io/pyo3/maturin publish --username LumaRay --password 
```

## License

This work is dual-licensed under MIT or Apache 2.0.
You can choose between one of them if you use this work.

`SPDX-License-Identifier: MIT OR Apache-2.0`
