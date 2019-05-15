#!/usr/bin/env bash

if [[ -z $(which python3) ]]
then
    echo "No Python3. Installing..."
    sudo pacman -S python
    sudo pacman -S python-pip
else
    echo "Python3 found."
fi

shift

pip3 install --user toml
python3 -m reginstall $*
