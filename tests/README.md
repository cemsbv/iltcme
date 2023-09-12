# Run

Numpy is needed in the environment to run. You can choose to either create a
virtual environment or just install numpy globally.

## Global (Debian)

```sh
sudo apt install python3-numpy
```

## Virtual Environment

Create a Python virtual environment, install the dependencies in the virtual env
and point PYO3 to it:

```sh
python3 -m venv /tmp/iltcme
source /tmp/iltcme/bin/activate
pip install numpy
PYO3_PYTHON=/tmp/iltcme/bin/python cargo test
```
