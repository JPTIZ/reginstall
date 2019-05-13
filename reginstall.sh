#!/usr/bin/env bash

if [[ -z $(which python3) ]]
then
    echo "No Python3. Installing..."
else
    echo "Python3 found."
fi

shift

python3 -m reginstall
