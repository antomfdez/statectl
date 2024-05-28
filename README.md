# statectl

Rust program to hibernate, sleep or freeze a linux system in CLI using ```/sys/power/state``` file.
## Usage/Examples

```bash
sudo statectl [hibernate,sleep,freeze]
```


## Compile

```bash
  git clone https://github.com/antomfdez/statectl.git
  cd statectl
  sudo cargo build
  sudo cp target/debug/statectl /usr/bin
```
    
### Dowload binary
[![Download](https://cdn-icons-png.flaticon.com/128/2989/2989976.png)](https://github.com/antomfdez/statectl/releases/latest)

### How to install from releases:
```bash
tar -xf statectl.tar
cd statectl
sudo chmod +x installer.sh
sudo ./installer.sh
```
## Screenshots

![App Screenshot](https://github.com/antomfdez/statectl/state.png)


