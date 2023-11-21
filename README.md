# MQTT Widget for KDE

## What does it do?

It is a widget which connects to an MQTT server and shows string data from a topic as text.

Note: it is a basic version and has limited functionality.

# Build instructions

## Build instructions

It is assumed that Rust, QT and KDE develop requirements are met.
The following commands build and install the project.

```
mkdir build
cd build
cmake .. -DCMAKE_INSTALL_PREFIX=/usr
cmake --build .
sudo cmake --install .
```

## Run contained environment

plasmoidviewer -a org.kde.plasma.mqttwidget

# Acknowledgements

Simon Brummer for his [plasmoid rust example](https://github.com/brummer-simon/rust_plasmoid_example)
